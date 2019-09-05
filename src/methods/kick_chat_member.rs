use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};

/// Kicks a member out of a chat.
///
/// Reflects the [`kickChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#kickchatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct KickChatMember<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    user_id: user::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
}

impl<'a, C> KickChatMember<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
        }
    }

    /// Configures when the user regains the ability to return to the chat.
    /// Reflects the `until_date` parameter.
    pub fn until_date(mut self, date: i64) -> Self {
        self.until_date = Some(date);
        self
    }
}

impl<C> IntoFuture for KickChatMember<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "kickChatMember",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on successы
        )
    }
}
