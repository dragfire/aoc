// subscription
pub trait SubscriptionLike {
    fn unsubscribe(&mut self);

    fn is_closed(&self) -> bool;
}

#[derive(Default, Clone)]
pub struct SingleSubscription(bool);

impl SubscriptionLike for SingleSubscription {
    #[inline]
    fn unsubscribe(&mut self) {
        self.0 = true;
    }

    #[inline]
    fn is_closed(&self) -> bool {
        self.0
    }
}

pub struct SubscriptionWrapper<T: SubscriptionLike>(pub(crate) T);

impl<T: SubscriptionLike> SubscriptionLike for SubscriptionWrapper<T> {
    #[inline]
    fn unsubscribe(&mut self) {
        self.0.unsubscribe();
    }

    #[inline]
    fn is_closed(&self) -> bool {
        self.0.is_closed()
    }
}

