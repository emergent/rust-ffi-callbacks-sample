use std::ffi::CString;
use std::ptr;

#[link(name = "readline")]
extern "C" {
    static mut rl_prompt: *const libc::c_char;
    static rl_readline_version: libc::c_int;
}

fn main() {
    let prompt = CString::new("[my-awesome-shell] $").unwrap();
    println!("You have readline version {} installed.", unsafe {
        rl_readline_version as i32
    });
    unsafe {
        rl_prompt = prompt.as_ptr();

        println!("{:?}", rl_prompt);

        rl_prompt = ptr::null();
    }
}
