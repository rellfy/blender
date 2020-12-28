use futures::{ready, TryFuture};
use std::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll,
    }
};
use super::{
    BlenderBase,
    Blender,
    Internal,
};
use crate::{
    combine::{
        Combine,
        CombinedTuples,
        Tuple,
    },
    rejection::{
        Rejection,
        CombineRejection,
        IsReject
    },
    Func,
};
use pin_project::pin_project;

#[derive(Clone, Copy, Debug)]
pub struct TryBlend<A, F> {
    pub(super) blender: A,
    pub(super) callback: F,
}

impl<A, F> BlenderBase for TryBlend<A, F>
    where
        A: Blender,
        F: Func<A::Extract> + Clone + Send,
        F::Output: TryFuture + Send,
        <F::Output as TryFuture>::Error: CombineRejection<A::Error>,
{
    type Extract = (<F::Output as TryFuture>::Ok,);
    type Error = <<F::Output as TryFuture>::Error as CombineRejection<A::Error>>::One;
    type Future = TryBlendFuture<A, F>;

    #[inline]
    fn blend(&self, _: Internal) -> Self::Future {
        TryBlendFuture {
            state: State::First(self.blender.blend(Internal), self.callback.clone()),
        }
    }
}

#[allow(missing_debug_implementations)]
#[pin_project]
pub struct TryBlendFuture<T: Blender, F>
    where
        T: Blender,
        F: Func<T::Extract>,
        F::Output: TryFuture + Send,
        <F::Output as TryFuture>::Error: CombineRejection<T::Error>,
{
    #[pin]
    state: State<T::Future, F>,
}

#[pin_project(project = StateProj)]
enum State<T, F>
    where
        T: TryFuture,
        F: Func<T::Ok>,
        F::Output: TryFuture + Send,
        <F::Output as TryFuture>::Error: CombineRejection<T::Error>,
{
    First(#[pin] T, F),
    Second(#[pin] F::Output),
    Done,
}

impl<T, F> Future for TryBlendFuture<T, F>
    where
        T: Blender,
        F: Func<T::Extract>,
        F::Output: TryFuture + Send,
        <F::Output as TryFuture>::Error: CombineRejection<T::Error>,
{
    type Output = Result<
        (<F::Output as TryFuture>::Ok,),
        <<F::Output as TryFuture>::Error as CombineRejection<T::Error>>::One,
    >;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.project().state.poll(cx)
    }
}

impl<T, F> Future for State<T, F>
    where
        T: TryFuture,
        F: Func<T::Ok>,
        F::Output: TryFuture + Send,
        <F::Output as TryFuture>::Error: CombineRejection<T::Error>,
{
    type Output = Result<
        (<F::Output as TryFuture>::Ok,),
        <<F::Output as TryFuture>::Error as CombineRejection<T::Error>>::One,
    >;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {
            match self.as_mut().project() {
                StateProj::First(first, second) => {
                    let ex1 = ready!(first.try_poll(cx))?;
                    let fut2 = second.call(ex1);
                    self.set(State::Second(fut2));
                }
                StateProj::Second(second) => {
                    let ex3 = match ready!(second.try_poll(cx)) {
                        Ok(item) => Ok((item,)),
                        Err(err) => Err(From::from(err)),
                    };
                    self.set(State::Done);
                    return Poll::Ready(ex3);
                }
                StateProj::Done => panic!("polled after complete"),
            }
        }
    }
}