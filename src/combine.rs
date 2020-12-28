use crate::Func;

#[derive(Debug)]
pub struct Product<H, A: HList>(pub(crate) H, pub(crate) A);

pub trait HList: Sized {
    type Tuple: Tuple<HList = Self>;

    fn flatten(self) -> Self::Tuple;
}

pub trait Tuple: Sized {
    type HList: HList<Tuple = Self>;

    fn hlist(self) -> Self::HList;

    #[inline]
    fn combine<A>(self, other: A) -> CombinedTuples<Self, A>
    where
        Self: Sized,
        A: Tuple,
        Self::HList: Combine<A::HList>,
    {
        self.hlist().combine(other.hlist()).flatten()
    }
}

pub type CombinedTuples<A, B> =
    <<<A as Tuple>::HList as Combine<<B as Tuple>::HList>>::Output as HList>::Tuple;

pub trait Combine<A: HList> {
    type Output: HList;

    fn combine(self, other: A) -> Self::Output;
}

impl<A: HList> Combine<A> for () {
    type Output = A;

    #[inline]
    fn combine(self, other: A) -> Self::Output {
        other
    }
}

impl<H, A: HList, B: HList> Combine<B> for Product<H, A>
where
    A: Combine<B>,
    Product<H, <A as Combine<B>>::Output>: HList,
{
    type Output = Product<H, <A as Combine<B>>::Output>;

    #[inline]
    fn combine(self, other: B) -> Self::Output {
        Product(self.0, self.1.combine(other))
    }
}

impl HList for () {
    type Tuple = ();

    #[inline]
    fn flatten(self) -> Self::Tuple {
        ()
    }
}

impl Tuple for () {
    type HList = ();

    #[inline]
    fn hlist(self) -> Self::HList {
        ()
    }
}



macro_rules! product {
    ($H:expr) => { Product($H, ()) };
    ($H:expr, $($T:expr),*) => { Product($H, product!($($T),*)) };
}

macro_rules! Product {
    ($H:ty) => { Product<$H, ()> };
    ($H:ty, $($T:ty),*) => { Product<$H, Product!($($T),*)> };
}

macro_rules! product_pat {
    ($H:pat) => { Product($H, ()) };
    ($H:pat, $($T:pat),*) => { Product($H, product_pat!($($T),*)) };
}

macro_rules! generics {
    ($type:ident) => {
        impl<$type> HList for Product!($type) {
            type Tuple = ($type,);

            #[inline]
            fn flatten(self) -> Self::Tuple {
                (self.0,)
            }
        }

        impl<$type> Tuple for ($type,) {
            type HList = Product!($type);
            #[inline]
            fn hlist(self) -> Self::HList {
                product!(self.0)
            }
        }

        impl<F, R, $type> Func<Product!($type)> for F
        where
            F: Fn($type) -> R,
        {
            type Output = R;

            #[inline]
            fn call(&self, args: Product!($type)) -> Self::Output {
                (*self)(args.0)
            }

        }

        impl<F, R, $type> Func<($type,)> for F
        where
            F: Fn($type) -> R,
        {
            type Output = R;

            #[inline]
            fn call(&self, args: ($type,)) -> Self::Output {
                (*self)(args.0)
            }
        }

    };

    ($type1:ident, $( $type:ident ),*) => {
        generics!($( $type ),*);

        impl<$type1, $( $type ),*> HList for Product!($type1, $($type),*) {
            type Tuple = ($type1, $( $type ),*);

            #[inline]
            fn flatten(self) -> Self::Tuple {
                #[allow(non_snake_case)]
                let product_pat!($type1, $( $type ),*) = self;
                ($type1, $( $type ),*)
            }
        }

        impl<$type1, $( $type ),*> Tuple for ($type1, $($type),*) {
            type HList = Product!($type1, $( $type ),*);

            #[inline]
            fn hlist(self) -> Self::HList {
                #[allow(non_snake_case)]
                let ($type1, $( $type ),*) = self;
                product!($type1, $( $type ),*)
            }
        }

        impl<F, R, $type1, $( $type ),*> Func<Product!($type1, $($type),*)> for F
        where
            F: Fn($type1, $( $type ),*) -> R,
        {
            type Output = R;

            #[inline]
            fn call(&self, args: Product!($type1, $($type),*)) -> Self::Output {
                #[allow(non_snake_case)]
                let product_pat!($type1, $( $type ),*) = args;
                (*self)($type1, $( $type ),*)
            }
        }

        impl<F, R, $type1, $( $type ),*> Func<($type1, $($type),*)> for F
        where
            F: Fn($type1, $( $type ),*) -> R,
        {
            type Output = R;

            #[inline]
            fn call(&self, args: ($type1, $($type),*)) -> Self::Output {
                #[allow(non_snake_case)]
                let ($type1, $( $type ),*) = args;
                (*self)($type1, $( $type ),*)
            }
        }
    };
}

generics! {
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16
}