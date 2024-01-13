use crate::{
    document::Document,
    row::Row,
    terminal::{Size, Terminal},
};
use derivative::Derivative;
use std::io::Error;
use termion::event::Key;

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
        let mut s = Self::default();
        s.document = Document::open(fname);
        s
    }
    fn process_keypress(&mut self) -> Result<(), Error> {
        let key = Terminal::read_key()?;
        match key {
            Key::Up | Key::Down | Key::Left | Key::Right => {
                self.move_cursor(key);
            }
            Key::Esc => {
                println!("AAAAA");
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
        let Size { width, height: _ } = self.terminal.size;
        let height = self.document.len();
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height as usize {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width as usize {
                    x = x.saturating_add(1)
                }
            }
            _ => unreachable!(),
        }
        self.cursor_position = Position { x, y }
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if !self.should_quit {
            self.draw_rows()?;
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size.width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size.width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{row}\r");
    }
    fn draw_rows(&self) -> Result<(), Error> {
        let height = self.terminal.size.height as usize;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
        Ok(())
    }
    pub fn run(&mut self) -> Result<(), Error> {
        while !self.should_quit {
            let _refresh = self.refresh_screen()?;
            let _key = self.process_keypress()?;
        }
        Ok(())
    }
}
