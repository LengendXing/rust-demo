fn test_print() {
    println!("Hello World")
}

fn test_print_format() {
    // 第一种方式打印
    println!("{}, {}!", "Hello", "world"); // Hello, world!
    // 第二种方式打印
    println!("{0}, {1}!", "Hello", "world"); // Hello, world!
    // 第三种方式打印
    println!("{greeting}, {name}!", greeting="Hello", name="world"); // Hello, world!
    // 第四种 声明变量的方式打印
    // let y = String::from("Hello, ") + "world!";
    let x = String::from("Hello world!") + "scx";
    println!("{}", x); // Hello, world!scx
}

fn main() {
    // test_print()
    test_print_format()
}