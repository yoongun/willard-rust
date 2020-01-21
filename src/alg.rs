/// Set of quantum algorithms implemented in the willard
use crate::*;

/// # Quantum Random Number Generator
///
/// Returns a single bit with a value 0 or 1 with
/// 50% of possibility each.
pub fn qrng() -> u32 {
    let mut qubit = Qubit::default();
    gate::h(&mut qubit);
    return measure(&mut qubit);
}

/// # Quantum Key Distribution algorithm
///
/// 
pub fn qkd(spy: bool, use_h: bool) -> bool {
    let mut alice = Qubit::default();
    let mut conn = Qubit::default();
    let mut bob = Qubit::default();

    // ### Start Alice part ###
    let a1 = qrng();
    let a2 = qrng();

    if a1 == 1 {
	gate::x(&mut alice);
    }
    if a2 == 1 {
	gate::h(&mut alice);
    }
    // ### End Alice part #####

    // ### Sends the qubit via fiber ###
    gate::swap(&mut conn, &mut alice);

    // ### Sniffing starts ###
    if spy == true {
	if use_h == true {
	    gate::h(&mut conn);
	}

	let stolen = measure(&mut conn);

	if stolen == 0 {
	    conn.state = (Complex::new(1.0, 0.0), Complex::new(0.0, 0.0));
	}
	else {
	    conn.state = (Complex::new(0.0, 0.0), Complex::new(1.0, 0.0));
	}

	if use_h == true {
	    gate::h(&mut conn);
	}
    }
    // ### Sniffing ends #####

    // ### Bob receives the qubit ###
    let a3 = qrng();
    gate::swap(&mut bob, &mut conn);

    if a3 == 1 {
	gate::h(&mut bob);
    }
    let recv = measure(&mut bob);
    // ### Bob part ends ############

    // ### Checks whether the spy sniffed the qubit ###
    if a2 == a3 {
	if a1 != recv {
	    return true;
	}
    }
    // ### Sniffing check end #########################

    return false;
}

