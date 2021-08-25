// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

use std::slice;
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    // Raw Pointer
    // Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    // Aren’t guaranteed to point to valid memory
    // Are allowed to be null
    // Don’t implement any automatic cleanup

    // Cases
    //  when interfacing with C code
    //  when building up safe abstractions that the borrow checker doesn’t understand


    // Creating raw pointers from references
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Dereferencing raw pointers within an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }


    // Creating a raw pointer to an arbitrary memory address
    let address = 0x012345usize;
    let r = address as *const i32;


    // Calling an Unsafe Function or Method

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // Using extern Functions to Call External Code (C)
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or Modifying a Mutable Static Variable

}

unsafe fn dangerous() {}

// created a safe abstraction to the unsafe code
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    
    let len = slice.len();
    // access the raw pointer of a slice
    let ptr = slice.as_mut_ptr();  // *mut i32

    assert!(mid <= len);
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Declaring and calling an extern function defined in another language
// Foreign Function Interface (FFI).
extern "C" {
    fn abs(input: i32) -> i32;
}

// Calling Rust Functions from Other Languages C
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

// Accessing and modifying mutable static variables is unsafe
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
