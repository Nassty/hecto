use crate::{
    document::Document,
    row::Row,
    terminal::{Size, Terminal},
};
use derivative::Derivative;
use std::{fmt::Display, io::Error};
use termion::{color, event::Key};

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Derivative)]
#[derivative(Default)]
pub struct Position {
    #[derivative(Default(value = "0"))]
    pub x: usize,
    #[derivative(Default(value = "0"))]
    pub y: usize,
}

#[derive(PartialEq)]
enum Mode {
    Insert,
    Normal,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Mode::Insert => "INSERT",
            Mode::Normal => "NORMAL",
        })?;
        Ok(())
    }
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct Editor {
    #[derivative(Default(value = "false"))]
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    #[derivative(Default(value = "Mode::Normal"))]
    mode: Mode,
}

impl Editor {
    pub fn open(fname: &str) -> Self {
        Self {
            document: Document::open(fname),
            ..Default::default()
        }
    }
    fn process_keypress_normal(&mut self) -> Result<(), Error> {
        let key = Terminal::read_key()?;
        match key {
            Key::Char('i') => self.switch_mode(Mode::Insert),
            Key::Char('h') => self.move_cursor(Key::Left),
            Key::Char('j') => self.move_cursor(Key::Down),
            Key::Char('k') => self.move_cursor(Key::Up),
            Key::Char('l') => self.move_cursor(Key::Right),
            Key::Ctrl('q') => {
                self.should_quit = true;
                Terminal::clear_screen();
                Terminal::cursor_position(&Position { x: 0, y: 0 });
            }
            _ => {}
        }
        self.scroll();
        Ok(())
    }
    fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
    fn process_keypress_insert(&mut self) -> Result<(), Error> {
        let key = Terminal::read_key()?;
        match key {
            Key::Up | Key::Down | Key::Left | Key::Right => {
                self.move_cursor(key);
            }
            Key::Char('\n') => {
                self.document.insert_nl(&self.cursor_position);
                let Position { x, y: _ } = self.cursor_position;
                Terminal::cursor_position(&Position {
                    x: x.saturating_sub(1),
                    y: 0,
                });
            }
            Key::Ctrl('q') => {
                self.should_quit = true;
                Terminal::clear_screen();
                Terminal::cursor_position(&Position { x: 0, y: 0 });
            }
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            }
            Key::Backspace => {
                let Position { x, y: _ } = self.cursor_position;
                self.move_cursor(Key::Left);
                self.document.delete(&self.cursor_position);
                if x == 0 {
                    self.move_cursor(Key::Up);
                }
            }

            Key::Esc => {
                self.switch_mode(Mode::Normal);
            }
            _ => {}
        }
        self.scroll();
        Ok(())
    }
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let Size { width, height } = self.terminal.size;
        let offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height as usize) {
            offset.y = y.saturating_sub(height as usize).saturating_add(10);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width as usize) {
            offset.x = x.saturating_sub(width as usize).saturating_add(1);
        }
    }
    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y, self.mode == Mode::Normal) {
            self.document.row_len(y)
        } else {
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            _ => unreachable!(),
        }
        width = if let Some(row) = self.document.row(y, self.mode == Mode::Normal) {
            self.document.row_len(y)
        } else {
            0
        };
        if x > width {
            x = width;
        }
        self.cursor_position = Position { x, y }
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position_no_offset(&Position { x: 0, y: 0 });
        if !self.should_quit {
            self.draw_rows();
            self.draw_status_bar();
            //self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {VERSION}");
        let width = self.terminal.size.width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        println!("{welcome_message}\r");
    }
    fn draw_row(&self, lineno: usize) {
        let width = self.terminal.size.width.saturating_sub(5) as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = self.document.render(lineno, start, end);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        print!("{lineno:<4}");
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
        println!(" {row}\r");
    }
    fn draw_status_bar(&self) {
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        let f = if let Some(fname) = &self.document.filename {
            fname.clone()
        } else {
            String::from("[No Name]")
        };

        let status = format!(
            "[{}] Editing: {f} {}/{}",
            self.mode,
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let spaces = " ".repeat(self.terminal.size.width as usize - status.len());
        println!("{status}{spaces}\r");
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }
    //fn draw_message_bar(&self) {
    //    Terminal::clear_current_line();
    //}
    fn draw_rows(&self) {
        let height = self.terminal.size.height as usize;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            let lineno = terminal_row + self.offset.y;
            if let Some(row) = self.document.row(lineno, self.mode == Mode::Normal) {
                self.draw_row(lineno);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
    pub fn run(&mut self) -> Result<(), Error> {
        while !self.should_quit {
            self.refresh_screen()?;
            match self.mode {
                Mode::Normal => self.process_keypress_normal()?,
                Mode::Insert => self.process_keypress_insert()?,
            }
        }
        Ok(())
    }
}
