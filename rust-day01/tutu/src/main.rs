
/// #[rust Application]
fn main() {
    println!("Hello, world!");
    do_const_dif();
}


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