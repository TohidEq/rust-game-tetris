use crossterm::style::StyledContent;
use crossterm::style::Stylize;
//
#[derive(Debug, Copy, Clone)]
pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Purple,
    Cyan,
    White,
}

impl Colors {
    pub fn bg_color(color: &Colors, text: StyledContent<String>) -> StyledContent<String> {
        match color {
            Colors::Black => {
                return text.on_black();
            }
            Colors::Red => {
                return text.on_red();
            }
            Colors::Green => {
                return text.on_green();
            }
            Colors::Yellow => {
                return text.on_yellow();
            }
            Colors::Blue => {
                return text.on_blue();
            }
            Colors::Magenta => {
                return text.on_magenta();
            }
            Colors::Purple => {
                return text.on_dark_magenta();
            }
            Colors::Cyan => {
                return text.on_cyan();
            }
            Colors::White => {
                return text.on_white();
            }
        }
    }

    pub fn fg_color(color: &Colors, text: StyledContent<String>) -> StyledContent<String> {
        match color {
            Colors::Black => {
                return text.black();
            }
            Colors::Red => {
                return text.red();
            }
            Colors::Green => {
                return text.green();
            }
            Colors::Yellow => {
                return text.yellow();
            }
            Colors::Blue => {
                return text.blue();
            }
            Colors::Magenta => {
                return text.magenta();
            }
            Colors::Purple => {
                return text.dark_red();
            }
            Colors::Cyan => {
                return text.cyan();
            }
            Colors::White => {
                return text.white();
            }
        }
    }
}
