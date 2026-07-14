fn main() {
    let x = 10;
    let f = || x + 1;
    println!("{x}");
    f();
    f();

    let mut x = 10;
    let mut f = || {
        x += 1;
    };
    f();
    f();
    println!("{x}");

    let world = "world".to_string();
    let f = || {
        let move_world = world;
    };
    f();
    // f(); // 不能再次执行 FnOnce
    // println!("{hello}");
    // println!("{world}"); // 不能借用已被移走的变量


    let s = String::from("hi");

    let f = || {
        let t = s;
    };

    // println!("{s}");
}