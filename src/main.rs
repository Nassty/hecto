mod document;
mod editor;
mod highlight; // NEW
mod row;
mod terminal;
use std::{env, io::Error};
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut e = {
        if let Some(fname) = args.get(1) {
            editor::Editor::open(fname)
        } else {
            editor::Editor::default()
        }
    };
    e.run()
}
