#[macro_use]
extern crate approx;
extern crate rand;
extern crate num;

pub mod gate;
pub mod alg;
pub mod types;

use std::f32;
use rand::Rng;
use num::complex::Complex;

use types::Qubit;
use types::Circuit;
use types::Entangle;
use gate::CGate;

/// Measure a qubit.
/// Change its state after the measure has occured.
fn measure(qubit: &mut Qubit) -> u32 {
    let mut rng = rand::thread_rng();
    let rn = rng.gen::<f32>();

    let alpha_sq = (qubit.state.0.conj() * qubit.state.0).re;

    if rn < alpha_sq {
	qubit.state = (Complex::new(1.0, 0.0), Complex::new(0.0, 0.0));
	return 0;
    }
    qubit.state = (Complex::new(0.0, 0.0), Complex::new(1.0, 0.0));
    return 1;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapes_of_state() {
	use crate::gate;

	let mut qubit = Qubit::default();

	gate::h(&mut qubit);

	let want = measure(&mut qubit);

	for _n in 0..100 {
	    let got = measure(&mut qubit);
	    assert_eq!(got, want);
	}
    }
}

