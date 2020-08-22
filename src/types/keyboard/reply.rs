//! Types representing reply keyboards.

use is_macro::Is;
use serde::{ser::SerializeMap, Serialize};
use std::borrow::Cow;

/// A shorthand for reply markup.
pub type Markup<'a> = &'a [&'a [Button<'a>]];

const REGULAR: &str = "regular";
const QUIZ: &str = "quiz";

/// Represents different kinds of poll that a [`Button`] can request.
///
/// [`Button`]: ./struct.Button.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[must_use]
pub enum RequestPollKind {
    /// Allows the user to create a poll of any type.
    Any,
    /// Allows the user to create only a regular poll.
    Regular,
    /// Allows the user to create only a quiz poll.
    Quiz,
}

impl<'a> serde::Serialize for RequestPollKind {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map =
            s.serialize_map(Some(if *self == Self::Any { 0 } else { 1 }))?;

        match self {
            Self::Any => Ok(()),
            Self::Regular => map.serialize_entry("type", REGULAR),
            Self::Quiz => map.serialize_entry("type", QUIZ),
        }?;

        map.end()
    }
}

/// Represents different information that a [`Button`] can request.
///
/// [`Button`]: ./struct.Button.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[must_use]
pub enum RequestKind {
    /// Requests a location from the user.
    Location,
    /// Requests a contact from the user.
    Contact,
    /// Requests a poll from the user.
    Poll(RequestPollKind),
}

/// Represents a [`KeyboardButton`].
///
/// [`KeyboardButton`]: https://core.telegram.org/bots/api#keyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Button<'a> {
    text: Cow<'a, str>,
    request: Option<RequestKind>,
}

/// Represents a [`ReplyKeyboardMarkup`].
///
/// [`ReplyKeyboardMarkup`]: https://core.telegram.org/bots/api#replykeyboardmarkup
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Keyboard<'a> {
    keyboard: Markup<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resize_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    one_time_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,
}

/// Represents a [`ReplyKeyboardRemove`].
///
/// [`ReplyKeyboardRemove`]: https://core.telegram.org/bots/api#replykeyboardremove
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[must_use]
pub struct Remove {
    // remove_keyboard is added when serializing
    selective: Option<bool>,
}

impl<'a> Button<'a> {
    /// Constructs a reply `Button`.
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            request: None,
        }
    }

    /// Configures `request`.
    pub const fn request(mut self, request: RequestKind) -> Self {
        self.request = Some(request);
        self
    }
}

impl<'a> serde::Serialize for Button<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let len = if self.request.is_some() { 2 } else { 1 };

        let mut map = s.serialize_map(Some(len))?;

        map.serialize_entry("text", &self.text)?;

        match self.request {
            Some(RequestKind::Location) => {
                map.serialize_entry("request_location", &true)
            }
            Some(RequestKind::Contact) => {
                map.serialize_entry("request_contact", &true)
            }
            Some(RequestKind::Poll(poll_kind)) => {
                map.serialize_entry("request_poll", &poll_kind)
            }
            None => Ok(()),
        }?;

        map.end()
    }
}

impl<'a> Keyboard<'a> {
    /// Constructs a reply `Keyboard`.
    pub const fn new(keyboard: Markup<'a>) -> Self {
        Self {
            keyboard,
            resize_keyboard: None,
            one_time_keyboard: None,
            selective: None,
        }
    }

    /// Configures `resize_keyboard`.
    pub const fn resize_keyboard(mut self, is_resized: bool) -> Self {
        self.resize_keyboard = Some(is_resized);
        self
    }

    /// Configures `one_time_keyboard`.
    pub const fn one_time_keyboard(mut self, is_one_time: bool) -> Self {
        self.one_time_keyboard = Some(is_one_time);
        self
    }

    /// Configures `selective`.
    pub const fn selective(mut self, is_selective: bool) -> Self {
        self.selective = Some(is_selective);
        self
    }
}

impl<'a> From<Markup<'a>> for Keyboard<'a> {
    fn from(markup: Markup<'a>) -> Self {
        Self::new(markup)
    }
}

impl Remove {
    /// Constructs a `reply::Remove`.
    pub const fn new() -> Self {
        Self { selective: None }
    }

    /// Configures `selective`.
    pub const fn selective(mut self, is_selective: bool) -> Self {
        self.selective = Some(is_selective);
        self
    }
}

impl serde::Serialize for Remove {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let len = if self.selective.is_some() { 2 } else { 1 };

        let mut map = s.serialize_map(Some(len))?;

        map.serialize_entry("remove_keyboard", &true)?;

        if let Some(selective) = self.selective {
            map.serialize_entry("selective", &selective)?;
        }

        map.end()
    }
}
