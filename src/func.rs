use crate::{
    rejection::Rejection
};

pub trait Func<Args> {
    type Output;

    fn call(&self, args: Args) -> Self::Output;
}

impl<F, R> Func<()> for F
    where
        F: Fn() -> R,
{
    type Output = R;

    #[inline]
    fn call(&self, _args: ()) -> Self::Output {
        (*self)()
    }
}

impl<F, R> Func<Rejection> for F
    where
        F: Fn(Rejection) -> R,
{
    type Output = R;

    #[inline]
    fn call(&self, arg: Rejection) -> Self::Output {
        (*self)(arg)
    }
}