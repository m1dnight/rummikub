use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Blue,
    Red,
    Orange,
    Black,
}

impl Color {
    pub fn from_str(s: &str) -> Option<Color> {
        match s.to_lowercase().as_str() {
            "blue" => Some(Color::Blue),
            "red" => Some(Color::Red),
            "orange" => Some(Color::Orange), // Accept both orange and yellow
            "black" => Some(Color::Black),
            _ => None,
        }
    }


    fn bg_ansi_code(&self) -> &'static str {
        match self {
            Color::Blue => "\x1b[44m",   // Blue background
            Color::Red => "\x1b[41m",    // Red background
            Color::Orange => "\x1b[43m", // Yellow/Orange background
            Color::Black => "\x1b[40m",  // Black background
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Tile {
    pub color: Color,
    pub value: u8,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ansi_code: &str = self.color.bg_ansi_code();
        let value: u8 = self.value;
        write!(f, "{}{:2}\x1b[0m", ansi_code, value)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Group {
    pub value: u8,
    pub colors: Vec<Color>,
}

impl Group {
    pub fn valid(&self) -> bool {
        let mut seen = HashSet::new();
        self.colors.iter().all(|x| seen.insert(x))
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<8}", "Group: ")?;
        if self.colors.is_empty() {
            write!(f, "empty")
        } else {
            Ok(for (index, color) in self.colors.iter().enumerate() {
                let tile = Tile {
                    color: *color,
                    value: self.value,
                };

                if index > 0 {
                    // write!(f, " → ")?;
                    write!(f, " ")?;
                }
                write!(f, "{}", tile.to_string())?;
            })
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Run {
    pub start: u8,
    pub end: u8,
    pub color: Color,
}

impl Run {
    pub fn valid(&self) -> bool {
        self.start < self.end && self.end - self.start >= 2
    }
}

impl fmt::Display for Run {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter: Vec<u8> = (self.start..self.end + 1).collect();
        write!(f, "{:<8}", "Run: ")?;
        Ok(for (index, value) in iter.iter().enumerate() {
            if index > 0 {
                write!(f, " ")?;
                // write!(f, " → ")?;
            }
            let tile = Tile {
                color: self.color,
                value: *value,
            };

            write!(f, "{}", tile.to_string())?;
        })
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Board {
    pub groups: Vec<Group>,
    pub runs: Vec<Run>,
}

impl Board {
    pub fn valid(&self) -> bool {
        for group in &self.groups {
            if !group.valid() {
                return false;
            }
        }

        for run in &self.runs {
            if !run.valid() {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Board =================================\n")?;
        for group in self.groups.iter() {
            writeln!(f, "{}", group)?;
        }
        for run in self.runs.iter() {
            writeln!(f, "{}", run)?;
        }
        write!(f, "=======================================\n")
    }
}

pub struct Player {
    pub tiles: Vec<Tile>,
}
