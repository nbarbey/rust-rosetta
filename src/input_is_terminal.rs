// Implements http://rosettacode.org/wiki/Check_input_device_is_a_terminal

extern crate libc;

#[cfg(not(test))]
fn main() {
    let istty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;
    if istty {
        println!("stdin is tty");
    } else {
        println!("stdin is not tty");
    }
}
