//! A crate containing traits and functions for iterators those iterator steps may fail

/// An iterator that only iterates over the first `n` iterations of `iter`.
///
/// This `struct` is created by the `take()` method on `Iterrator`
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Take<I>{
    iter: I,
    n: usize,
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Map<I,F>{
    iter: I,
    f: F,
}

/// An iterator which may or may not succeed to advance to its next element
pub trait Iterrator{
    type Item;
    type Error;

    /// Advances the iterator and returns the next value
    ///
    /// If advancing fails it will return `Err(Error)`. Returns `Ok(None)` when iteration is
    /// finished, otherwise `Ok(Some(Item))` is returned.
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>;

    /// An iterator adaptor that applies a function, producing a single, final value.
    fn fold<B, F>(mut self, init:B, mut f: F) -> Result<B, Self::Error> where
        Self: Sized, F: FnMut(B, Self::Item) -> B
    {
        let mut accum = init;
        while let Some(x) = self.next()?{
            accum = f(accum, x);
        }
        Ok(accum)
    }

    /// Takes a closure and creates an iterator which calls that closure on each element.
    fn map<F>(self, f: F) -> Map<Self, F> where
        Self: Sized
    {
        Map{iter: self, f}
    }

    /// Creates an iterator that yields its first n elements.
    fn take(self, n: usize) -> Take<Self> where
        Self: Sized
    {
        Take{iter: self, n}
    }
}

impl<I> Iterrator for Take<I> where I: Iterrator{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        if self.n != 0 {
            self.n -= 1;
            self.iter.next()
        } else {
            Ok(None)
        }
    }
}

impl<B, I, F> Iterrator for Map<I,F> where
    I: Iterrator,
    F: FnMut(I::Item) -> B
{
    type Item = B;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<B>, Self::Error> {
        Ok(self.iter.next()?.map(&mut self.f))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct FailIterator;
    impl Iterrator for FailIterator{
        type Item = usize;
        type Error = ();

        fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>{
            Err(())
        }
    }

    struct NumbersIterator(usize);
    impl Iterrator for NumbersIterator{
        type Item = usize;
        type Error = ();

        fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>{
            let n = &mut self.0;
            *n += 1;
            Ok(Some(*n))
        }
    }

    #[test]
    fn fold_fail() {

        let it = FailIterator;
        assert_eq!(it.fold(0, |a,b| a + b), Err(()));
    }

    #[test]
    fn sum() {

        let it = NumbersIterator(0);
        assert_eq!(it.take(5).fold(0, |a,b| a + b), Ok(15));
    }
}
