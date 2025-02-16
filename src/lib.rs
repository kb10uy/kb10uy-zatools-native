#![deny(unsafe_attr_outside_unsafe)]

#[unsafe(no_mangle)]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
