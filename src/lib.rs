mod willard {
    struct Qubit{
    }

    impl Default for Qubit {
	fn default -> Qubit {
	    Qubit{state: [1.0, 0.0], phase: 0.0}
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
    }
}
