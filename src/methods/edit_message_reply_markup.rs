use super::*;
use crate::internal::Client;
use std::sync::Arc;

/// Represents the [`editMessageReplyMarkup`][docs] method for chat messsages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagereplymarkup
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageReplyMarkup<'a, C> {
    #[serde(skip)]
    client: Arc<Client<C>>,
    #[serde(skip)]
    token: Token,
    chat_id: types::ChatId<'a>,
    message_id: u32,
    reply_markup: types::InlineKeyboard<'a>,
}

impl<'a, C> EditMessageReplyMarkup<'a, C> {
    pub(crate) fn new(
        client: Arc<Client<C>>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
            reply_markup,
        }
    }
}

impl<C> IntoFuture for EditMessageReplyMarkup<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.client,
            &self.token,
            "editMessageReplyMarkup",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
