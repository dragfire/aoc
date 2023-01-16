use std::marker::PhantomData;
use crate::observable::LocalObservable;
use crate::subscription::{SubscriptionLike, SubscriptionWrapper};

pub trait Observer {
    type Item;
    type Err;

    fn next(&mut self, value: Self::Item);

    fn error(&mut self, err: Self::Err);

    fn complete(&mut self);
}

#[derive(Clone)]
pub struct ObserverN<N, Item> {
    next: N,
    is_stopped: bool,
    _marker: PhantomData<Item>
}

impl<Item, N> Observer for ObserverN<N, Item>
    where
        N: FnMut(Item),
{
    type Item = Item;
    type Err = ();

    fn next(&mut self, value: Self::Item) {
        if !self.is_stopped {
            (self.next)(value);
        }
    }

    #[inline]
    fn error(&mut self, _err: ()) {
        self.is_stopped = true;
    }

    #[inline]
    fn complete(&mut self) {
        self.is_stopped = true;
    }
}

pub trait SubscribeNext<N> {
    type Unsub: SubscriptionLike;

    fn subscribe(self, next: N) -> SubscriptionWrapper<Self::Unsub>;
}

impl<S, N> SubscribeNext<N> for S
    where S: LocalObservable<Err=()>,
          N: FnMut(S::Item) {
    type Unsub = S::Unsub;

    fn subscribe(self, next: N) -> SubscriptionWrapper<Self::Unsub> {
        let unsub = self.actual_subscribe(ObserverN {
            next,
            is_stopped: false,
            _marker: Default::default()
        });

        SubscriptionWrapper(unsub)
    }
}