use crate::observable::{LocalObservable, Observable};
use crate::observer::Observer;

#[derive(Clone)]
pub struct FilterOp<S, F> {
    pub source: S,
    pub filter: F,
}

impl<S, F> Observable for FilterOp<S, F>
    where S: Observable, F: FnMut(&S::Item) -> bool {
    type Item = S::Item;
    type Err = S::Err;
}

impl<S, F> LocalObservable for FilterOp<S, F>
    where S: LocalObservable,
          F: FnMut(&S::Item) -> bool
{
    type Unsub = S::Unsub;

    fn actual_subscribe<O>(self, mut _observer: O) -> Self::Unsub
        where O: Observer<Item=Self::Item, Err=Self::Err> {
        let filter = self.filter;
        self.source.actual_subscribe(FilterObserver {
            observer: _observer,
            filter,
        })
    }
}

struct FilterObserver<O, F> {
    observer: O,
    filter: F,
}

impl<Item, Err, O, F> Observer for FilterObserver<O, F>
    where
        O: Observer<Item=Item, Err=Err>,
        F: FnMut(&Item) -> bool
{
    type Item = Item;
    type Err = Err;

    fn next(&mut self, value: Self::Item) {
        if (self.filter)(&value) {
            self.observer.next(value);
        }
    }

    fn error(&mut self, err: Self::Err) {
        self.observer.error(err);
    }

    fn complete(&mut self) {
        self.observer.complete();
    }
}

#[cfg(test)]
mod tests {
    use crate::observable;
    use crate::observer::SubscribeNext;

    use super::*;

    #[test]
    fn filter_works() {
        let mut total = 0;

        observable::from_iter(vec![1, 2, 3, 4, 5, 6])
            .map(|v| v + 100)
            .filter(|v| v % 2 == 0)
            .map(|v| v - 100)
            .map(|v| v * 100)
            .subscribe(|v| {
                total += v;
            });

        assert_eq!(1200, total);
    }
}
