use std::fmt::{Display, Formatter, Result};

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum NetworkKind {
    Ethereum = 1,
    Polygon = 137,
}

impl Display for NetworkKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            NetworkKind::Ethereum => write!(f, "Ethereum"),
            NetworkKind::Polygon => write!(f, "Polygon"),
        }
    }
}
