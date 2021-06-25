use std::mem;

pub fn run() {
	let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

	numbers[2] = 99;

	println!("{:?}", numbers);

	// Single val
	println!("Single: {}", numbers[0]);

	// Array length
	println!("Vector length: {}", numbers.len());

	// Array mem size
	println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

	// Slice
	let slice: &[i32] = &numbers[1..3];
	println!("Slice: {:?}", slice);

	// Push
	numbers.push(5);

	// Loop
	for x in numbers.iter() {
		println!("Val: {}", x);
	}

	// Loop, mutated
	for x in numbers.iter_mut() {
		*x += 1;
		println!("Val (mutated): {}", x);
	}
}