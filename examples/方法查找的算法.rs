use std::ops::{Deref, DerefMut};

struct Wrapper(i32);

// 1. 在原始类型上定义方法
impl Wrapper {
    fn who_am_i(&self) -> &'static str { "Original Wrapper (&self)" }
    fn who_am_i_mut(&mut self) -> &'static str { "Original Wrapper (&mut self)" }
}

// 定义一个 Trait 来模拟“解引用后的目标类型”上的方法
trait TargetMethods {
    fn who_am_i(&self) -> &'static str;
}

// 为 i32 (Wrapper 的 Target) 实现这个 Trait
impl TargetMethods for i32 {
    fn who_am_i(&self) -> &'static str { "Deref Target (i32)" }
}

impl Deref for Wrapper {
    type Target = i32;
    fn deref(&self) -> &Self::Target { &self.0 }
}

fn main() {
    let mut w = Wrapper(42);

    println!("--- 开始测试方法查找优先级 ---");

    // A. 直接调用：匹配 Wrapper 上的方法
    // 优先级最高：直接在当前类型上找到了匹配。
    println!("w.who_am_i()       => {}", w.who_am_i());

    // B. 对引用调用：自动解引用
    // 虽然 w_ref 是 &Wrapper，但编译器发现 &Wrapper 也可以调用 Wrapper 上的 &self 方法。
    let w_ref = &w;
    println!("w_ref.who_am_i()   => {}", w_ref.who_am_i());

    // C. 手动解引用再调用：触发 Deref 后的方法
    // *w 会变成 i32 类型。由于 i32 上有同名方法，编译器直接定位到 i32 的实现。
    println!("(*w).who_am_i()     => {}", (*w).who_am_i());

    // D. 强制触发 Deref Target 的另一种方式
    // 这里通过 &*w 获取了 &i32，从而绕过了 Wrapper 本身的方法查找。
    println!("(&*w).who_am_i()    => {}", (&*w).who_am_i());

    // E. 一个有趣的现象：如果没有 Wrapper 上的方法呢？
    // 如果我们注释掉 impl Wrapper 里的 who_am_i，
    // 那么 w.who_am_i() 就会因为在第一层找不到，而自动 Deref 到 i32 上去找。
}