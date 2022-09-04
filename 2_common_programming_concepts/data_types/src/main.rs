// four scalar types: integers, floating-point numbers, 
// Booleans, and characters


/* 
    integer is a number without a fractional component
    can be signed or unsigned. Signed numbers use two's complement.
    Signed numbers can store numbers from -(2^n-1) to (2^n-1) - 1, where
    n is the number of bits that varint uses. ex: i8 -> -(2^7) - (2^7) - 1
    or -128 to 127.

    unsigned variants ca nstore from 0 t0 (2^n) - 1, 
    so u8 would be 0 - 255

    isize and usize are architecture dependent, ie 64 bits or 32 bits

    when in doubt, default to i32 ints (which are the rust default). primary use case for isize/usize
    is indexing some type of collection
*/ 

/*
    floating point numbers comoe in two sizes, f64 and f32

    default is f64, because with modern cpus the speed is roughly the same as f32,
    but allows for more precision

*/
fn main() {
    let sum = 1_00 + 1;
    println!("Hello, {sum}!");

    // can have diff types and are fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("tup! {:?}", tup);
    println!("from tup! {x} {y} {z}");
    let z = tup.0;
    println!("tup.0! {z}");
    let unit = ();
    println!("unit! {:?}", unit);

    // arrays are fixed length and elements must have same type
    // allocates on the stack, not the heap
    let a = [1,2,3,4,5];
    println!("array: {:?}", a);
    // let a_typed: [i32; 5] = [1,2,3,4,5];
    let a_same_value = [3; 5];
    println!("same value array: {:?}", a_same_value);
}
