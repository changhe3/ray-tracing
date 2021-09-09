use crate::geometry::ray::Ray;
use crate::util::float_ord::FloatRange;
use btreemultimap::BTreeMultiMap;
use decorum::Total;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::iter;
use std::iter::FromIterator;
use std::ops::RangeBounds;
use std::sync::atomic::{AtomicU64, Ordering::SeqCst};

pub mod ray;
pub mod sphere;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u64);

impl Id {
    const COUNTER: AtomicU64 = AtomicU64::new(0);

    pub fn new() -> Self {
        Self(Self::COUNTER.fetch_add(1, SeqCst))
    }

    pub fn count() -> u64 {
        Self::COUNTER.load(SeqCst)
    }
}

#[derive(Clone, Debug)]
pub struct Object<S> {
    id: Id,
    shape: S,
}

impl<S> Object<S> {
    pub fn new(shape: S) -> Self {
        let id = Id::new();
        Self { id, shape }
    }
}

impl<S> PartialEq for Object<S> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<S> Eq for Object<S> {}

impl<S> PartialOrd for Object<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.id, &other.id)
    }
}

impl<S> Ord for Object<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.id, &other.id)
    }
}

impl<S> Hash for Object<S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub trait Hittable {
    fn id(&self) -> Id;

    fn intersect(&self, ray: Ray) -> Hits;
}

impl<S: Shape> Hittable for Object<S> {
    fn id(&self) -> Id {
        self.id
    }

    fn intersect(&self, ray: Ray) -> Hits {
        self.shape
            .intersect(ray)
            .into_iter()
            .zip(iter::repeat(HitContext { obj_hit: self }))
            .collect()
    }
}

pub trait Shape {
    // not object safe
    type Hits: IntoIterator<Item = f32>;
    fn intersect(&self, ray: Ray) -> Self::Hits;

    fn into_object(self) -> Object<Self>
    where
        Self: Sized,
    {
        Object::new(self)
    }
}

impl PartialEq for dyn Hittable + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for dyn Hittable + '_ {}

impl PartialOrd for dyn Hittable + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for dyn Hittable + '_ {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[derive(Debug)]
        struct Hittable {
            id: Id,
        }

        let mimic = Hittable { id: self.id() };
        mimic.fmt(f)
    }
}

impl Ord for dyn Hittable + '_ {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.id(), &other.id())
    }
}

impl Hash for dyn Hittable + '_ {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hits<'a> {
    inner: BTreeMultiMap<Total<f32>, HitContext<'a>>,
}

impl<'a> Hits<'a> {
    pub fn hit(&self) -> Option<(f32, &HitContext<'a>)> {
        self.inner
            .range(FloatRange::new(0.0..))
            .map(|(t, ctx)| (t.into_inner(), ctx))
            .next()
    }

    pub fn hits_between<R: RangeBounds<f32>>(
        &self,
        range: R,
    ) -> impl Iterator<Item = (f32, &HitContext<'a>)> {
        self.inner
            .range(FloatRange::new(range))
            .map(|(t, ctx)| (t.into_inner(), ctx))
    }
}

impl<'a> FromIterator<(f32, HitContext<'a>)> for Hits<'a> {
    fn from_iter<T: IntoIterator<Item = (f32, HitContext<'a>)>>(iter: T) -> Self {
        let inner = iter.into_iter().map(|(t, ctx)| (t.into(), ctx)).collect();
        Self { inner }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct HitContext<'a> {
    pub obj_hit: &'a dyn Hittable,
}
