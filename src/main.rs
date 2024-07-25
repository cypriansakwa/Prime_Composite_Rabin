use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};

fn mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
	let mut base = base.clone();
	let mut exp = exp.clone();
	let mut result = BigUint::one();
	
	while !exp.is_zero() {
		if &exp % 2u8 == BigUint::one() {
			result = (&result * &base) % modulus;
		}
		exp >>= 1;
		base = (&base * &base) % modulus;
	}
	
	result
}

fn miller_rabin_test(n: &BigUint, k: u32) -> bool {
	if n <= &BigUint::from(1u8) {
		return false;
	}
	if n <= &BigUint::from(3u8) {
		return true;
	}
	if n % 2u8 == BigUint::zero() {
		return false;
	}
	
	let one = BigUint::one();
	let two = &one + &one;
	let n_minus_one = n - &one;
	
	let mut d = n_minus_one.clone();
	let mut r = 0u32;
	while &d % 2u8 == BigUint::zero() {
		d >>= 1;
		r += 1;
	}
	
	let mut rng = rand::thread_rng();
	for _ in 0..k {
		let a = rng.gen_biguint_range(&two, &(n - &two));
		let mut x = mod_exp(&a, &d, &n);
		
		if x == one || x == n_minus_one {
			continue;
		}
		
		let mut continue_outer = false;
		for _ in 0..(r - 1) {
			x = (&x * &x) % n;
			if x == n_minus_one {
				continue_outer = true;
				break;
			}
		}
		
		if continue_outer {
			continue;
		}
		
		return false;
	}
	
	true
}

fn main() {
	let number = BigUint::parse_bytes(
	b"561",
	10,
	)
	.unwrap();
	let k = 20; // Number of iterations for the Miller-Rabin test
	
	if miller_rabin_test(&number, k) {
		println!("The number is probably prime.");
	} else {
		println!("The number is composite.");
	}
}
