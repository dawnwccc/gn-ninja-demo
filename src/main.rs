include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        println!("2 + 3 = {}", add(2, 3));
        println!("4 * 5 = {}", mul(4, 5));
    }
}