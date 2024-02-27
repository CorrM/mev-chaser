pub(super) trait AmmProtocol: Send + Sync {
    fn name(&self) -> &str;
}
