use super::call_method;
use crate::{connectors::Client, errors, token, types::shipping};
use serde::Serialize;
use std::borrow::Cow;

/// Answers a shipping query.
///
/// Reflects the [`answerShippingQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answershippingquery
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerShippingQuery<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    shipping_query_id: shipping::query::id::Ref<'a>,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Cow<'a, [shipping::Option<'a>]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<Cow<'a, str>>,
}

impl<'a> AnswerShippingQuery<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        shipping_query_id: shipping::query::id::Ref<'a>,
        result: Result<
            impl Into<Cow<'a, [shipping::Option<'a>]>>,
            impl Into<Cow<'a, str>>,
        >,
    ) -> Self {
        if result.is_ok() {
            Self {
                client,
                token,
                shipping_query_id,
                ok: true,
                shipping_options: result.ok().map(Into::into),
                error_message: None,
            }
        } else {
            Self {
                client,
                token,
                shipping_query_id,
                ok: false,
                shipping_options: None,
                error_message: result.err().map(Into::into),
            }
        }
    }
}

impl AnswerShippingQuery<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "answerShippingQuery",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
