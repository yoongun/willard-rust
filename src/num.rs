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

