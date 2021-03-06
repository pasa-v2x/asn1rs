use crate::syn::{ReadableType, Reader, WritableType, Writer};
use core::marker::PhantomData;

pub struct OctetString<C: Constraint = NoConstraint>(PhantomData<C>);

impl<C: Constraint> Default for OctetString<C> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub trait Constraint {
    const MIN: Option<usize> = None;
    const MAX: Option<usize> = None;
}

#[derive(Default)]
pub struct NoConstraint;
impl Constraint for NoConstraint {}

impl<C: Constraint> WritableType for OctetString<C> {
    type Type = Vec<u8>;

    #[inline]
    fn write_value<W: Writer>(writer: &mut W, value: &Self::Type) -> Result<(), W::Error> {
        writer.write_octet_string::<C>(value.as_slice())
    }
}

impl<C: Constraint> ReadableType for OctetString<C> {
    type Type = Vec<u8>;

    #[inline]
    fn read_value<R: Reader>(reader: &mut R) -> Result<Self::Type, <R as Reader>::Error> {
        reader.read_octet_string::<C>()
    }
}
