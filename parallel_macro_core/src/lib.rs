#![feature(try_trait_v2)]
use std::ops::{ControlFlow, FromResidual, Try};
use std::convert::Infallible;

pub enum TimeoutResult<T, E> {
    Success(T),
    Error(E),
    TimedOut,
}

// Define the error type for the residual
pub enum TimeoutResultError<E> {
    Error(E),
    TimedOut,
}

// Implement Try
impl<T, E> Try for TimeoutResult<T, E> {
    type Output = T;
    type Residual = Result<Infallible, TimeoutResultError<E>>;

    fn from_output(output: Self::Output) -> Self {
        TimeoutResult::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            TimeoutResult::Success(v) => ControlFlow::Continue(v),
            TimeoutResult::Error(e) => ControlFlow::Break(Err(TimeoutResultError::Error(e))),
            TimeoutResult::TimedOut => ControlFlow::Break(Err(TimeoutResultError::TimedOut)),
        }
    }
}

// Implement FromResidual
impl<T, E> FromResidual<Result<Infallible, TimeoutResultError<E>>> for TimeoutResult<T, E> {
    fn from_residual(residual: Result<Infallible, TimeoutResultError<E>>) -> Self {
        match residual {
            Err(TimeoutResultError::Error(e)) => TimeoutResult::Error(e),
            Err(TimeoutResultError::TimedOut) => TimeoutResult::TimedOut,
            Ok(infallible) => match infallible {},
        }
    }
}