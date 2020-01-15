use crate::Qubit;
use num::complex::Complex;

pub fn x(qubit: &mut Qubit) {
    let mat = [[Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
	       [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]];
    let mut state: (Complex<f32>, Complex<f32>)= (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));

    state.0 = mat[0][0] * qubit.state.0 + mat[0][1] * qubit.state.1;
    state.1 = mat[1][0] * qubit.state.0 + mat[1][1] * qubit.state.1;

    qubit.state = state;
}

pub fn y(qubit: &mut Qubit) {
    let mat = [[Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
	       [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)]];
    let mut state: (Complex<f32>, Complex<f32>)= (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));

    state.0 = mat[0][0] * qubit.state.0 + mat[0][1] * qubit.state.1;
    state.1 = mat[1][0] * qubit.state.0 + mat[1][1] * qubit.state.1;

    qubit.state = state;
}

pub fn z(qubit: &mut Qubit) {
    phase(qubit, std::f32::consts::PI);
}

pub fn phase(qubit: &mut Qubit, deg: f32) {
    let mat = [[Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
	       [Complex::new(0.0, 0.0), Complex::new(deg.cos(), deg.sin())]];
    let mut state: (Complex<f32>, Complex<f32>)= (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));

    state.0 = mat[0][0] * qubit.state.0 + mat[0][1] * qubit.state.1;
    state.1 = mat[1][0] * qubit.state.0 + mat[1][1] * qubit.state.1;

    qubit.state = state;
}

pub fn not(qubit: &mut Qubit) {
    x(qubit)
}

pub fn h(qubit: &mut Qubit) {
    let root_two = (2.0 as f32).sqrt();
    let mat = [[1.0 / root_two, 1.0 / root_two],
		[1.0 / root_two, -1.0 / root_two]];
    let mut state: (Complex<f32>, Complex<f32>)= (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));

    state.0 = mat[0][0] * qubit.state.0 + mat[0][1] * qubit.state.1;
    state.1 = mat[1][0] * qubit.state.0 + mat[1][1] * qubit.state.1;

    qubit.state = state;
}

pub fn sqrt_not(qubit: &mut Qubit) {
    let mat = [[Complex::new(1.0, 1.0) / 2.0, Complex::new(1.0, -1.0) / 2.0],
	       [Complex::new(1.0, -1.0) / 2.0, Complex::new(1.0, 1.0) / 2.0]];
    let mut state: (Complex<f32>, Complex<f32>)= (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));

    state.0 = mat[0][0] * qubit.state.0 + mat[0][1] * qubit.state.1;
    state.1 = mat[1][0] * qubit.state.0 + mat[1][1] * qubit.state.1;

    qubit.state = state;
}

fn normalize_phase(state: (Complex<f32>, Complex<f32>)) -> (f32, Complex<f32>) {
    if (state.0 == Complex{re: 0.0, im: 0.0}) {
	return (0.0, Complex::new(1.0, 0.0));
    }

    let size = (state.0.conj() * state.0).re.sqrt();
    let d_phase = (state.0 / size).conj();

    let l = (d_phase * state.0).re;
    let r = d_phase * state.1;
    (l, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex;

    #[test]
    fn test_x() {
	let mut qubit = Qubit::default();
	x(&mut qubit);
	assert_eq!(qubit.state, (Complex{re: 0.0, im: 0.0}, Complex{re: 1.0, im: 0.0}));
    }

    #[test]
    fn test_y() {
	let mut qubit0 = Qubit::default();
	y(&mut qubit0);
	assert_eq!(qubit0.state, (0.0, Complex{re: 1.0, im: 0.0}));

	let mut qubit1 = Qubit::default();
	x(&mut qubit1);
	y(&mut qubit1);
	assert_eq!(qubit1.state, (1.0, Complex{re: 0.0, im: 0.0}));
    }

    #[test]
    fn test_z() {
	let mut qubit0 = Qubit::default();
	z(&mut qubit0);
	assert_eq!(qubit0.state, (1.0, Complex{re: 0.0, im: 0.0}));

	let mut qubit1 = Qubit::default();
	x(&mut qubit1);
	z(&mut qubit1);
	assert_eq!(qubit1.state, (0.0, Complex{re: 1.0, im: 0.0}));
    }

    #[test]
    fn test_not() {
	let mut qubit = Qubit::default();
	not(&mut qubit);
	assert_eq!(qubit.state, (0.0, Complex{re: 1.0, im: 0.0}));
    }

    #[test]
    fn test_h() {
	// Test on a qubit of state [1.0, 0.0]
	let mut qubit0 = Qubit::default();

	h(&mut qubit0);

	let root_two = (2.0 as f32).sqrt();

	let got = qubit0.state;
	let want = (1.0 / root_two, Complex{re: 1.0 / root_two, im: 0.0});

	assert_eq!(got, want);

	// Test on a qubit of state [0.0, 1.0]
	let mut qubit1 = Qubit::default();
	not(&mut qubit1);

	h(&mut qubit1);

	let got = qubit1.state;
	let want = (1.0 / root_two, Complex{re: -1.0 / root_two, im: 0.0});

	assert_eq!(got, want);
    }

    #[test]
    fn test_sqrt_not() {
	let mut qubit = Qubit::default();

	sqrt_not(&mut qubit);

	sqrt_not(&mut qubit);
	let mut other_qubit = Qubit::default();
	not(&mut other_qubit);

	let got = qubit.state;
	let want = other_qubit.state;

	assert_eq!(got, want);
    }

    #[test]
    fn test_normalize_phase() {
	let state = (Complex::new(0.0, -1.0), Complex::new(1.0, 0.0));
	let got = normalize_phase(state);
	let want = (1.0, Complex::new(0.0, 1.0));
	assert_eq!(got, want);
    }
}

