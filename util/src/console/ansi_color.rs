
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Grey,
}

pub enum AnsiLayer {
    Foreground,
    Background,
}


pub fn get(color: AnsiColor) -> String { get_with_layer(color, AnsiLayer::Foreground) }

pub fn get_with_layer(color : AnsiColor, layer: AnsiLayer) -> String 
{
    let code = match color {
        AnsiColor::Black   => 30,
        AnsiColor::Red     => 31,
        AnsiColor::Green   => 32,
        AnsiColor::Yellow  => 33,
        AnsiColor::Blue    => 34,
        AnsiColor::Magenta => 35,
        AnsiColor::Cyan    => 36,
        AnsiColor::White   => 37,
        AnsiColor::Grey    => 90,
    };

    let layer_code = match layer 
    {
        AnsiLayer::Foreground => 0,
        AnsiLayer::Background => 10,
    };

    format!("\x1b[{}m", code + layer_code)
}

pub const BLACK_FOREGROUND  : &str = "\x1b[30m";
pub const RED_FOREGROUND    : &str = "\x1b[31m";
pub const GREEN_FOREGROUND  : &str = "\x1b[32m";
pub const YELLOW_FOREGROUND : &str = "\x1b[33m";
pub const BLUE_FOREGROUND   : &str = "\x1b[34m";
pub const MAGENTA_FOREGROUND: &str = "\x1b[35m";
pub const CYAN_FOREGROUND   : &str = "\x1b[36m";
pub const WHITE_FOREGROUND  : &str = "\x1b[37m";
pub const GREY_FOREGROUND   : &str = "\x1b[90m";

pub const BLACK_BACKGROUND  : &str = "\x1b[40m";
pub const RED_BACKGROUND    : &str = "\x1b[41m";
pub const GREEN_BACKGROUND  : &str = "\x1b[42m";
pub const YELLOW_BACKGROUND : &str = "\x1b[43m";
pub const BLUE_BACKGROUND   : &str = "\x1b[44m";
pub const MAGENTA_BACKGROUND: &str = "\x1b[45m";
pub const CYAN_BACKGROUND   : &str = "\x1b[46m";
pub const WHITE_BACKGROUND  : &str = "\x1b[47m";
pub const GREY_BACKGROUND   : &str = "\x1b[100m";

/* 
pub const COLOR_TITLE: &str = "\x1b[35m";

pub const COLOR_ERROR:   &str           = "\x1b[31m";
pub const COLOR_HIGHLIGHT_ERROR:   &str = "\x1b[47m\x1b[41m";

pub const COLOR_SUCCESS: &str           = "\x1b[32m";
pub const COLOR_HIGHLIGHT_SUCCESS: &str = "\x1b[47m\x1b[42m";

pub const COLOR_WARNING: &str           = "\x1b[33m";
pub const COLOR_HIGHLIGHT_WARNING: &str = "\x1b[47m\x1b[43m";

pub const COLOR_INFO: &str           = "\x1b[36m";
pub const COLOR_HIGHLIGHT_INFO: &str = "\x1b[47m\x1b[46m";
*/

pub const COLOR_BLACK_ON_WHITE : &str   = "\x1b[30m\x1b[47m";
pub const COLOR_RESET: &str = "\x1b[37m\x1b[40m";