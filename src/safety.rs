fn main() {
    let mut x = vec![10, 20]; //Create vec owned by x
    let y = &x[0]; //Borrow(Reference) the first element of x
    //x.push(30); //Error: might invalidate reference y
}