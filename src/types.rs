use crate::*;
use num::complex::Complex;


struct Qubit {
    pub state: [Complex<f32>; 2],
}


impl Default for Qubit {
    fn default() -> Qubit {
	Qubit{state: [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]}
    }
}


pub struct Qubyte {
    bits: [Qubit; 8],
}


impl Default for Qubyte {
    fn default() -> Qubyte {
	Qubyte{bits: [
	    Qubit::default(),
	    Qubit::default(),
	    Qubit::default(),
	    Qubit::default(),
	    Qubit::default(),
	    Qubit::default(),
	    Qubit::default(),
	    Qubit::default(),
	]}
    }
}

impl Qubyte {
    fn measure(&mut self, idx: usize) -> u32 {
	let mut rng = rand::thread_rng();
	let rn = rng.gen::<f32>();

	let alpha_sq = (self.bits[idx].state[0].conj() * self.bits[idx].state[0]).re;

	if rn < alpha_sq {
	    self.bits[idx].state = [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)];
	    return 0;
	}
	self.bits[idx].state = [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)];
	return 1;
    }

    fn h(&mut self, idx: usize) {
	let root_two = (2.0 as f32).sqrt();
	let mat = [[1.0 / root_two, 1.0 / root_two],
		    [1.0 / root_two, -1.0 / root_two]];

	let state = [
	    mat[0][0] * self.bits[idx].state[0] + mat[0][1] * self.bits[idx].state[1],
	    mat[1][0] * self.bits[idx].state[0] + mat[1][1] * self.bits[idx].state[1]
	];

	self.bits[idx].state = state;
    }

    fn x(&self) {
    }

    fn y(&self) {
    }

    fn z(&self) {
    }

    fn cnot(&self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex;


    #[test]
    fn test_collapse_of_state() {
	let mut qubyte = Qubyte::default();
	qubyte.h(0);

	let want = qubyte.measure(0);
	for _n in 0..100 {
	    let got = qubyte.measure(0);
	    assert_eq!(got, want);
	}
    }

    #[test]
    fn test_swap_gate() {
    }

    #[test]
    fn test_cnot_gate() {
	for _n in 0..100 {
	    let mut qubyte = Qubyte::default();
	    qubyte.h(0);

	    qubyte.cnot(1, [0]);

	    let want = qubyte.measure(0);
	    let got = qubyte.measure(1);
	    assert_eq!(got, want);
	}
    }
}
    
