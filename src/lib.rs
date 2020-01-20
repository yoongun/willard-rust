extern crate rand;
extern crate num;

use std::f32;
use rand::Rng;
use num::complex::Complex;

pub mod gate;

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

/// Set of quantum algorithms implemented in the willard
pub mod alg {
    use crate::*;

    /// # Quantum Random Number generator
    ///
    /// Returns a single bit with a value 0 or 1 with
    /// 50% of possibility each.
    pub fn qrn() -> u32 {
	let mut qubit = Qubit::default();
	gate::h(&mut qubit);
	return measure(&mut qubit);
    }

    /// # Quantum Key Distribution algorithm
    ///
    /// 
    pub fn qkd(spy: bool, use_had: bool) -> bool {
	let alice = Qubit::default();
	let conn = Qubit::default();
	let bob = Qubit::default();

	// ### Start Alice part ###
	let a1 = qrn();
	let a2 = qrn();

	if (a1 == 1) {
	    gate::x(&mut alice);
	}
	if (a2 == 1) {
	    gate::had(&mut alice);
	}
	// ### End Alice part #####

	// ### Sends the qubit via fiber ###
	gate::swap(&mut conn, &mut alice);

	// ### Sniffing starts ###
	if (spy == true) {
	    if (use_had == true) {
		gate::h(&mut conn);
	    }

	    let stolen = measure(&mut conn);
	    conn = c2q(stolen);

	    if (use_had == true) {
		gate::h(&mut conn);
	    }
	}
	// ### Sniffing ends #####

	// ### Bob receives the qubit ###
	let a3 = qrn();
	gate::swap(&mut bob, &mut conn);

	if (a3 == 1) {
	    gate::h(&mut bob);
	}
	let recv = measure(&mut bob);
	// ### Bob part ends ############

	// ### Checks whether the spy sniffed the qubit ###
	if (a2 == a3) {
	    if (a1 != recv) {
		return true;
	    }
	}
	// ### Sniffing check end #########################

	return false;
    }
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

