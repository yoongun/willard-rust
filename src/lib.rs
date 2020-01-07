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

	pub fn had(qubit: &mut Qubit) {
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
	fn test_had() {
	    // Test on a qubit of state [1.0, 0.0]
	    let mut qubit0 = Qubit::default();

	    gate::had(&mut qubit0);

	    assert_eq!(qubit0.state, [0.5, 0.5]);
	    assert_eq!(qubit0.phase, 0.0);

	    // Test on a qubit of state [0.0, 1.0]
	    let mut qubit1 = Qubit::default();
	    gate::not(&mut qubit);

	    gate::had(&mut qubit1);

	    assert_eq!(qubit1.state, [0.5, 0.5]);
	    assert_eq!(qubit1.phase, f32::consts::PI);
	}
    }
}
