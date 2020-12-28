use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::blender::{
    Blender,
    BlenderBase,
    Internal
};

pub fn any() -> impl Blender<Extract = (), Error = Infallible> + Copy {
    Any
}

#[derive(Copy, Clone)]
#[allow(missing_debug_implementations)]
struct Any;

impl BlenderBase for Any {
    type Extract = ();
    type Error = Infallible;
    type Future = AnyFut;

    #[inline]
    fn blend(&self, _: Internal) -> Self::Future {
        AnyFut
    }
}

#[allow(missing_debug_implementations)]
struct AnyFut;

impl Future for AnyFut {
    type Output = Result<(), Infallible>;

    #[inline]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(Ok(()))
    }
}