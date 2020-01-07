pub mod willard {
    pub struct Qubit{
	state: [f32; 2],
	phase: f32
    }

    impl Default for Qubit {
	fn default() -> Qubit {
	    Qubit{state: [1.0, 0.0], phase: 0.0}
	}
    }

    pub mod Gate {
	use crate::willard::Qubit;

	pub fn not(qubit: &mut Qubit) {
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

	    Gate::not(&qubit);

	    assert_eq!(qubit.state, [0.0, 1.0]);
	    assert_eq!(qubit.phase, 0.0);
	}
    }
}
