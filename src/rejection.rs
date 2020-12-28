use std::{
    fmt,
    any::Any,
    convert::Infallible,
    error::Error as StdError,
};

pub trait Reject: fmt::Debug + Sized + Send + Sync + 'static {}

#[inline]
pub fn reject() -> Rejection {
    Rejection::new()
}

#[derive(Debug)]
pub struct Rejection {
    reason: Option<String>
}

impl Rejection {
    pub fn new() -> Rejection {
        Rejection {
            reason: None,
        }
    }

    pub fn with_reason(reason: String) -> Rejection {
        Rejection {
           reason: Some(reason),
        }
    }
}

// CombinedRejection
pub(crate) use self::sealed::{CombineRejection, IsReject};

mod sealed {
    use super::{Rejection};
    use std::convert::Infallible;
    use std::fmt;

    pub trait IsReject: fmt::Debug + Send + Sync {
        fn status(&self) -> i32;
    }

    fn _assert_object_safe() {
        fn _assert(_: &dyn IsReject) {}
    }

    pub trait CombineRejection<E>: Send + Sized {
        type One: IsReject + From<Self> + From<E> + Into<Rejection>;
        type Combined: IsReject;

        fn combine(self, other: E) -> Self::Combined;
    }

    impl CombineRejection<Rejection> for Rejection {
        type One = Rejection;
        type Combined = Rejection;

        fn combine(self, other: Rejection) -> Self::Combined {
            let reason = match (self.reason, other.reason) {
                (Some(left), Some(right)) => {
                    Some(format!("{}, {}", left, right))
                }
                (Some(other), None) | (None, Some(other)) => {
                    Some(other)
                }
                (None, None) => None,
            };

            Rejection {
                reason
            }
        }
    }

    impl CombineRejection<Infallible> for Rejection {
        type One = Rejection;
        type Combined = Infallible;

        fn combine(self, other: Infallible) -> Self::Combined {
            match other {}
        }
    }

    impl CombineRejection<Rejection> for Infallible {
        type One = Rejection;
        type Combined = Infallible;

        fn combine(self, _: Rejection) -> Self::Combined {
            match self {}
        }
    }

    impl CombineRejection<Infallible> for Infallible {
        type One = Infallible;
        type Combined = Infallible;

        fn combine(self, _: Infallible) -> Self::Combined {
            match self {}
        }
    }
}

// impl Traits
impl From<Infallible> for Rejection {
    #[inline]
    fn from(infallible: Infallible) -> Rejection {
        match infallible {}
    }
}

impl IsReject for Infallible {
    fn status(&self) -> i32 {
        0
    }
}

impl IsReject for Rejection {
    fn status(&self) -> i32 {
        0
    }
}