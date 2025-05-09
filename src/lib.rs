#![feature(slice_as_array)]

use eyre::Result;
use statrs::{
	distribution::{Binomial, Continuous, Discrete, Normal},
	function::beta::{checked_beta, checked_beta_reg},
};
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

#[cfg(target_endian = "big")]
compile_error!("adfa");

initiate_protocol!();

macro_rules! p {
	($type:ty, $slice:expr) => {
		<$type>::from_le_bytes(*$slice.as_array().ok_or(::eyre::eyre!(concat!(
			"expected valid ",
			::std::stringify!($type),
			" for ",
			::std::stringify!($slice),
		)))?)
	};
}

macro_rules! ret {
	($res:expr) => {
		Ok($res.to_le_bytes().to_vec())
	};
}

#[wasm_func]
fn binomial(n: &[u8], k: &[u8], p: &[u8]) -> Result<Vec<u8>> {
	ret!(Binomial::new(p!(f64, p), p!(u64, n))?.pmf(p!(u64, k)))
}

#[wasm_func]
fn normal(mu: &[u8], sigma: &[u8], at: &[u8]) -> Result<Vec<u8>> {
	ret!(Normal::new(p!(f64, mu), p!(f64, sigma))?.pdf(p!(f64, at)))
}

#[wasm_func]
fn beta_reg(a: &[u8], b: &[u8], x: &[u8]) -> Result<Vec<u8>> {
	ret!(checked_beta_reg(p!(f64, a), p!(f64, b), p!(f64, x))?)
}

#[wasm_func]
fn beta(a: &[u8], b: &[u8]) -> Result<Vec<u8>> {
	ret!(checked_beta(p!(f64, a), p!(f64, b))?)
}

#[wasm_func]
fn show_code() -> Vec<u8> {
	include_bytes!("lib.rs").to_vec()
}
