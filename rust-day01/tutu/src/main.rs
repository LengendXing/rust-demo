/// Test function for variable declarations and assignments
/// This function does not return any value and is used to verify
/// that variables can be declared and assigned correctly.
/// this function will be tested in the test module
#[test]
fn test_const_dif() {
    // defined a as integer
    let a;
    // defined b as integer and assigned value 1
    let b = 1;
    // defined a as boolean
    a = false;
     // defined c as boolean
    let c:bool = true;  
    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn do_const_dif() {
    // defined a as integer
    let a;
    // defined b as integer and assigned value 1
    let b = 1;
    // defined a as boolean
    a = false;
     // defined c as boolean
    let c:bool = true;  
    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn write_rust_as_java_script() {
    // defined a as integer
    let a;
    // defined b as integer and assigned value 1
    let b = 1;
    // defined a as boolean
    a = false;
     // defined c as boolean
    let c:bool = true;  
    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn test_muti_define() {
    let mut x = 1;
    x += 1;
    println!("x: {}", x);
}

fn test_static_var() {
    static VAR: i32 = 100;
    static N:i32 = 200;
    static mut M:i32 = 300;
    println!("VAR: {}", VAR);
    println!("N: {}", N);
    unsafe {
        M += 1;
    }
    println!("M: {}", unsafe { M });
}

/// #[rust Application]
fn main() {
    println!("Hello, world!");
    do_const_dif();
    write_rust_as_java_script();
    test_muti_define();
    test_static_var();
}