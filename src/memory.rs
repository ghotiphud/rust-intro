fn main(){
    automatic_cleanup();
    println!("");

    give_vec();
    println!("");

    return_vec();
    println!("");

    pass_vec();
    println!("");

    pass_vec_mut();
    println!("");
}

fn automatic_cleanup() {
    let mut v = Vec::new(); //Create Vec on Heap
    v.push(10); //Add elements
    v.push(20); //Backing store is resized if needed
    v.push(30);
    println!("vec: {:?}", v);
} //Vec has now gone out of scope and is immediately freed


fn give_vec() {
    let vec = vec![1, 2, 3];
    take_ownership(vec);
    // no longer have access to vec, so this would fail.
    // println!("{:?}", vec);
}

fn take_ownership(vec: Vec<i32>){
    println!("owned vec: {:?}", vec);
} //Vec has now gone out of scope and is immediately freed

fn return_vec() {
    let vec = vec![1, 2, 3];
    let vec2 = return_ownership(vec);
    // no longer have access to vec, so this would fail.
    println!("returned vec: {:?}", vec2);
} // Vec2 freed

fn return_ownership(vec: Vec<i32>) -> Vec<i32> {
    println!("owned vec: {:?}", vec);
    vec // Return the vec from the function
}

fn pass_vec() {
    let vec = vec![4, 5, 6];
    borrow_ref(&vec);
    // vec is still owned, so we can access it.
    println!("vec: {:?}", vec);
}

fn borrow_ref(vec: &Vec<i32>) {
    // Passed in an immutable reference to our Vec.
    println!("borrowed vec: {:?}", vec);
    // Error: Cannot mutate an immutable reference.
    // vec.push(50);
} //Borrow has gone out of scope


fn pass_vec_mut() {
    let mut vec = vec![7,8,9];
    borrow_mut(&mut vec);
    println!("vec: {:?}", vec);
}

fn borrow_mut(vec: &mut Vec<i32>) {
    println!("borrowed mut vec: {:?}", vec);
    // in effect we have temporary ownership.
    vec.push(50);
    vec[0] += 100;
}