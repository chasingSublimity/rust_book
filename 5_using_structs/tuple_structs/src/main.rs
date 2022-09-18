/*
    Tuple structs are useful when one wants to gice the whole tuple a name
    and make the tuple different from other tuples, and when naming
    each field would be verbose or redundant
*/
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(r, g, b) = black;
    let Point(x, y, z) = origin;
    println!("{}, {}, {}", r, g, b);
    println!("{}, {}, {}", x, y, z);

    let subject = AlwaysEqual;
}
