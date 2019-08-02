use crate::{
    methods::AnswerPreCheckoutQuery,
    types::{pre_checkout_query, value, OrderInfo, PreCheckoutQuery, User},
    Bot,
};
use std::sync::Arc;

common! {
    /// The context for [`pre_checkout`][handler] handlers.
    ///
    /// [handler]: ../event_loop/struct.EventLoop.html#method.pre_checkout
    struct PreCheckout {
        /// The ID of the query.
        id: pre_checkout_query::Id,
        /// The user who sent the query.
        from: User,
        /// The currency of of the invoice.
        currency: String,
        /// The total price.
        total_amount: u32,
        /// The invoice payload sent previously by the bot.
        invoice_payload: String,
        /// The ID of the chosen shipping option.
        shipping_option_id: Option<String>,
        /// The order information.
        order_info: Option<OrderInfo>,
    }
}

impl<C> PreCheckout<C> {
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(bot: Arc<Bot<C>>, query: PreCheckoutQuery) -> Self {
        Self {
            bot,
            id: query.id,
            from: query.from,
            currency: query.currency,
            total_amount: query.total_amount,
            invoice_payload: query.invoice_payload,
            shipping_option_id: query.shipping_option_id,
            order_info: query.order_info,
        }
    }

    /// Reports if the checkout is possible.
    ///
    /// Note that this method suits better when you already deal with
    /// an `Option`. You might also want to use the [`ok`] and [`err`]
    /// methods from this context.
    ///
    /// [`ok`]: #method.ok
    /// [`err`]: #method.err
    pub fn answer<'a>(
        &'a self,
        result: Result<(), value::String<'a>>,
    ) -> AnswerPreCheckoutQuery<'a, C> {
        self.bot.answer_pre_checkout_query(&self.id, result)
    }

    /// Reports that shipping is possible and shows possible shipping options.
    pub fn ok(&self) -> AnswerPreCheckoutQuery<'_, C> {
        self.answer(Ok(()))
    }

    /// Reports that shipping is impossible and shows the error message.
    pub fn err<'a>(
        &'a self,
        err: impl Into<value::String<'a>>,
    ) -> AnswerPreCheckoutQuery<'a, C> {
        self.answer(Err(err.into()))
    }
}
