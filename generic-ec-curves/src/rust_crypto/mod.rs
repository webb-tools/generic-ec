use core::fmt;
use core::hash::{self, Hash};
use core::marker::PhantomData;
use core::ops::Mul;

use elliptic_curve::group::cofactor::CofactorGroup;
use elliptic_curve::hash2curve::ExpandMsgXmd;
use elliptic_curve::ops::Reduce;
use elliptic_curve::sec1::{FromEncodedPoint, ModulusSize, ToEncodedPoint};
use elliptic_curve::{CurveArithmetic, FieldBytesSize, ScalarPrimitive};
use generic_ec_core::{CompressedEncoding, Curve, IntegerEncoding, UncompressedEncoding};
use subtle::{ConditionallySelectable, ConstantTimeEq};
use zeroize::{DefaultIsZeroes, Zeroize};

#[cfg(any(feature = "secp256k1", feature = "secp256r1", feature = "stark"))]
use sha2::Sha256;

pub use self::{curve_name::CurveName, point::RustCryptoPoint, scalar::RustCryptoScalar};

mod affine_coords;
mod curve_name;
mod hash_to_curve;
mod point;
mod scalar;

pub struct RustCryptoCurve<C, X> {
    _ph: PhantomData<fn() -> (C, X)>,
}

/// secp256k1 curve
///
/// Based on [k256] crate
#[cfg(feature = "secp256k1")]
pub type Secp256k1 = RustCryptoCurve<k256::Secp256k1, ExpandMsgXmd<Sha256>>;
/// secp256r1 curve
///
/// Based on [p256] crate
#[cfg(feature = "secp256r1")]
pub type Secp256r1 = RustCryptoCurve<p256::NistP256, ExpandMsgXmd<Sha256>>;

#[cfg(feature = "stark")]
pub type Stark = RustCryptoCurve<stark_curve::StarkCurve, ExpandMsgXmd<Sha256>>;

impl<C, X> Curve for RustCryptoCurve<C, X>
where
    C: CurveName + CurveArithmetic,
    C::ProjectivePoint: From<C::AffinePoint>
        + CofactorGroup
        + Copy
        + Eq
        + Default
        + ConstantTimeEq
        + ConditionallySelectable
        + Zeroize
        + Unpin,
    C::AffinePoint: From<C::ProjectivePoint> + ToEncodedPoint<C> + FromEncodedPoint<C>,
    for<'a> &'a C::ProjectivePoint: Mul<&'a C::Scalar, Output = C::ProjectivePoint>,
    C::Scalar:
        Reduce<C::Uint> + Eq + ConstantTimeEq + ConditionallySelectable + DefaultIsZeroes + Unpin,
    for<'a> ScalarPrimitive<C>: From<&'a C::Scalar>,
    FieldBytesSize<C>: ModulusSize,
    X: 'static,
{
    const CURVE_NAME: &'static str = C::CURVE_NAME;

    type Point = RustCryptoPoint<C>;
    type Scalar = RustCryptoScalar<C>;

    type CompressedPointArray = <Self::Point as CompressedEncoding>::Bytes;
    type UncompressedPointArray = <Self::Point as UncompressedEncoding>::Bytes;

    type ScalarArray = <Self::Scalar as IntegerEncoding>::Bytes;

    type CoordinateArray = elliptic_curve::FieldBytes<C>;
}

impl<C: CurveName, X> fmt::Debug for RustCryptoCurve<C, X> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RustCryptoCurve")
            .field("curve", &C::CURVE_NAME)
            .finish()
    }
}

impl<C, X> Clone for RustCryptoCurve<C, X> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C, X> Copy for RustCryptoCurve<C, X> {}

impl<C, X> PartialEq for RustCryptoCurve<C, X> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<C, X> Eq for RustCryptoCurve<C, X> {}

impl<C, X> PartialOrd for RustCryptoCurve<C, X> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C, X> Ord for RustCryptoCurve<C, X> {
    fn cmp(&self, _other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}

impl<C, X> Hash for RustCryptoCurve<C, X>
where
    C: CurveName,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(C::CURVE_NAME.as_bytes())
    }
}

impl<C, X> Default for RustCryptoCurve<C, X> {
    fn default() -> Self {
        Self { _ph: PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use generic_ec_core::{
        coords::{HasAffineX, HasAffineXAndParity, HasAffineXY},
        hash_to_curve::HashToCurve,
        Curve,
    };

    use super::{Secp256k1, Secp256r1};

    /// Asserts that `E` implements `Curve`
    fn _impls_curve<E: Curve>() {}
    fn _exposes_affine_coords<E: HasAffineX + HasAffineXAndParity + HasAffineXY>() {}
    fn _impls_hash_to_curve<E: HashToCurve>() {}

    fn _curves_impl_trait() {
        _impls_curve::<Secp256k1>();
        _impls_curve::<Secp256r1>();

        _exposes_affine_coords::<Secp256k1>();
        _exposes_affine_coords::<Secp256r1>();

        _impls_hash_to_curve::<Secp256k1>();
        _impls_hash_to_curve::<Secp256r1>();
    }
}
