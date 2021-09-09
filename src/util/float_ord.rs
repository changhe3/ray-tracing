use std::marker::PhantomData;
use std::ops::{Bound, RangeBounds};

use decorum::{ConstrainedFloat, Constraint, Float, Primitive};
use partial_application::partial;
use tap::Pipe;

type Invariant<T> = PhantomData<*const T>;

pub struct FloatRange<R, T, P> {
    inner: R,
    _t: Invariant<T>,
    _p: Invariant<P>,
}

impl<R, T, P> FloatRange<R, T, P> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            _t: PhantomData,
            _p: PhantomData,
        }
    }
}

impl<R: RangeBounds<T>, T: Primitive + Float, P: Constraint<T>> RangeBounds<ConstrainedFloat<T, P>>
    for FloatRange<R, T, P>
{
    fn start_bound(&self) -> Bound<&ConstrainedFloat<T, P>> {
        self.inner
            .start_bound()
            .pipe(partial!(map_bound; _, ConstrainedFloat::from_ref))
    }

    fn end_bound(&self) -> Bound<&ConstrainedFloat<T, P>> {
        self.inner
            .end_bound()
            .pipe(partial!(map_bound; _, ConstrainedFloat::from_ref))
    }
}

fn map_bound<T, U, F: FnOnce(T) -> U>(bound: Bound<T>, f: F) -> Bound<U> {
    match bound {
        Bound::Included(val) => Bound::Included(f(val)),
        Bound::Excluded(val) => Bound::Excluded(f(val)),
        Bound::Unbounded => Bound::Unbounded,
    }
}
