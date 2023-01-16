use std::marker::PhantomData;

use crate::observable::{LocalObservable, Observable};
use crate::observer::Observer;

#[derive(Clone)]
pub struct MapOp<S, M> {
    pub source: S,
    pub func: M,
}

impl<Item, S, M> Observable for MapOp<S, M>
    where S: Observable, M: FnMut(S::Item) -> Item
{
    type Item = Item;
    type Err = S::Err;
}

impl<Item, S, M> LocalObservable for MapOp<S, M>
    where S: LocalObservable,
          M: FnMut(S::Item) -> Item
{
    type Unsub = S::Unsub;

    fn actual_subscribe<O>(self, mut _observer: O) -> Self::Unsub
        where
            O: Observer<Item=Self::Item, Err=Self::Err>
    {
        let map = self.func;
        self.source
            .actual_subscribe(MapObserver {
                observer: _observer,
                map,
                _marker: Default::default(),
            })
    }
}

#[derive(Clone)]
struct MapObserver<O, M, Item> {
    observer: O,
    map: M,
    _marker: PhantomData<Item>,
}

impl<Item, Err, O, M, B> Observer for MapObserver<O, M, Item>
    where
        O: Observer<Item=B, Err=Err>,
        M: FnMut(Item) -> B,
{
    type Item = Item;
    type Err = Err;

    fn next(&mut self, value: Item) {
        self.observer.next((self.map)(value))
    }

    fn error(&mut self, err: Self::Err) {
        self.observer.error(err)
    }

    fn complete(&mut self) {
        self.observer.complete()
    }
}

#[cfg(test)]
mod tests {
    use crate::observable;
    use crate::observer::SubscribeNext;

    use super::*;

    #[test]
    fn map_works() {
        let mut total = 0;

        observable::from_iter(vec![1, 2, 3])
            .map(|v| v * 100)
            .subscribe(|v| {
                total += v;
            });

        assert_eq!(600, total);
    }
}
