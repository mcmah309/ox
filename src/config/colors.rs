/// For dealing with colours in the configuration file
use crate::error::{OxError, Result};
use crossterm::style::Color as CColor;
use mlua::prelude::*;

use super::issue_warning;

#[derive(Debug)]
pub struct Colors {
    pub editor_bg: Color,
    pub editor_fg: Color,

    pub status_bg: Color,
    pub status_fg: Color,

    pub highlight: Color,

    pub line_number_fg: Color,
    pub line_number_bg: Color,

    pub tab_active_fg: Color,
    pub tab_active_bg: Color,
    pub tab_inactive_fg: Color,
    pub tab_inactive_bg: Color,

    pub info_bg: Color,
    pub info_fg: Color,
    pub warning_bg: Color,
    pub warning_fg: Color,
    pub error_bg: Color,
    pub error_fg: Color,

    pub selection_fg: Color,
    pub selection_bg: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            editor_bg: Color::Black,
            editor_fg: Color::Black,

            status_bg: Color::Black,
            status_fg: Color::Black,

            highlight: Color::Black,

            line_number_fg: Color::Black,
            line_number_bg: Color::Black,

            tab_active_fg: Color::Black,
            tab_active_bg: Color::Black,
            tab_inactive_fg: Color::Black,
            tab_inactive_bg: Color::Black,

            info_bg: Color::Black,
            info_fg: Color::Black,
            warning_bg: Color::Black,
            warning_fg: Color::Black,
            error_bg: Color::Black,
            error_fg: Color::Black,

            selection_fg: Color::White,
            selection_bg: Color::Blue,
        }
    }
}

impl LuaUserData for Colors {
    #[allow(clippy::too_many_lines)]
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("editor_bg", |env, this| Ok(this.editor_bg.to_lua(env)));
        fields.add_field_method_get("editor_fg", |env, this| Ok(this.editor_fg.to_lua(env)));
        fields.add_field_method_get("status_bg", |env, this| Ok(this.status_bg.to_lua(env)));
        fields.add_field_method_get("status_fg", |env, this| Ok(this.status_fg.to_lua(env)));
        fields.add_field_method_get("highlight", |env, this| Ok(this.highlight.to_lua(env)));
        fields.add_field_method_get("line_number_bg", |env, this| {
            Ok(this.line_number_bg.to_lua(env))
        });
        fields.add_field_method_get("line_number_fg", |env, this| {
            Ok(this.line_number_fg.to_lua(env))
        });
        fields.add_field_method_get("tab_active_fg", |env, this| {
            Ok(this.tab_active_fg.to_lua(env))
        });
        fields.add_field_method_get("tab_active_bg", |env, this| {
            Ok(this.tab_active_bg.to_lua(env))
        });
        fields.add_field_method_get("tab_inactive_fg", |env, this| {
            Ok(this.tab_inactive_fg.to_lua(env))
        });
        fields.add_field_method_get("tab_inactive_bg", |env, this| {
            Ok(this.tab_inactive_bg.to_lua(env))
        });
        fields.add_field_method_get("error_bg", |env, this| Ok(this.error_bg.to_lua(env)));
        fields.add_field_method_get("error_fg", |env, this| Ok(this.error_fg.to_lua(env)));
        fields.add_field_method_get("warning_bg", |env, this| Ok(this.warning_bg.to_lua(env)));
        fields.add_field_method_get("warning_fg", |env, this| Ok(this.warning_fg.to_lua(env)));
        fields.add_field_method_get("info_bg", |env, this| Ok(this.info_bg.to_lua(env)));
        fields.add_field_method_get("info_fg", |env, this| Ok(this.info_fg.to_lua(env)));
        fields.add_field_method_get("selection_fg", |env, this| {
            Ok(this.selection_fg.to_lua(env))
        });
        fields.add_field_method_get("selection_bg", |env, this| {
            Ok(this.selection_bg.to_lua(env))
        });
        fields.add_field_method_set("editor_bg", |_, this, value| {
            this.editor_bg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("editor_fg", |_, this, value| {
            this.editor_fg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("status_bg", |_, this, value| {
            this.status_bg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("status_fg", |_, this, value| {
            this.status_fg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("highlight", |_, this, value| {
            this.highlight = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("line_number_bg", |_, this, value| {
            this.line_number_bg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("line_number_fg", |_, this, value| {
            this.line_number_fg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("tab_active_fg", |_, this, value| {
            this.tab_active_fg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("tab_active_bg", |_, this, value| {
            this.tab_active_bg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("tab_inactive_fg", |_, this, value| {
            this.tab_inactive_fg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("tab_inactive_bg", |_, this, value| {
            this.tab_inactive_bg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("error_bg", |_, this, value| {
            this.error_bg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("error_fg", |_, this, value| {
            this.error_fg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("warning_bg", |_, this, value| {
            this.warning_bg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("warning_fg", |_, this, value| {
            this.warning_fg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("info_bg", |_, this, value| {
            this.info_bg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("info_fg", |_, this, value| {
            this.info_fg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("selection_fg", |_, this, value| {
            this.selection_fg = Color::from_lua(value);
            Ok(())
        });
        fields.add_field_method_set("selection_bg", |_, this, value| {
            this.selection_bg = Color::from_lua(value);
            Ok(())
        });
    }
}

#[derive(Debug)]
pub enum Color {
    Rgb(u8, u8, u8),
    Hex(String),
    Ansi(u8),
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
    Transparent,
}

impl Color {
    /// Converts from a lua value into a colour
    pub fn from_lua(value: LuaValue<'_>) -> Self {
        match value {
            LuaValue::String(string) => match string.to_str().unwrap_or("transparent") {
                "black" => Self::Black,
                "darkgrey" => Self::DarkGrey,
                "red" => Self::Red,
                "darkred" => Self::DarkRed,
                "green" => Self::Green,
                "darkgreen" => Self::DarkGreen,
                "yellow" => Self::Yellow,
                "darkyellow" => Self::DarkYellow,
                "blue" => Self::Blue,
                "darkblue" => Self::DarkBlue,
                "magenta" => Self::Magenta,
                "darkmagenta" => Self::DarkMagenta,
                "cyan" => Self::Cyan,
                "darkcyan" => Self::DarkCyan,
                "white" => Self::White,
                "grey" => Self::Grey,
                "transparent" => Self::Transparent,
                hex => Self::Hex(hex.to_string()),
            },
            LuaValue::Table(table) => {
                if table.len().unwrap_or(3) != 3 {
                    issue_warning("Invalid RGB sequence used in configuration file (must be a list of 3 numbers)");
                    return Self::Transparent;
                }
                let mut tri: Vec<u8> = vec![];
                for _ in 0..3 {
                    if let Ok(val) = table.pop() {
                        tri.insert(0, val);
                    } else {
                        issue_warning("Invalid RGB sequence provided - please check your numerical values are between 0 and 255");
                        tri.insert(0, 255);
                    }
                }
                Self::Rgb(tri[0], tri[1], tri[2])
            }
            LuaValue::Integer(number) => {
                if (0..=255).contains(&number) {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    Self::Ansi(number as u8)
                } else {
                    issue_warning(
                        "ANSI colour codes must be between 0-255 inclusively, defaulting to black",
                    );
                    Self::Ansi(0)
                }
            }
            _ => {
                issue_warning("Invalid data type used for colour in configuration file");
                Self::Transparent
            }
        }
    }

    /// Converts from a colour into a lua value
    pub fn to_lua<'a>(&self, env: &'a Lua) -> LuaValue<'a> {
        let msg = "Failed to create lua string";
        match self {
            Color::Hex(hex) => {
                let string = env.create_string(hex).expect(msg);
                LuaValue::String(string)
            }
            Color::Rgb(r, g, b) => {
                // Create lua table
                let table = env.create_table().expect("Failed to create lua table");
                let _ = table.push(*r as isize);
                let _ = table.push(*g as isize);
                let _ = table.push(*b as isize);
                LuaValue::Table(table)
            }
            Color::Ansi(code) => LuaValue::Integer(i64::from(*code)),
            Color::Black => LuaValue::String(env.create_string("black").expect(msg)),
            Color::DarkGrey => LuaValue::String(env.create_string("darkgrey").expect(msg)),
            Color::Red => LuaValue::String(env.create_string("red").expect(msg)),
            Color::DarkRed => LuaValue::String(env.create_string("darkred").expect(msg)),
            Color::Green => LuaValue::String(env.create_string("green").expect(msg)),
            Color::DarkGreen => LuaValue::String(env.create_string("darkgreen").expect(msg)),
            Color::Yellow => LuaValue::String(env.create_string("yellow").expect(msg)),
            Color::DarkYellow => LuaValue::String(env.create_string("darkyellow").expect(msg)),
            Color::Blue => LuaValue::String(env.create_string("blue").expect(msg)),
            Color::DarkBlue => LuaValue::String(env.create_string("darkblue").expect(msg)),
            Color::Magenta => LuaValue::String(env.create_string("magenta").expect(msg)),
            Color::DarkMagenta => LuaValue::String(env.create_string("darkmagenta").expect(msg)),
            Color::Cyan => LuaValue::String(env.create_string("cyan").expect(msg)),
            Color::DarkCyan => LuaValue::String(env.create_string("darkcyan").expect(msg)),
            Color::White => LuaValue::String(env.create_string("white").expect(msg)),
            Color::Grey => LuaValue::String(env.create_string("grey").expect(msg)),
            Color::Transparent => LuaValue::String(env.create_string("transparent").expect(msg)),
        }
    }

    /// Returns a colour as a crossterm colour, ready to turn into ANSI codes
    pub fn to_color(&self) -> Result<CColor> {
        Ok(match self {
            Color::Hex(hex) => {
                let (r, g, b) = Self::hex_to_rgb(hex)?;
                CColor::Rgb { r, g, b }
            }
            Color::Rgb(r, g, b) => CColor::Rgb {
                r: *r,
                g: *g,
                b: *b,
            },
            Color::Ansi(code) => CColor::AnsiValue(*code),
            Color::Black => CColor::Black,
            Color::DarkGrey => CColor::DarkGrey,
            Color::Red => CColor::Red,
            Color::DarkRed => CColor::DarkRed,
            Color::Green => CColor::Green,
            Color::DarkGreen => CColor::DarkGreen,
            Color::Yellow => CColor::Yellow,
            Color::DarkYellow => CColor::DarkYellow,
            Color::Blue => CColor::Blue,
            Color::DarkBlue => CColor::DarkBlue,
            Color::Magenta => CColor::Magenta,
            Color::DarkMagenta => CColor::DarkMagenta,
            Color::Cyan => CColor::Cyan,
            Color::DarkCyan => CColor::DarkCyan,
            Color::White => CColor::White,
            Color::Grey => CColor::Grey,
            Color::Transparent => CColor::Reset,
        })
    }

    /// Turn a hex value into an rgb value
    fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8)> {
        // Remove the leading '#' if present
        let hex = hex.trim_start_matches('#');

        // Ensure the hex code is exactly 6 characters long
        if hex.len() != 6 {
            let msg = "Invalid hex code used in configuration file - ensure they are of length 6";
            return Err(OxError::Config(msg.to_string()));
        }

        // Parse the hex string into the RGB components
        let mut tri: Vec<u8> = vec![];
        for i in 0..3 {
            let section = &hex[(i * 2)..(i * 2 + 2)];
            if let Ok(val) = u8::from_str_radix(section, 16) {
                tri.push(val);
            } else {
                let msg = "Invalid hex code used in configuration file - ensure all digits are between 0 and F";
                return Err(OxError::Config(msg.to_string()));
            }
        }
        Ok((tri[0], tri[1], tri[2]))
    }
}
