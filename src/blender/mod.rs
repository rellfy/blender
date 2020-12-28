pub mod and;
pub mod try_blend;

use futures::TryFuture;
use std::{
    future::Future
};
use and::And;
use crate::{
    Func,
    rejection::{
        Rejection,
        CombineRejection,
        IsReject,
    },
    combine::{
        Combine,
        CombinedTuples,
        Tuple,
    }
};
use crate::blender::try_blend::TryBlend;

pub trait BlenderBase {
    type Extract: Tuple;
    type Error: IsReject;
    type Future: Future<Output = Result<Self::Extract, Self::Error>> + Send;

    fn blend(&self, internal: Internal) -> Self::Future;
}

#[allow(missing_debug_implementations)]
pub struct Internal;

pub trait Blender: BlenderBase {
    fn and<B>(self, other: B) -> And<Self, B>
    where
        Self: Sized,
        <Self::Extract as Tuple>::HList: Combine<<B::Extract as Tuple>::HList>,
        B: Blender + Clone,
        B::Error: CombineRejection<Self::Error>,
    {
        And {
            first: self,
            second: other,
        }
    }

    // fn or<B>(self, other: B) -> Or<Self, B>
    // where
    //     Self: Blender<Error = Rejection> + Sized,
    //     B: Blender,
    //     B::Error: CombineRejection<Self::Error>,
    // {
    //     Or {
    //         first: self,
    //         second: other,
    //     }
    // }

    fn try_blend<F>(self, blender: F) -> TryBlend<Self, F>
    where
        Self: Sized,
        F: Func<Self::Extract> + Clone,
        F::Output: TryFuture + Send,
        <F::Output as TryFuture>::Error: CombineRejection<Self::Error>,
    {
        TryBlend {
            blender: self,
            callback: blender,
        }
    }
}

impl<T: BlenderBase> Blender for T { }

pub trait BlenderClone: Blender + Clone {}

impl<T: Blender + Clone> BlenderClone for T {}