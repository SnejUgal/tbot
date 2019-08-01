use crate::types::{
    parameters::{self, ParseMode, WebPagePreviewState},
    value,
};
use serde::Serialize;

/// Represents an [`InputTextMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputtextmessagecontent
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Text<'a> {
    message_text: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
}

impl<'a> Text<'a> {
    /// Constructs a new `Text`.
    pub fn new(message_text: impl Into<parameters::Text<'a>>) -> Self {
        let parameters::Text {
            text,
            parse_mode,
        } = message_text.into();

        Self {
            message_text: text,
            parse_mode,
            disable_web_page_preview: None,
        }
    }

    /// Configures if the web page preview will be shown.
    pub fn web_page_preview(mut self, state: WebPagePreviewState) -> Self {
        self.disable_web_page_preview = Some(state.is_disabled());
        self
    }
}
