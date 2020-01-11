extern crate rand;
extern crate num;

use std::f32;
use rand::Rng;
use num::complex::Complex;

pub mod gate;

pub struct Qubit{
    state: (f32, Complex<f32>),
}

impl Default for Qubit {
    fn default() -> Qubit {
	Qubit{state: (1.0, Complex::new(0.0, 0.0))}
    }
}

fn measure(qubit: &mut Qubit) -> u32 {
    let mut rng = rand::thread_rng();
    let rn = rng.gen::<f32>();

    if rn < qubit.state.0.powf(2.0) {
	qubit.state = (1.0, Complex::new(0.0, 0.0));
	return 0;
    }
    qubit.state = (0.0, Complex::new(1.0, 0.0));
    return 1;
}

pub mod alg {
    use crate::*;

    pub fn qrn() -> u32 {
	let mut qubit = Qubit::default();
	gate::h(&mut qubit);
	return measure(&mut qubit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex;

    #[test]
    fn test_qubit_init() {
	let qubit = Qubit::default();

	assert_eq!(qubit.state, (1.0, Complex{re: 0.0, im: 0.0}));
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

