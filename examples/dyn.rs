trait Speaker {
    fn speak(&self);
}

struct Dog;
impl Speaker for Dog {
    fn speak(&self) { println!("Woof!"); }
}
struct Cat;
impl Speaker for Cat {
    fn speak(&self) { println!("Mew!"); }
}

fn main() {
    let mut animal: &dyn Speaker = &Dog;
    animal.speak();
    animal = &Cat;
    animal.speak();

    let dog = &Dog;
    dog.speak();
    // dog = &Cat; // ❌ 类型错误

    println!("usize 的大小:      {} 字节", size_of::<usize>());
    println!("&Dog (普通引用):   {} 字节", size_of::<&Dog>());
    println!("&dyn Speaker (胖): {} 字节", size_of::<&dyn Speaker>());

    // ❌ 不能直接使用 dyn Animal（它是 DST，大小未知）
    // let animal: dyn Speaker = Dog;  // 编译错误！

    // ✅ 特征对象必须通过指针使用
    let animal: &dyn Speaker = &Dog;   // &dyn Animal 是特征对象
    let boxed: Box<dyn Speaker> = Box::new(Dog);  // Box<dyn Animal> 是特征对象
}

trait Good {
    fn foo(&self);
    fn bar(&self) -> i32;
    fn baz(&self) -> Box<dyn Good>;
}

trait Bad {
    // 启用以下任意一行都会报错
    // fn foo();
    // fn bar<T>(&self, t: T) -> i32;
    // fn baz(&self) -> Self;
}

fn call(a: &dyn Good, b: &dyn Bad) {
    todo!()
}

trait Worker {
    type Job;
    fn work(&self, t: Self::Job) -> i32;
}

fn start(c: &dyn Worker<Job=String>) {
    todo!()
}