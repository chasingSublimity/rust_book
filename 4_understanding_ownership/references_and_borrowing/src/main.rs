// References are like pointers in that they are an address that
// can be followed to access data. Unlike a pointer, a ref is
// guaranteed to point to a valid value of a particular type
// for the life of that reference.

fn main() {
    let mut s1 = String::from("hello");

    // ampersands represent references, and they allow you to 
    // refer to some value without taking ownership of it
    // the act of creating a reference is called borrowing
    change(&mut s1);

    println!("{}", s1)
}

fn change(s: &mut String) {
    s.push_str(", World");
}
