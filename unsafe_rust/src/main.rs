use core::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    add_to_count(3);

    unsafe { println!("COUNTER: {}", *(&raw const COUNTER)) }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

unsafe fn dengerous() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
