use futures::{
    ready,
};
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
        IsReject,
    },
};
use pin_project::pin_project;

#[derive(Copy, Clone, Debug)]
pub struct And<A, B> {
    pub(super) first: A,
    pub(super) second: B,
}

impl<A, B> BlenderBase for And<A, B>
where
    A: Blender,
    A::Extract: Send,
    B: Blender + Clone + Send,
    <A::Extract as Tuple>::HList: Combine<<B::Extract as Tuple>::HList> + Send,
    CombinedTuples<A::Extract, B::Extract>: Send,
    B::Error: CombineRejection<A::Error>,
{
    type Extract = CombinedTuples<A::Extract, B::Extract>;
    type Error = <B::Error as CombineRejection<A::Error>>::One;
    type Future = AndFuture<A, B>;

    fn blend(&self, _: Internal) -> Self::Future {
        AndFuture {
            state: State::First(self.first.blend(Internal), self.second.clone()),
        }
    }
}

#[allow(missing_debug_implementations)]
#[pin_project]
pub struct AndFuture<T: Blender, U: Blender> {
    #[pin]
    state: State<T::Future, T::Extract, U>,
}

#[pin_project(project = StateProj)]
enum State<T, TE, U: Blender> {
    First(#[pin] T, U),
    Second(Option<TE>, #[pin] U::Future),
    Done,
}

impl<T, U> Future for AndFuture<T, U>
    where
        T: Blender,
        U: Blender,
        <T::Extract as Tuple>::HList: Combine<<U::Extract as Tuple>::HList> + Send,
        U::Error: CombineRejection<T::Error>,
{
    type Output = Result<
        CombinedTuples<T::Extract, U::Extract>,
        <U::Error as CombineRejection<T::Error>>::One,
    >;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.project().state.poll(cx)
    }
}

impl<T, TE, U, E> Future for State<T, TE, U>
    where
        T: Future<Output = Result<TE, E>>,
        U: Blender,
        TE: Tuple,
        TE::HList: Combine<<U::Extract as Tuple>::HList> + Send,
        U::Error: CombineRejection<E>,
{
    type Output = Result<CombinedTuples<TE, U::Extract>, <U::Error as CombineRejection<E>>::One>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {
            match self.as_mut().project() {
                StateProj::First(first, second) => {
                    let ex1 = ready!(first.poll(cx))?;
                    let fut2 = second.blend(Internal);
                    self.set(State::Second(Some(ex1), fut2));
                }
                StateProj::Second(ex1, second) => {
                    let ex2 = ready!(second.poll(cx))?;
                    let ex3 = ex1.take().unwrap().combine(ex2);
                    self.set(State::Done);
                    return Poll::Ready(Ok(ex3));
                }
                StateProj::Done => panic!("polled after complete"),
            }
        }
    }
}