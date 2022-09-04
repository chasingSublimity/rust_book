// Constants are immutable, can be declared in any scope
// and can only be set to a constant expression, not the
// result of a value only computed at runtime
const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}")
// }



fn main() {
    let x = 5;

    // this x shadows the first x
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }
    println!("The value of x is: {x}")
 }