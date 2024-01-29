use super::{AmmPool, AmmProtocolKind};

pub trait AmmProtocol {
    fn name(&self) -> &str;
    fn kind(&self) -> AmmProtocolKind;
    fn pools(&self) -> &Vec<impl AmmPool>;
}