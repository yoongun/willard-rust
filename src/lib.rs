pub mod willard {
    use std::f32;

    pub struct Qubit{
	state: [f32; 2],
	phase: f32
    }

    impl Default for Qubit {
	fn default() -> Qubit {
	    Qubit{state: [1.0, 0.0], phase: 0.0}
	}
    }

    pub mod gate {
	use crate::willard::Qubit;

	pub fn not(qubit: &mut Qubit) {
	    let state = qubit.state;
	    qubit.state = [state[1], state[0]];
	}

	pub fn h(qubit: &mut Qubit) {
	    let two = 2.0_f32;
	    let t_sqrt = two.sqrt();
	    let mat = [[1.0 / t_sqrt, 1.0 / t_sqrt],
		       [1.0 / t_sqrt, -1.0 / t_sqrt]];
	    let mut state: [f32; 2] = [1.0, 0.0];
	    for row in mat.iter() {
		state[0] += row[0] * qubit.state[0];
		state[1] += row[1] * qubit.state[1];
	    }
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
	    assert_eq!(qubit.phase, 0.0);
	}

	#[test]
	fn test_not() {
	    let mut qubit = Qubit::default();

	    gate::not(&mut qubit);

	    assert_eq!(qubit.state, [0.0, 1.0]);
	    assert_eq!(qubit.phase, 0.0);
	}

	#[test]
	fn test_h() {
	    // Test on a qubit of state [1.0, 0.0]
	    let mut qubit0 = Qubit::default();

	    gate::h(&mut qubit0);

	    assert_eq!(qubit0.state, [0.5, 0.5]);
	    assert_eq!(qubit0.phase, 0.0);

	    // Test on a qubit of state [0.0, 1.0]
	    let mut qubit1 = Qubit::default();
	    gate::not(&mut qubit1);

	    gate::h(&mut qubit1);

	    assert_eq!(qubit1.state, [0.5, 0.5]);
	    assert_eq!(qubit1.phase, f32::consts::PI);
	}
    }
}
