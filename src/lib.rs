extern crate rand;
extern crate num;


use std::f32;
use rand::Rng;
use num::complex::Complex;

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

pub mod gate {
    use crate::Qubit;
    use num::complex::Complex;

    pub fn not(qubit: &mut Qubit) {
	let state = qubit.state;
	qubit.state = (state.1.re, Complex::new(state.0, -state.1.im));
    }

    pub fn h(qubit: &mut Qubit) {
	let root_two = (2.0 as f32).sqrt();
	let mat = [[1.0 / root_two, 1.0 / root_two],
		    [1.0 / root_two, -1.0 / root_two]];
	let mut state: (Complex<f32>, Complex<f32>)= (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));

	state.0 = mat[0][0] * qubit.state.0 + mat[0][1] * qubit.state.1;
	state.1 = mat[1][0] * qubit.state.0 + mat[1][1] * qubit.state.1;

	qubit.state = normalize_phase(state);
    }

    fn normalize_phase(state: (Complex<f32>, Complex<f32>)) -> (f32, Complex<f32>) {
	let size = (state.0.conj() * state.0).re.sqrt();
	let d_phase = (state.0 / size).conj();

	let l = (d_phase * state.0).re;
	let r = d_phase * state.1;
	(l, r)
    }

    #[cfg(test)]
    mod tests {
	use super::*;
	use num::complex::Complex;

	#[test]
	fn test_normalize_phase() {
	    let state = (Complex::new(0.0, -1.0), Complex::new(1.0, 0.0));
	    let got = normalize_phase(state);
	    let want = (1.0, Complex::new(0.0, 1.0));
	    assert_eq!(got, want);
	}

	#[test]
	fn test_not() {
	    let mut qubit = Qubit::default();

	    not(&mut qubit);

	    assert_eq!(qubit.state, (0.0, Complex{re: 1.0, im: 0.0}));
	}

	#[test]
	fn test_h() {
	    // Test on a qubit of state [1.0, 0.0]
	    let mut qubit0 = Qubit::default();

	    h(&mut qubit0);

	    let root_two = (2.0 as f32).sqrt();

	    let got = qubit0.state;
	    let want = (1.0 / root_two, Complex{re: 1.0 / root_two, im: 0.0});

	    assert_eq!(got, want);

	    // Test on a qubit of state [0.0, 1.0]
	    let mut qubit1 = Qubit::default();
	    not(&mut qubit1);

	    h(&mut qubit1);

	    let got = qubit1.state;
	    let want = (1.0 / root_two, Complex{re: -1.0 / root_two, im: 0.0});

	    assert_eq!(got, want);
	}
    }
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

