use super::*;

/// Representation of the [`editMessageLiveLocation`] method for the case if
/// the message was sent by the bot.
///
/// [`editMessageLiveLocation`]: https://core.telegram.org/bots/api#editmessagelivelocation
#[derive(Serialize)]
pub struct EditMessageLocation<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    message_id: u64,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::raw::Keyboard<'a>>,
}

impl<'a> EditMessageLocation<'a> {
    /// Constructs a new `EditMessageLocation`.
    #[must_use]
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        message_id: u64,
        (latitude, longitude): (f64, f64),
    ) -> EditMessageLocation<'a> {
        EditMessageLocation {
            token,
            chat_id: chat_id.into(),
            message_id,
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    /// Sets `reply_markup` to `Some(markup)`.
    #[must_use]
    pub fn reply_markup(
        mut self,
        markup: impl Into<types::raw::Keyboard<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::raw::Message, Error = DeliveryError> {
        send_method::<types::raw::Message>(
            self.token,
            "editMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}
