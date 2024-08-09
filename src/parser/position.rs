use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position
{
    pub line: u32,
    pub column: u32
}

impl fmt::Display for Position
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "line: {}, column: {}.", self.line, self.column)
    }
}