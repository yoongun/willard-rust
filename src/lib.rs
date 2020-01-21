extern crate rand;
extern crate num;

use std::f32;
use rand::Rng;
use num::complex::Complex;

pub mod gate;
pub mod alg;

/// # Data struct of the qubit
///
/// Contains the complex value of the alpha and beta
/// to express the superposition and the phase of the qubit
pub struct Qubit{
    state: (Complex<f32>, Complex<f32>),
}

impl Default for Qubit {
    /// Define the default states of the qubit as |0>
    fn default() -> Qubit {
	Qubit{state: (Complex::new(1.0, 0.0), Complex::new(0.0, 0.0))}
    }
}

pub struct Qucrumb {
    bits: [Qubit; 2],
}

impl Qucrumb {
    fn state(&self) -> [Complex<f32>; 4] {
	let mut ret: [Complex<f32>; 4];

	let bit1 = [self.bits[0].state.0, self.bits[0].state.1];
	let bit2 = [self.bits[1].state.0, self.bits[1].state.1];

	for n in 0..3 {
	    let bit_list = [
		n & 1,
		n & 2
	    ];
	    ret[n] = bit1[bit_list[0]] * bit2[bit_list[1]];
	}
	return ret;
    }
}

pub struct Qunibble {
    crumbs: [Qucrumb; 2],
}

pub struct Qubyte {
    nibbles: [Qunibble; 2],
}

impl Qubyte {
    fn state(&self) -> [Complex<f32>; 256] {
	let mut a: [Complex<f32>; 256];

	for n in 0..256 {
	    let bit_list = [
		n & 1,
		n & 2,
		n & 4,
		n & 8,
		n & 16,
		n & 32,
		n & 64,
		n & 128,
	    ];
	    
	}
    }
}

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
    use num::complex::Complex;

    #[test]
    fn test_qubit_init() {
	let qubit = Qubit::default();

	assert_eq!(qubit.state, (Complex{re: 1.0, im: 0.0}, Complex{re: 0.0, im: 0.0}));
    }

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

