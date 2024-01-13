use derivative::Derivative;
use termion::color;

#[derive(Derivative)]
#[derivative(PartialEq, Debug)]
pub enum Type {
    None,
    Number,
    String,
}

impl Type {
    pub fn to_color(&self) -> impl color::Color {
        match self {
            Self::Number => color::Rgb(220, 60, 60),
            Self::String => color::Rgb(60, 220, 60),
            Self::None => color::Rgb(255, 255, 255),
        }
    }
}
