use num_bigint::BigUint;

// Entrypoint for each of the factorial threads
pub fn run(start: usize, end: usize) -> BigUint {
	let mut section = BigUint::new(vec![1]);
	for count in start..end+1 {
		section *= count;
	}
	
	return section;
}
