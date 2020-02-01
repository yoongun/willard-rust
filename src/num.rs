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
    fn test_h_gate() {
	let qubyte = Qubyte::default();

	qubyte.h(0);
    }

    #[test]
    fn test_collapse_of_state() {
	// Prepare the qubyte and the qubit
	let qubyte = Qubyte::default();
	qubyte.h(0);
	let want = qubyte.measure(0);
	for _n in 0..100 {
	    let got = qubyte.maasure(0);
	    assert_eq!(got, want);
	}
    }

    #[test]
    fn test_x_gate() {
    }

    #[test]
    fn test_y_gate() {
    }

    #[test]
    fn test_z_gate() {
    }

    #[test]
    fn test_swap_gate() {
    }

    #[test]
    fn test_cnot_gate() {
    }
}
    
