fn main() {
    // String type is allocated on the heap, unlike string literals.
    // String is mutable, unlike string literals. So, the latter is for hardcoded values.
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}",s);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}", s1); // not allowed, since the value has moved to s2
    
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); // allowed, since s2 is a deep copy
    

    let s3 = String::from("hello");
    takes_ownership(s);
    println!("{}", s3);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// For mutable strings to work, we need:
// * the memeory must be requested from the memory allocator at runtime
// * a way of returning this memory to the allocator when we're done with our String

// In rust, memory is automatically returned to the allocator after the owning
// variable goes out of scope. Also known as RAII (Resource Acquisition is Initialization)

// String is made up of three parts:
//  * pointer
//  * length
//  * capacity

// Rust will never automatically create deep copies of your data
// So, any automatic copying can be assumed to be cheap wrt runtime perf