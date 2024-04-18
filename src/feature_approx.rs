use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::{
    si::{Dimension, Quantity, Units},
    Conversion,
};

impl<
        D: Dimension + ?Sized,
        U: Units<V> + ?Sized,
        V: crate::num::Num + Conversion<V> + AbsDiffEq,
    > AbsDiffEq for Quantity<D, U, V>
{
    type Epsilon = V::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        V::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        V::abs_diff_eq(&self.value, &other.value, epsilon)
    }
}

impl<
        D: Dimension + ?Sized,
        U: Units<V> + ?Sized,
        V: crate::num::Num + Conversion<V> + RelativeEq,
    > RelativeEq for Quantity<D, U, V>
{
    fn default_max_relative() -> Self::Epsilon {
        V::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        V::relative_eq(&self.value, &other.value, epsilon, max_relative)
    }
}
impl<D: Dimension + ?Sized, U: Units<V> + ?Sized, V: crate::num::Num + Conversion<V> + UlpsEq>
    UlpsEq for Quantity<D, U, V>
{
    fn default_max_ulps() -> u32 {
        V::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        V::ulps_eq(&self.value, &other.value, epsilon, max_ulps)
    }
}
