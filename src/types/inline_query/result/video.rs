//! Types for representing [`InlineQueryResult::Video`][docs].
//!
//! [docs]: ../enum.InlineQueryResult.html#variant.Video

use crate::types::{
    file,
    parameters::{ParseMode, Text},
    InputMessageContent, InteriorBorrow,
};
use is_macro::Is;
use serde::Serialize;
use std::borrow::Cow;

/// Represents possible MIME types.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
#[non_exhaustive]
#[must_use]
pub enum MimeType {
    /// The `text/html` MIME type.
    #[serde(rename = "text/html")]
    TextHtml,
    /// The `video/mp4` MIME type.
    #[serde(rename = "video/mp4")]
    #[is(name = "video_mp4")]
    VideoMp4,
}

/// Represents a non-cached video.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Fresh<'a> {
    #[serde(rename = "video_url")]
    url: Cow<'a, str>,
    mime_type: MimeType,
    thumb_url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "video_width")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "video_height")]
    height: Option<usize>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "video_duration"
    )]
    duration: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
#[must_use]
enum Kind<'a> {
    Cached {
        #[serde(rename = "video_file_id")]
        id: file::Id<'a>,
    },
    Fresh(Fresh<'a>),
}

/// Represents an [`InlineQueryResultVideo`]/[`InlineQueryResultCachedVideo`].
///
/// [`InlineQueryResultVideo`]: https://core.telegram.org/bots/api#inlinequeryresultvideo
/// [`InlineQueryResultCachedVideo`]: https://core.telegram.org/bots/api#inlinequeryresultcachedvideo
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Video<'a> {
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
    /// Constructs a `Fresh` video.
    pub fn new(
        url: impl Into<Cow<'a, str>>,
        mime_type: MimeType,
        thumb_url: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            url: url.into(),
            mime_type,
            thumb_url: thumb_url.into(),
            width: None,
            height: None,
            duration: None,
        }
    }

    /// Configures the width of the video.
    pub const fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures the height of the video.
    pub const fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Configures the duration of the video.
    pub const fn duration(mut self, duration: usize) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl<'a> Video<'a> {
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

    /// Constructs a cached `Video` result.
    pub fn with_cached(
        title: impl Into<Cow<'a, str>>,
        id: file::Id<'a>,
    ) -> Self {
        Self::new(title, Kind::Cached { id })
    }

    /// Constructs a fresh `Video` result.
    pub fn with_fresh(
        title: impl Into<Cow<'a, str>>,
        video: Fresh<'a>,
    ) -> Self {
        Self::new(title, Kind::Fresh(video))
    }

    /// Configures the description of the result.
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Configures the caption of the video.
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
            thumb_url: self.thumb_url.borrow_inside(),
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

impl<'a> InteriorBorrow<'a> for Video<'a> {
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
