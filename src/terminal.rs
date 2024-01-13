use crate::editor::Position;
use std::io::{self, stdout, Error, Stdout, Write};
use termion::{
    color,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use derivative::Derivative;
#[derive(Derivative)]
pub struct Terminal {
    _stdout: RawTerminal<Stdout>,
    pub size: Size,
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

impl From<(u16, u16)> for Size {
    fn from(p: (u16, u16)) -> Self {
        Self {
            height: p.1.saturating_sub(2),
            width: p.0,
        }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        let size = termion::terminal_size()
            .expect("Get the terminal size")
            .into();
        let _stdout = stdout().into_raw_mode().expect("Get the terminal raw mode");
        Self { _stdout, size }
    }
}

impl Terminal {
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
    pub fn flush() -> Result<(), Error> {
        io::stdout().flush()
    }
    pub fn cursor_position(pos: &Position) {
        let x = pos.x.saturating_add(1) as u16;
        let y = pos.y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }
    pub fn set_bg_color(color: color::Rgb) {
        #[cfg(target_os = "linux")]
        print!("{}", color::Bg(color));
        #[cfg(target_os = "macos")]
        print!("{}", termion::style::Invert);
    }
    pub fn reset_bg_color() {
        #[cfg(target_os = "linux")]
        print!("{}", color::Bg(color::Reset));
        #[cfg(target_os = "macos")]
        print!("{}", termion::style::Reset);
    }
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
