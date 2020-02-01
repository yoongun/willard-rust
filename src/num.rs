use crate::*;
use num::complex::Complex;


struct Qubit {
    pub state: (Complex<f32>, Complex<f32>),
}


impl Default for Qubit {

}


pub struct Qubyte {
    bits: [Qubit; 8],
}


impl Default for Qubyte {

}

impl Qubyte {
}

