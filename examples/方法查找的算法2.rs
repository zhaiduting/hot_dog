use std::ops::Deref;

struct Data {
    value: i32,
}

impl Data {
    // Data 的方法：只能通过引用调用
    fn data_method(&self) -> String {
        format!("✅ Data::data_method(&self) -> value = {}", self.value)
    }
}

struct Wrapper<T> {
    inner: T,
}

impl<T> Wrapper<T> {
    fn new(inner: T) -> Self {
        Wrapper { inner }
    }

    // Wrapper 没有实现 data_method
    // 但为了证明自动加引用，我们不给 Wrapper 任何同名方法
}

impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("  🔍 触发了 Deref::deref()");
        &self.inner
    }
}

// 一个只能通过引用调用的独立函数
fn requires_ref(x: &i32) -> String {
    format!("🎯 函数接收到了引用: {}", x)
}

fn main() {
    let w = Wrapper::new(Data { value: 42 });
    let num = Wrapper::new(100);

    println!("========== 证明自动加引用 ==========");
    // 关键：Wrapper 没有 value 方法，但 i32 有
    // 编译器会尝试：
    // 1. num.value() ❌ 不存在
    // 2. (&num).value() ❌ 还是不存在
    // 3. (*num).value() ✅ 解引用后是 i32，然后呢？等等，i32 也没有 value() 方法啊

    // 更清晰的例子：调用一个需要 &self 的方法
    // Data 有 data_method(&self)，但 Wrapper 没有
    println!("w.data_method() 调用过程:");
    let result = w.data_method();  // 编译器会自动加引用和解引用！
    println!("  结果: {}\n", result);

    println!("让我们一步步模拟编译器做的事:");
    println!("第1步: w.data_method() ❌ Wrapper 没有 data_method");
    println!("第2步: (&w).data_method() ❌ &Wrapper 也没有 data_method");
    println!("第3步: (*w).data_method() ✅ 解引用得到 Data");
    println!("第4步: 等等，(*w) 是 Data 类型，但 data_method 需要 &self");
    println!("第5步: 自动加引用 → (&(*w)).data_method() ✅ 匹配成功！");

    println!("\n========== 手动模拟完整过程 ==========");
    // 手动模拟编译器做的事
    println!("手动: (&(*w)).data_method() -> {}", (&(*w)).data_method());

    println!("\n========== 证明多层解引用 ==========");
    let w2 = Wrapper::new(Wrapper::new(Data { value: 99 }));
    println!("w2.data_method() 调用过程:");
    println!("  结果: {}", w2.data_method());
    // 编译器会一直解引用直到找到 Data

    println!("\n========== 区分不同形态 ==========");
    let w3 = Wrapper::new(Data { value: 777 });

    // 形态1: 直接调用（自动引用+解引用）
    println!("1. w3.data_method()       -> {}", w3.data_method());

    // 形态2: 手动解引用一次
    println!("2. (*w3).data_method()    -> {}", (*w3).data_method());

    // 形态3: 手动引用后调用
    println!("3. (&w3).data_method()    -> {}", (&w3).data_method());
}