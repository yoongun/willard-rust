use num::complex::Complex;


/// # Data struct of the qubit
///
/// Contains the complex value of the alpha and beta
/// to express the superposition and the phase of the qubit
pub struct Qubit{
    pub state: (Complex<f32>, Complex<f32>),
}

impl Default for Qubit {
    /// Define the default states of the qubit as |0>
    fn default() -> Qubit {
	Qubit{state: (Complex::new(1.0, 0.0), Complex::new(0.0, 0.0))}
    }
}

pub struct Qucrumb {
    bits: [Qubit; 2],
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

}
