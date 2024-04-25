use crate::colors;
use colors::Colors;
pub const GAME_SPEED: u16 = 60;

// (%)   100% W == half screen
pub const PLAYGROUND_WITH: u16 = 60;
// (%)   100% H == full screen
pub const PLAYGROUND_HEIGHT: u16 = 100;

pub const MARGIN_LEFT: u16 = 0;
pub const MARGIN_TOP: u16 = 0;

pub const BORDER_COLOR_FG: Colors = Colors::Black;
pub const BORDER_COLOR_BG: Colors = Colors::Yellow;
pub const BORDER_CHAR: &str = "🮪";
pub const BLOCK_CHAR: &str = "█";
pub const PLAYGROUND_COLOR_BG: Colors = Colors::Black;
pub const PLAYGROUND_COLOR_FG: Colors = Colors::Black;
// border is = 1 :D idk how to make it dynamic
// pub const BORDER: u16 = 1;
