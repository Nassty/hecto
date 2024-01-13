use crate::{
    document::Document,
    row::Row,
    terminal::{Size, Terminal},
};
use derivative::Derivative;
use std::io::Error;
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

#[derive(Derivative)]
#[derivative(Default)]
pub struct Editor {
    #[derivative(Default(value = "false"))]
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn open(fname: &str) -> Self {
        Self {
            document: Document::open(fname),
            ..Default::default()
        }
    }
    fn process_keypress(&mut self) -> Result<(), Error> {
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
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            }
            Key::Backspace => {
                self.move_cursor(Key::Left);
                self.document.delete(&self.cursor_position);
            }

            Key::Esc => {
                self.should_quit = true;
                Terminal::clear_screen();
                Terminal::cursor_position(&Position { x: 0, y: 0 });
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
            offset.y = y.saturating_sub(height as usize).saturating_add(1);
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
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
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
        width = if let Some(row) = self.document.row(y) {
            row.len()
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
        Terminal::cursor_position(&Position { x: 0, y: 0 });
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
    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size.width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{row}\r");
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
            "Editing: {f} {}/{}",
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
            if let Some(row) = self.document.row(terminal_row + self.offset.y) {
                self.draw_row(row);
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
            self.process_keypress()?;
        }
        Ok(())
    }
}
