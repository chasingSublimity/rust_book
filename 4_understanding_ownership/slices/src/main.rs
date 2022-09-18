// slices let you reference a contiguous sequence of elements
// in a collection rather than the whole collection.

// string literals are slices. The following pointsto a specific point
// of the binary
// let s = "Hello, world!"


fn main() {
    let s = String::from("hello world");

    // second arg is exclusive
    println!("{}", first_word(&s))
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}