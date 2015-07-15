#![feature(core_intrinsics)]
#[allow(dead_code)]

fn main() {
    numeric_types();

    string_types();

    structs();

    enums();
}

fn numeric_types() {
    let x = 2_i32;

    let mut y = x as i64;
    y = 4;

    let z: u8 = 8;

    println!("x:{:?} y:{:?}, z:{:?}", x, y, z);
}

fn string_types() {
    let s = "a string";
    let mut string = s.to_string();

    string.push_str(", hi!");

    print_type_of(&s);
    print_type_of(&string);

    println!("{:?}", s);
    println!("{:?}", string);
}

#[derive(Debug, Clone, Copy)]
struct Point { x: i32, y: i32 }

fn structs() {
    let origin = Point { x: 0, y: 0 };

    let mut not_origin = origin.clone();
    not_origin.x = 10;

    let mut not_origin2 = origin;
    not_origin2.y = 20;

    println!("{:?}, {:?}, {:?}", origin, not_origin, not_origin2);
}

#[derive(Debug)]
enum Options { None, One, Two, Three }

fn enums() {
    let option = Options::One;

    let option_str = match option {
        Options::One => "Option One",
        Options::Two => "Option Two",
        _ => "The Rest",
    };

    println!("{:?}", option_str);
}


fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            std::intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}