extern crate rand;


pub mod willard {
    use std::f32;
    use rand::Rng;

    pub struct Qubit{
	state: [f32; 2],
    }

    impl Default for Qubit {
	fn default() -> Qubit {
	    Qubit{state: [1.0, 0.0]}
	}
    }

    impl Qubit {
	fn measure() {
	}
    }

    pub mod gate {
	use crate::willard;

	pub fn not(qubit: &mut willard::Qubit) {
	    let state = qubit.state;
	    qubit.state = [state[1], state[0]];
	}

	pub fn h(qubit: &mut willard::Qubit) {
	    let root_two = (2.0 as f32).sqrt();
	    let mat = [[1.0 / root_two, 1.0 / root_two],
		       [1.0 / root_two, -1.0 / root_two]];
	    let mut state: [f32; 2] = [0.0, 0.0];

	    state[0] += mat[0][0] * qubit.state[0] + mat[0][1] * qubit.state[1];
	    state[1] += mat[1][0] * qubit.state[0] + mat[1][1] * qubit.state[1];

	    qubit.state = state;
	}
    }

    pub mod alg {
	use crate::willard;

	pub fn qrn() -> u32 {
	    let qubit = willard::Qubit::default();
	    willard::gate::h(qubit);
	    return Qubit::measure(&mut qubit);
	}
    }

    #[cfg(test)]
    mod tests {
	use super::*;
	
	#[test]
	fn test_qubit_init() {
	    let qubit = Qubit::default();

	    assert_eq!(qubit.state, [1.0, 0.0]);
	}

	#[test]
	fn test_not() {
	    let mut qubit = Qubit::default();

	    gate::not(&mut qubit);

	    assert_eq!(qubit.state, [0.0, 1.0]);
	}

	#[test]
	fn test_h() {
	    // Test on a qubit of state [1.0, 0.0]
	    let mut qubit0 = Qubit::default();

	    gate::h(&mut qubit0);

	    let root_two = (2.0 as f32).sqrt();

	    let got_state = qubit0.state;
	    let want_state = [1.0 / root_two, 1.0 / root_two];

	    assert_eq!(got_state, want_state);

	    // Test on a qubit of state [0.0, 1.0]
	    let mut qubit1 = Qubit::default();
	    gate::not(&mut qubit1);

	    gate::h(&mut qubit1);

	    let got_state = qubit1.state;
	    let want_state = [1.0 / root_two, -1.0 / root_two];

	    assert_eq!(got_state, want_state);
	}

	#[test]
	fn test_collapes_of_state() {
	    let mut qubit = Qubit::default();

	    gate::h(&mut qubit);

	    let want = Qubit::measure(&mut qubit);

	    for _n in 0..100 {
		let got = Qubit::measure(&mut qubit);
		assert_eq!(got, want);
	    }
	}

    }
}
