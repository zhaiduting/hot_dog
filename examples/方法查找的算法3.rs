use std::ops::Deref;

struct Data {
    value: i32,
}

impl Data {
    // Data 自己的方法
    fn action(&self) -> String {
        format!("✅ Data::action(&self) -> value = {}", self.value)
    }

    fn data_only(&self) -> String {
        format!("🔷 Data::data_only() -> only Data has this")
    }
}

struct Wrapper<T> {
    inner: T,
}

impl<T> Wrapper<T> {
    fn new(inner: T) -> Self {
        Wrapper { inner }
    }

    // Wrapper 的方法：接收 &self
    fn action(&self) -> String {
        format!("📦 Wrapper::action(&self) -> wrapper reference")
    }

    // Wrapper 的方法：接收 &mut self
    fn action_mut(&mut self) -> String {
        format!("🔧 Wrapper::action_mut(&mut self) -> mutable reference")
    }
}

impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

fn main() {
    let mut w = Wrapper::new(Data { value: 42 });

    println!("========== 直接调用 ==========");
    // 1. w.action() -> Wrapper 自己有 action(&self)，直接匹配
    println!("w.action()           -> {}", w.action());

    // 2. (&w).action() -> 显式引用，还是匹配 Wrapper::action(&self)
    println!("(&w).action()        -> {}", (&w).action());

    // 3. (&mut w).action_mut() -> 匹配 &mut self 版本
    println!("(&mut w).action_mut()-> {}", (&mut w).action_mut());

    println!("\n========== 手动解引用 ==========");
    // 4. (*w).action() -> 手动解引用得到 Data，调用 Data::action
    println!("(*w).action()        -> {}", (*w).action());

    // 5. (&*w).action() -> 先解引用再引用，得到 &Data，调用 Data::action
    println!("(&*w).action()       -> {}", (&*w).action());

    println!("\n========== 自动解引用演示 ==========");
    // 6. w.data_only() -> Wrapper 没有这个方法，自动解引用找到 Data 的方法
    println!("w.data_only()        -> {}", w.data_only());

    // 7. 另一种自动解引用的例子
    println!("w.deref().action()   -> {}", w.deref().action());
}