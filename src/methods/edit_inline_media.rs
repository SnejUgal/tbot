use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    types::{inline_message_id, input_file::*, keyboard::inline},
};

/// Edits the media of a message sent via the inline mode.
///
/// Reflects the [`editMessageMedia`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagemedia
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineMedia<'a, C> {
    client: &'a Client<C>,
    token: Token,
    inline_message_id: inline_message_id::Ref<'a>,
    media: EditableMedia<'a>,
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a, C> EditInlineMedia<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: inline_message_id::Ref<'a>,
        media: impl Into<EditableMedia<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id,
            media: media.into(),
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl<C: Connector> IntoFuture for EditInlineMedia<'_, C> {
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let mut multipart = Multipart::new(4)
            .str("inline_message_id", self.inline_message_id.0)
            .maybe_json("reply_markup", self.reply_markup);

        match &self.media {
            EditableMedia::Animation(Animation { media, .. })
            | EditableMedia::Audio(Audio { media, .. })
            | EditableMedia::Document(Document { media, .. })
            | EditableMedia::Photo(Photo { media, .. })
            | EditableMedia::Video(Video { media, .. }) => {
                if let InputFile::File { filename, bytes } = media {
                    multipart =
                        multipart.file(self.media.name(), filename, bytes);
                }
            }
        }

        let (boundary, body) = multipart.json("media", self.media).finish();

        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "editMessageMedia",
                Some(boundary),
                body,
            )
            .map(|_| ()),
        )
    }
}
