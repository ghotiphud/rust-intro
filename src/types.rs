#[allow(dead_code)]

fn main() {
	primative_types();

	structs();

	enums();
}

fn primative_types() {
	let x = 1_i32;

	let y = x as u64;

	let mut z: u8 = x as u8;
	z = 8;

	println!("x:{:?} y:{:?}, z:{:?}", x, y, z);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn structs() {
	let origin = Point { x: 0, y: 0 };

	println!("{:?}", origin);
}

#[derive(Debug)]
enum Options {
	None,
    One,
    Two,
    Three,
}

fn enums() {
	let option = Options::One;

	println!("{:?}", option);
}