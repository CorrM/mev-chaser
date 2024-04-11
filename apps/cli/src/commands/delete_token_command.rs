use anyhow::Result;

use shared::database::Database;
use vidger::logger::info;
use vidger::types::NetworkKind;

pub struct DeleteTokenCommand;

impl DeleteTokenCommand {
    pub fn process(db: &Database, target_network: &NetworkKind, token_address: &str) -> Result<()> {
        db.delete_token(target_network, token_address)?;
        info!("Token '{}' deleted", token_address);

        Ok(())
    }
}
