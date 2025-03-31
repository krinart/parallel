pub enum TimeoutResult<T, E> {
    Success(T),
    Error(E),
    TimedOut,
}