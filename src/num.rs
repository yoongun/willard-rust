use crate::*;
use num::complex::Complex;


struct Qubit {
    pub state: (Complex<f32>, Complex<f32>),
}


impl Default for Qubit {
    fn default() -> Qubit {
	Qubit{state: (Complex::new(1.0, 0.0), Complex::new(0.0, 0.0))}
    }
}


pub struct Qubyte {
    bits: [Qubit; 8],
}


impl Default for Qubyte {
    fn default() -> Qubyte {
	Qubyte{bits: [Qubit::default; 8]}
    }
}

impl Qubyte {

}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex;

    #[test]
    fn test_init_qubyte() {
    }

    #[test]
    fn test_apply_gate_to_qubit_in_qubyte() {
    }
}
    
