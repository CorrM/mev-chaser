use anyhow::Result;
use async_trait::async_trait;
use teloxide::prelude::ChatId;
use teloxide::types::Recipient;
use teloxide::{requests::Requester, Bot};

use crate::core::Notifier;
use crate::types::Notification;

pub struct TelegramNotifier {
    bot: Bot,
    chat_id: Recipient,
}

impl TelegramNotifier {
    pub fn new(token: impl Into<String>, chat_id: impl Into<String>) -> Self {
        let bot = Bot::new(token);

        Self {
            bot,
            chat_id: Recipient::Id(ChatId(chat_id.into().parse::<i64>().unwrap())),
        }
    }
}

#[async_trait]
impl Notifier for TelegramNotifier {
    #[inline]
    async fn notify(&self, notification: Notification) -> Result<()> {
        self.bot
            .send_message(self.chat_id.clone(), notification.message)
            .await?;

        Ok(())
    }
}
