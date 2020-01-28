use num::complex::Complex;
use std::vec::Vec;


/// # Data struct of the qubit
///
/// Contains the complex value of the alpha and beta
/// to express the superposition and the phase of the qubit
pub struct Qubit{
    pub state: (Complex<f32>, Complex<f32>),
}

pub struct Circuit{
    pub qubits: Vec<(Qubit, i32)>,
}

impl Default for Qubit {
    /// Define the default states of the qubit as |0>
    fn default() -> Qubit {
	Qubit{state: (Complex::new(1.0, 0.0), Complex::new(0.0, 0.0))}
    }
}

impl Default for Circuit {
    fn default() -> Circuit {
	Circuit{qubits: Vec::new()}
    }
}

impl Circuit {
    fn add(&mut self, qubit: Qubit, idx: i32) {
	self.qubits.push((qubit, idx));
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
    fn test_add_qubit_to_circuti() {
	let mut circ = Circuit::default();

	let qubit = Qubit::default();
	circ.add(qubit, 0);

	assert_eq!(circ.qubits.len(), 1);
	assert_eq!(circ.qubits[0].0.state,
		   (Complex{re: 1.0, im: 0.0},
		   Complex{re: 0.0, im: 0.0}));
	assert_eq!(circ.qubits[0].1, 0);
    }

}
