use std::cell::Cell;

struct Signal {
    // 1. 用 Cell 装数据，这样不用写 mut 也能改
    value: Cell<i32>,
}

impl Signal {
    // 2. 写：定义一个简单的 set 方法
    fn set(&self, new_val: i32) {
        self.value.set(new_val);
    }

    // 3. 读：定义一个简单的 get 方法（对应你看到的 title()）
    fn get(&self) -> i32 {
        self.value.get()
    }
}

fn main() {
    // 这里没写 mut 吧？
    let title = Signal { value: Cell::new(100) };

    println!("Value is: {}", title.get());

    // 但我依然能改它！这就是 Cell 的功劳
    title.set(200);

    // 我通过调用方法拿回值
    println!("Value is: {}", title.get());
}