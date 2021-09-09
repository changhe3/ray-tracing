use nalgebra::{
    allocator::Allocator, AbstractRotation, ClosedAdd, ClosedSub, Const, DefaultAllocator,
    DimNameAdd, DimNameSum, Isometry, Point, RealField, Rotation, SVector, Scalar, SimdRealField,
    Similarity, SubTCategoryOf, TCategory, TProjective, Transform, Translation, UnitComplex,
    UnitDualQuaternion, UnitQuaternion, U1,
};

pub trait GenericTransform<T: Scalar + Clone, const D: usize> {
    fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D>;

    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D>;
}

pub trait InverseTransform<T: Scalar + Clone, const D: usize> {
    fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D>;

    fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D>;
}

impl<T: RealField, C: TCategory, const D: usize> GenericTransform<T, D> for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
        + Allocator<T, DimNameSum<Const<D>, U1>>,
{
    fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.transform_point(pt)
    }

    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.transform_vector(v)
    }
}

impl<T: RealField, C: TCategory, const D: usize> InverseTransform<T, D> for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    C: SubTCategoryOf<TProjective>,
    DefaultAllocator: Allocator<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
        + Allocator<T, DimNameSum<Const<D>, U1>>,
{
    fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.inverse_transform_point(pt)
    }

    fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.inverse_transform_vector(v)
    }
}

impl<T: Scalar + ClosedAdd, const D: usize> GenericTransform<T, D> for Translation<T, D> {
    fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.transform_point(pt)
    }

    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        v.clone()
    }
}

impl<T: Scalar + ClosedSub, const D: usize> InverseTransform<T, D> for Translation<T, D> {
    fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.inverse_transform_point(pt)
    }

    fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        v.clone()
    }
}

impl<T: SimdRealField, const D: usize> GenericTransform<T, D> for Rotation<T, D>
where
    T::Element: SimdRealField,
{
    fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.transform_point(pt)
    }

    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.transform_vector(v)
    }
}

impl<T: SimdRealField, const D: usize> InverseTransform<T, D> for Rotation<T, D>
where
    T::Element: SimdRealField,
{
    fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.inverse_transform_point(pt)
    }

    fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.inverse_transform_vector(v)
    }
}

impl<T: SimdRealField> GenericTransform<T, 2> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    fn transform_point(&self, pt: &Point<T, 2>) -> Point<T, 2> {
        self.transform_point(pt)
    }

    fn transform_vector(&self, v: &SVector<T, 2>) -> SVector<T, 2> {
        self.transform_vector(v)
    }
}

impl<T: SimdRealField> InverseTransform<T, 2> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    fn inverse_transform_point(&self, pt: &Point<T, 2>) -> Point<T, 2> {
        self.inverse_transform_point(pt)
    }

    fn inverse_transform_vector(&self, v: &SVector<T, 2>) -> SVector<T, 2> {
        self.inverse_transform_vector(v)
    }
}

impl<T: SimdRealField> GenericTransform<T, 3> for UnitQuaternion<T>
where
    T::Element: SimdRealField,
{
    fn transform_point(&self, pt: &Point<T, 3>) -> Point<T, 3> {
        self.transform_point(pt)
    }

    fn transform_vector(&self, v: &SVector<T, 3>) -> SVector<T, 3> {
        self.transform_vector(v)
    }
}

impl<T: SimdRealField> InverseTransform<T, 3> for UnitQuaternion<T>
where
    T::Element: SimdRealField,
{
    fn inverse_transform_point(&self, pt: &Point<T, 3>) -> Point<T, 3> {
        self.inverse_transform_point(pt)
    }

    fn inverse_transform_vector(&self, v: &SVector<T, 3>) -> SVector<T, 3> {
        self.inverse_transform_vector(v)
    }
}

impl<T: SimdRealField> GenericTransform<T, 3> for UnitDualQuaternion<T>
where
    T::Element: SimdRealField,
{
    fn transform_point(&self, pt: &Point<T, 3>) -> Point<T, 3> {
        self.transform_point(pt)
    }

    fn transform_vector(&self, v: &SVector<T, 3>) -> SVector<T, 3> {
        self.transform_vector(v)
    }
}

impl<T: SimdRealField> InverseTransform<T, 3> for UnitDualQuaternion<T>
where
    T::Element: SimdRealField,
{
    fn inverse_transform_point(&self, pt: &Point<T, 3>) -> Point<T, 3> {
        self.inverse_transform_point(pt)
    }

    fn inverse_transform_vector(&self, v: &SVector<T, 3>) -> SVector<T, 3> {
        self.inverse_transform_vector(v)
    }
}

impl<T: SimdRealField, R: AbstractRotation<T, D>, const D: usize> GenericTransform<T, D>
    for Isometry<T, R, D>
where
    T::Element: SimdRealField,
{
    fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.transform_point(pt)
    }

    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.transform_vector(v)
    }
}

impl<T: SimdRealField, R: AbstractRotation<T, D>, const D: usize> InverseTransform<T, D>
    for Isometry<T, R, D>
where
    T::Element: SimdRealField,
{
    fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.inverse_transform_point(pt)
    }

    fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.inverse_transform_vector(v)
    }
}

impl<T: SimdRealField, R: AbstractRotation<T, D>, const D: usize> GenericTransform<T, D>
    for Similarity<T, R, D>
where
    T::Element: SimdRealField,
{
    fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.transform_point(pt)
    }

    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.transform_vector(v)
    }
}

impl<T: SimdRealField, R: AbstractRotation<T, D>, const D: usize> InverseTransform<T, D>
    for Similarity<T, R, D>
where
    T::Element: SimdRealField,
{
    fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.inverse_transform_point(pt)
    }

    fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.inverse_transform_vector(v)
    }
}
