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
    state: [Complex<f32>; 4]
}
    
impl Qucrumb {
    fn new(qubit1: Qubit, qubit2: Qubit) -> Qucrumb {
	let mut state: [Complex<f32>; 4] = [
	    Complex::new(1.0, 0.0),
	    Complex::new(0.0, 0.0),
	    Complex::new(0.0, 0.0),
	    Complex::new(0.0, 0.0),
	];

	let bit1 = [qubit1.state.0, qubit1.state.1];
	let bit2 = [qubit2.state.0, qubit2.state.1];

	for n in 0..3 {
	    let bit_list = [
		(n & 1),
		(n & 2)
	    ];
	    state[n] = bit1[bit_list[0]] * bit2[bit_list[1]];
	}
	return Qucrumb{state: state};
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

