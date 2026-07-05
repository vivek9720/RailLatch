use core::fmt;

pub type Result<T> = core::result::Result<T, RailError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RailError {
    Empty,
    ShortRead { needed: usize, remaining: usize },
    BadMagic,
    BadVersion(u8),
    BadSection(&'static str),
    BadFrame(&'static str),
    BadScript(&'static str),
    LimitExceeded(&'static str),
}

impl fmt::Display for RailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RailError::Empty => write!(f, "empty input"),
            RailError::ShortRead { needed, remaining } => {
                write!(f, "short read: needed {needed}, remaining {remaining}")
            }
            RailError::BadMagic => write!(f, "bad magic"),
            RailError::BadVersion(v) => write!(f, "bad version {v}"),
            RailError::BadSection(s) => write!(f, "bad section: {s}"),
            RailError::BadFrame(s) => write!(f, "bad frame: {s}"),
            RailError::BadScript(s) => write!(f, "bad script: {s}"),
            RailError::LimitExceeded(s) => write!(f, "limit exceeded: {s}"),
        }
    }
}

impl std::error::Error for RailError {}
