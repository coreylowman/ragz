use crate::envs::Env;
use tch::{nn::VarStore, Tensor};

pub trait Policy<E: Env<N>, const N: usize> {
    fn eval(&mut self, state: &E::State) -> ([f32; N], f32);
}

pub trait NNPolicy<E: Env<N>, const N: usize> {
    fn new(vs: &VarStore) -> Self;
    fn forward(&self, xs: &Tensor) -> (Tensor, Tensor);
}
