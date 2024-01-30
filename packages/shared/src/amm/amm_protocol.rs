use super::AmmPool;

pub trait AmmProtocol {
    fn name(&self) -> &str;
    fn pools(&self) -> &Vec<impl AmmPool>;
}