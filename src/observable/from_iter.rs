use crate::observable::{LocalObservable, Observable};
use crate::observer::Observer;
use crate::subscription::SingleSubscription;

pub fn from_iter<Iter>(iter: Iter) -> ObservableIter<Iter>
    where
        Iter: IntoIterator {
    ObservableIter(iter)
}

#[derive(Clone)]
pub struct ObservableIter<Iter>(Iter);

impl<Iter> Observable for ObservableIter<Iter>
    where
        Iter: IntoIterator,
{
    type Item = Iter::Item;
    type Err = ();
}

impl<Iter> LocalObservable
for ObservableIter<Iter>
    where Iter: IntoIterator,
{
    type Unsub = SingleSubscription;

    fn actual_subscribe<O>(self, mut _observer: O) -> Self::Unsub
        where
            O: Observer<Item=Self::Item, Err=Self::Err>
    {
        self.0.into_iter().for_each(|v| _observer.next(v));
        _observer.complete();
        SingleSubscription::default()
    }
}