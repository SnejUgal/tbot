//! Types representing an inline message ID.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents an inline message ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InlineMessageId<'a>(pub Cow<'a, str>);

impl<'a> InlineMessageId<'a> {
    /// Create a new reference to an inline message ID.
    #[must_use]
    pub fn as_borrowed(&'a self) -> Self {
        Self(Cow::Borrowed(&self.0))
    }
}

impl<'a> From<String> for InlineMessageId<'a> {
    #[must_use]
    fn from(id: String) -> Self {
        Self(id.into())
    }
}

impl<'a> From<&'a str> for InlineMessageId<'a> {
    #[must_use]
    fn from(id: &'a str) -> Self {
        Self(id.into())
    }
}
