use std::cell::Cell;
use std::fmt;

// 1. 定义结构体
struct Signal<T>(Cell<T>);

impl<T: Copy> Signal<T> {
    fn new(val: T) -> Self {
        Signal(Cell::new(val))
    }
    // 模拟 title.set(val)
    fn set(&self, val: T) {
        self.0.set(val);
    }
}

// 2. 实现 Display：让 println!("{}", title) 能够运行
impl<T: fmt::Display + Copy> fmt::Display for Signal<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get())
    }
}

// 3. 实现 Deref：让你能用 *title 直接取值（代替 title()）
impl<T: Copy> std::ops::Deref for Signal<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // 注意：Cell 没法直接返回引用，这里仅作演示逻辑
        // 实际框架中这里会有一套极其复杂的“槽”索引逻辑
        unsafe { &*self.0.as_ptr() }
    }
}

fn main() {
    let title = Signal::new(100);

    title.set(200); // 写

    println!("Value is: {}", title); // 读：直接打印！

    let val = *title + 5; // 读：通过解引用直接参与运算
    println!("Calc: {}", val);
}
