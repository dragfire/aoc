use crate::{observer::Observer,
            operators::MapOp};
use crate::prelude::FilterOp;
use crate::subscription::SubscriptionLike;

pub trait Observable: Sized {
    type Item;
    type Err;

    #[inline]
    fn map<B, F>(self, f: F) -> MapOp<Self, F>
        where F: FnMut(Self::Item) -> B {
        MapOp {
            source: self,
            func: f,
        }
    }

    #[inline]
    fn filter<F>(self, f: F) -> FilterOp<Self, F>
        where F: FnMut(&Self::Item) -> bool {
        FilterOp {
            source: self,
            filter: f,
        }
    }
}

pub trait LocalObservable: Observable {
    type Unsub: SubscriptionLike;

    fn actual_subscribe<O>(self, observer: O) -> Self::Unsub
        where
            O: Observer<Item=Self::Item, Err=Self::Err>;
}
