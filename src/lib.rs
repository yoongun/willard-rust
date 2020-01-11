pub mod willard {
    use std::f32;

    pub struct Qubit{
	state: [f32; 2],
    }

    impl Default for Qubit {
	fn default() -> Qubit {
	    Qubit{state: [1.0, 0.0]}
	}
    }

    pub mod gate {
	use crate::willard::Qubit;

	pub fn not(qubit: &mut Qubit) {
	    let state = qubit.state;
	    qubit.state = [state[1], state[0]];
	}

	pub fn h(qubit: &mut Qubit) {
	    let root_two = (2.0 as f32).sqrt();
	    let mat = [[1.0 / root_two, 1.0 / root_two],
		       [1.0 / root_two, -1.0 / root_two]];
	    let mut state: [f32; 2] = [0.0, 0.0];

	    state[0] += mat[0][0] * qubit.state[0] + mat[0][1] * qubit.state[1];
	    state[1] += mat[1][0] * qubit.state[0] + mat[1][1] * qubit.state[1];

	    qubit.state = state;
	}
    }

    #[cfg(test)]
    mod tests {
	use super::*;
	
	#[test]
	fn test_qubit() {
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
    }
}
