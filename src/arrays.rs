use std::mem;

pub fn run() {
	let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

	numbers[2] = 99;

	println!("{:?}", numbers);

	// Single val
	println!("Single: {}", numbers[0]);

	// Array length
	println!("Array length: {}", numbers.len());

	// Array mem size
	println!("Array occupies {} bytes", mem::size_of_val(&numbers));

	// Slice
	let slice: &[i32] = &numbers[1..3];
	println!("Slice: {:?}", slice);
}