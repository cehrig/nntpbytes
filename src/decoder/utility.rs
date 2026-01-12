use std::cmp::Ordering;

pub(crate) struct PositionWithLength(usize, usize);

impl PositionWithLength {
    pub(crate) fn new(p: usize, l: usize) -> Self {
        Self(p, l)
    }

    pub(crate) fn position(&self) -> usize {
        self.0
    }

    pub(crate) fn length(&self) -> usize {
        self.1
    }
}

impl PartialEq for PositionWithLength {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for PositionWithLength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for PositionWithLength {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Eq for PositionWithLength {}

pub(crate) trait Pipe {
    fn pipe<F, U>(self, cb: F) -> U
    where
        F: Fn(Self) -> U,
        Self: Sized;
}

impl<T> Pipe for T {
    fn pipe<F, U>(self, cb: F) -> U
    where
        F: Fn(T) -> U,
    {
        cb(self)
    }
}
