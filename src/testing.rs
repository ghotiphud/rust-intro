#[cfg(not(test))]
fn main(){
	automatic_cleanup();
}

pub fn automatic_cleanup() {
	let mut v = Vec::new();
	v.push(10);
	v.push(20);
	v.push(30);
	println!("vec: {:?}", v);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn positive_test() {
		automatic_cleanup();
		assert!(true);
	}

	#[test]
	#[should_panic]
	fn negative_test() {
		assert!(false);
	}
}