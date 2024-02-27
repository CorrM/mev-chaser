use std::fmt::{Display, Formatter};

#[repr(i32)]
#[derive(Debug, Clone)] // , PartialEq, Eq, Hash
pub enum NetworkKind {
    Ethereum = 1,
    Polygon = 137,
}

impl From<i32> for NetworkKind {
    fn from(value: i32) -> Self {
        match value {
            1 => NetworkKind::Ethereum,
            137 => NetworkKind::Polygon,
            _ => panic!("Invalid network kind: {}", value),
        }
    }
}

impl From<u32> for NetworkKind {
    fn from(value: u32) -> Self {
        NetworkKind::from(value as i32)
    }
}

impl From<NetworkKind> for i32 {
    fn from(val: NetworkKind) -> Self {
        val as i32
    }
}

impl Display for NetworkKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            NetworkKind::Ethereum => write!(f, "Ethereum"),
            NetworkKind::Polygon => write!(f, "Polygon"),
        }
    }
}
