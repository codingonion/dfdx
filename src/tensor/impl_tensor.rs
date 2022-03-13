use super::structs::*;
use super::traits::*;
use crate::diff_fns::*;
use crate::gradients::GradientTape;
use ndarray::{Array, Ix0, Ix1, Ix2, Ix3, Ix4};
use rand::prelude::{Distribution, Rng};
use rand_distr::{Standard, StandardNormal};
use std::cell::RefCell;
use std::ops::SubAssign;

macro_rules! tensor_impl {
    ($typename:ident, [$($const_names:tt),*], $dim:ty, $shape:ty) => {
        impl<$(const $const_names: usize),*> IsShapedArray for $typename<$($const_names),*> {
            type Dimension = $dim;
            type Shape = $shape;
            const SHAPE: Self::Shape = tupleify!($($const_names),*);
            const NUM_ELEMENTS: usize = prod!($($const_names),*);

            fn data(&self) -> &Array<f32, Self::Dimension> { &self.data }
            fn mut_data(&mut self) -> &mut Array<f32, Self::Dimension> { &mut self.data }
        }

        impl<$(const $const_names: usize),*> CanStoreGradientTape for $typename<$($const_names),*> {
            fn tape(&self) -> &RefCell<Option<Box<GradientTape>>> { &self.tape }
        }

        impl<$(const $const_names: usize),*> HasGradients for $typename<$($const_names),*> {
            fn update_with_gradients(&mut self, tape: &GradientTape) {
                let gradient = tape.gradient_for(self.id);
                self.mut_data().sub_assign(gradient);
            }
        }

        impl<$(const $const_names: usize),*> Randomize for $typename<$($const_names),*> {
            fn randomize<R: Rng, D: Distribution<f32>>(&mut self, rng: &mut R, dist: &D) {
                self.mut_data().map_inplace(|f| *f = dist.sample(rng))
            }
        }

        impl<$(const $const_names: usize),*> HasUniqueId for $typename<$($const_names),*> {
            fn id(&self) -> usize {
                self.id
            }
        }
    }
}

macro_rules! prod {
    () => {
        1
    };
    ($head:ident) => {
        $head
    };
    ($head:ident, $($tail:ident),+) => {
        $head * prod!($($tail),+)
    };
}

macro_rules! tupleify {
    () => {
        ()
    };
    ($elem:tt) => {
        ($elem,)
    };
    ($($elems:tt),+) => {
        ($($elems),*)
    };
}

tensor_impl!(Tensor0D, [], Ix0, ());
tensor_impl!(Tensor1D, [M], Ix1, (usize,));
tensor_impl!(Tensor2D, [M, N], Ix2, (usize, usize));
tensor_impl!(Tensor3D, [M, N, O], Ix3, (usize, usize, usize));
tensor_impl!(Tensor4D, [M, N, O, P], Ix4, (usize, usize, usize, usize));

impl<T: Tensor> TensorSugar for T {
    fn zeros() -> Self {
        Self::new(Array::zeros(Self::SHAPE))
    }

    fn ones() -> Self {
        Self::new(Array::ones(Self::SHAPE))
    }

    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut data = Array::zeros(Self::SHAPE);
        data.map_inplace(|f| *f = Standard.sample(rng));
        Self::new(data)
    }

    fn randn<R: Rng>(rng: &mut R) -> Self {
        let mut data = Array::zeros(Self::SHAPE);
        data.map_inplace(|f| *f = StandardNormal.sample(rng));
        Self::new(data)
    }

    fn relu(&self) -> Self {
        self.apply::<ReLU>()
    }

    fn sin(&self) -> Self {
        self.apply::<Sin>()
    }

    fn cos(&self) -> Self {
        self.apply::<Cos>()
    }

    fn ln(&self) -> Self {
        self.apply::<Ln>()
    }

    fn exp(&self) -> Self {
        self.apply::<Exp>()
    }

    fn sigmoid(&self) -> Self {
        self.apply::<Sigmoid>()
    }

    fn tanh(&self) -> Self {
        self.apply::<Tanh>()
    }

    fn square(&self) -> Self {
        self.apply::<Square>()
    }

    fn abs(&self) -> Self {
        self.apply::<Abs>()
    }
}
