#[repr(C)]
struct RustObject {
    a: i32,
    // Other members...
}

extern "C" fn callback(target: *mut RustObject, a: i32) {
    println!("I'm called from C with value {0}", a);
    unsafe {
        println!("I'm called from C with value {0}", (*target).a);

        // Update the value in RustObject with the value received from the callback:
        (*target).a = a;
        println!("I'm called from C with value {0}", (*target).a);
    }
}

#[link(name = "extlib")]
extern "C" {
    fn register_callback(target: *mut RustObject, cb: extern "C" fn(*mut RustObject, i32)) -> i32;
    fn trigger_callback();
}

fn main() {
    // Create the object that will be referenced in the callback:
    let mut rust_object = Box::new(RustObject { a: 5 });
    unsafe {
        register_callback(&mut *rust_object, callback);
        trigger_callback(); // Triggers the callback.
    }
}
