enum Movement {
	Up,
	Down,
	Left,
	Right
}

fn move_piece(m: Movement) {
	match m {
		Movement::Up => println!("Mvoing up"),
		Movement::Down => println!("Mvoing down"),
		Movement::Left => println!("Mvoing left"),
		Movement::Right => println!("Mvoing right"),
	}
}

pub fn run() {
	let p1 = Movement::Left;
	let p2 = Movement::Right;
	let p3 = Movement::Up;
	let p4 = Movement::Down;
	move_piece(p1);
	move_piece(p2);
	move_piece(p3);
	move_piece(p4);
}