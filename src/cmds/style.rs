use args::{Argument, Area, Style};
use cmds::EscCode;

/// Set a style of all content written from now on.
pub struct SetTextStyle(pub Style);

impl EscCode for SetTextStyle {
    fn OPCODE(&self) -> u16 { 0x30 }
    fn args(&self) -> Vec<String> {
        encode_args![self.0]
    }
}

/// Unset all styles that have been sent for content to be written.
pub struct DefaultTextStyle;

impl EscCode for DefaultTextStyle {
    fn OPCODE(&self) -> u16 { 0x30 }
}

/// Set the style of the cursor marker in the terminal.
pub struct SetCursorStyle(pub Style);

impl EscCode for SetCursorStyle {
    fn OPCODE(&self) -> u16 { 0x31 }
    fn args(&self) -> Vec<String> {
        encode_args![self.0]
    }
}

/// Unset all styles on the cursor and use the default cursor style.
pub struct DefaultCursorStyle;

impl EscCode for DefaultCursorStyle {
    fn OPCODE(&self) -> u16 { 0x31 }
}

/// Set a style in a given area of the grid.
pub struct SetStyleInArea(pub Area, pub Style);

impl EscCode for SetStyleInArea {
    fn OPCODE(&self) -> u16 { 0x32 }
    fn args(&self) -> Vec<String> {
        encode_args![self.0, self.1]
    }
}

/// Unset all styles in a given area of the grid.
pub struct DefaultStyleInArea(pub Area);

impl EscCode for DefaultStyleInArea {
    fn OPCODE(&self) -> u16 { 0x32 }
    fn args(&self) -> Vec<String> {
        encode_args![self.0]
    }
}
