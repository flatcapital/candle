use crate::{DType, Device, Shape};

/// Main library error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unexpected dtype, expected: {expected:?}, got: {got:?}")]
    UnexpectedDType { expected: DType, got: DType },

    #[error("shape mismatch in {op}, lhs: {lhs:?}, rhs: {rhs:?}")]
    ShapeMismatchBinaryOp {
        lhs: Shape,
        rhs: Shape,
        op: &'static str,
    },

    #[error("device mismatch in {op}, lhs: {lhs:?}, rhs: {rhs:?}")]
    DeviceMismatchBinaryOp {
        lhs: Device,
        rhs: Device,
        op: &'static str,
    },

    #[error("dtype mismatch in {op}, lhs: {lhs:?}, rhs: {rhs:?}")]
    DTypeMismatchBinaryOp {
        lhs: DType,
        rhs: DType,
        op: &'static str,
    },

    #[error("unexpected rank, expected: {expected}, got: {got} ({shape:?})")]
    UnexpectedNumberOfDims {
        expected: usize,
        got: usize,
        shape: Shape,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
