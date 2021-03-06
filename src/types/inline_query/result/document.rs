//! Types for representing [`InlineQueryResult::Document`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Document

use super::Thumb;
use crate::types::{
    file,
    parameters::{ParseMode, Text},
    InputMessageContent, InteriorBorrow,
};
use is_macro::Is;
use serde::Serialize;
use std::borrow::Cow;

/// Represents possible MIME types for a fresh document.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
#[non_exhaustive]
#[must_use]
pub enum MimeType {
    /// The `application/pdf` MIME type.
    #[serde(rename = "application/pdf")]
    ApplicationPdf,
    /// The `application/zip` MIME type.
    #[serde(rename = "application/zip")]
    ApplicationZip,
}

/// Represents a non-cached document.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Fresh<'a> {
    #[serde(rename = "document_url")]
    url: Cow<'a, str>,
    mime_type: MimeType,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    thumb: Option<Thumb<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
#[must_use]
enum Kind<'a> {
    Cached {
        #[serde(rename = "document_file_id")]
        id: file::Id<'a>,
    },
    Fresh(Fresh<'a>),
}

/// Represents an [`InlineQueryResultDocument`]/[`InlineQueryResultCachedDocument`].
///
/// [`InlineQueryResultDocument`]: https://core.telegram.org/bots/api#inlinequeryresultdocument
/// [`InlineQueryResultCachedDocument`]: https://core.telegram.org/bots/api#inlinequeryresultcacheddocument
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Document<'a> {
    #[serde(flatten)]
    kind: Kind<'a>,
    title: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl<'a> Fresh<'a> {
    /// Constructs a `Fresh` document.
    pub fn new(url: impl Into<Cow<'a, str>>, mime_type: MimeType) -> Self {
        Self {
            url: url.into(),
            mime_type,
            thumb: None,
        }
    }

    /// Configures the thumb of the document.
    #[allow(clippy::missing_const_for_fn)]
    pub fn thumb(mut self, thumb: Thumb<'a>) -> Self {
        self.thumb = Some(thumb);
        self
    }
}

impl<'a> Document<'a> {
    fn new(title: impl Into<Cow<'a, str>>, kind: Kind<'a>) -> Self {
        Self {
            kind,
            title: title.into(),
            description: None,
            caption: None,
            parse_mode: None,
            input_message_content: None,
        }
    }

    /// Constructs a cached `Document` result.
    pub fn with_cached(
        title: impl Into<Cow<'a, str>>,
        id: file::Id<'a>,
    ) -> Self {
        Self::new(title, Kind::Cached { id })
    }

    /// Constructs a fresh `Document` result.
    pub fn with_fresh(
        title: impl Into<Cow<'a, str>>,
        document: Fresh<'a>,
    ) -> Self {
        Self::new(title, Kind::Fresh(document))
    }

    /// Configures the description of the result.
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Configures the caption of the document.
    pub fn caption(mut self, caption: impl Into<Text<'a>>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text);
        self.parse_mode = caption.parse_mode;
        self
    }

    /// Configures the content shown after sending the message.
    pub fn input_message_content(
        mut self,
        content: impl Into<InputMessageContent<'a>>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}

impl<'a> InteriorBorrow<'a> for Fresh<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            url: self.url.borrow_inside(),
            thumb: self.thumb.borrow_inside(),
            ..*self
        }
    }
}

impl<'a> InteriorBorrow<'a> for Kind<'a> {
    fn borrow_inside(&'a self) -> Self {
        match self {
            Self::Cached { id } => Self::Cached {
                id: id.borrow_inside(),
            },
            Self::Fresh(fresh) => Self::Fresh(fresh.borrow_inside()),
        }
    }
}

impl<'a> InteriorBorrow<'a> for Document<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            kind: self.kind.borrow_inside(),
            title: self.title.borrow_inside(),
            description: self.description.borrow_inside(),
            caption: self.caption.borrow_inside(),
            input_message_content: self.input_message_content.borrow_inside(),
            ..*self
        }
    }
}
