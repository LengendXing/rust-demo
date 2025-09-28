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

fn test_param_type(x: i32, y: bool) {
    println!("x: {}, y: {}", x, y);
}

fn test_param_type2(x: i8, y: bool) -> i8 {
    println!("x: {}, y: {}", x, y);
    x + 1
}

fn test_plus_one(x: i8) -> (i8, i8) {
    (x, x + 1)
}

fn mehtod_point_as_context(param: i8) {
    println!("This is a method point as context");
    let (x,y) = test_plus_one(param);
    println!("x: {}, y: {}", x, y);
    let g = test_param_type2;
    let result = g(50, true);
    println!("result: {}", result);
}

/// rust中数据居然可以定义为可变的,在java中是不可变的
/// 数组的定义
fn test_array() {
    let a = [1, 2, 3]; // a[0] = 1, a[1] = 2, a[2] = 3
    let b: [i32; 4] = [1, 2, 3, 2]; // b[0] = 1, b[1] = 2, b[2] = 3
    let c = [0; 10]; // c[0] = 0, c[1] = 0, c[2] = 0, c[3] = 0, c[4] = 0, c[5] = 0, c[6] = 0, c[7] = 0, c[8] = 0, c[9] = 0
    println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);
    // 测试可变
    let mut d = [1, 2, 3];
    d[0] = 10;
    println!("d: {:?}", d);
}

/// #[rust Application]
fn main() {
    println!("Hello, world!");
    // 测试变量声明和赋值
    do_const_dif();
    // 以 JavaScript 语法来写 Rust
    write_rust_as_java_script();
    // 测试多次定义
    test_muti_define();
    // 测试静态变量
    test_static_var();
    // 测试参数类型
    test_param_type(10, true);
    // 测试表达式返回值省略
    let result = test_param_type2(20, false);
    println!("result: {}", result);
    // 测试操作元组
    let (a, b) = test_plus_one(5);
    // 测试返回元组
    println!("a: {}, b: {}", a, b);
    // 函数指针也可以作为变量使用
    mehtod_point_as_context(1);
    // 测试数组
    test_array();
}

