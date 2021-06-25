pub fn run() {
	let x = 123;

	let y: u64 = 456;

	let z: i128 = 789;

	let is_active: bool = true;

	let x_gt_y: bool = x > y;

	let face_smile = '\u{1f600}';

	// let cum_rocket = '\u{1f600}\u{1f599}';

	println!("{:?}", (x, y, z, is_active, x_gt_y, face_smile));
}