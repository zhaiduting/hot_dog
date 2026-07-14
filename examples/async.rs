#![allow(unused)]
use futures::executor::block_on;
fn main() {
    // test2();
    test3();
    test3();
}

fn test1() {
    let fut = async {
        println!("hello");
    };

    let s = String::from("hello");
    let fut = async move {
        println!("{s}");
    };
    // println!("{s}"); // ❌

    let n = 8;
    let fut = async move {
        println!("{n}");
    };
    println!("{n}"); // ✅
}
fn test2() {
    let s = String::from("hello");
    let w = String::from("world");
    let f = |ww| async move {
        println!("{s} {ww}");
    };
    block_on(f(w));

    // println!("{s}"); // ❌
    // println!("{w}"); // ❌
}
fn test3() {
    use futures::executor::block_on;

    let s = String::from("hello");

    let f = move |x: i32| async move {
        println!("{s}, {x}");
    };

    block_on({ f(1) });
    // block_on({ f(2) });
}