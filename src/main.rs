#[allow(dead_code)]

fn main() {
    let data = vec![1,2,3,4,5,6,7,8,9,10,128,222,12,13,14];

    println!("{:?}", sum_pos(&data));
}

// sums all the positive values in `v`
fn sum_pos(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v.iter().filter(|i| **i > 0) {
        sum += *i;
    }
    sum
}

// fn this_wont_compile(v: &mut Vec<i32>) -> i32 {
//     let mut sum = 0;
//     for &i in v.iter() {
//         sum += i;
//         if i > 0 { v.push(0); }
//         //         ~~~~~~~~~ invalid! (might realloc
//         //                   the backing storage for `v`)
//     }
//     sum
// }