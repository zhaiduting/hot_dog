trait Config {
    const VERSION: &str;  // 类似静态属性
    const MAX_SIZE: usize;
}

struct MyConfig;

impl Config for MyConfig {
    const VERSION: &str = "hello";
    const MAX_SIZE: usize = 1024;
}

// 使用
fn main() { println!("{}, {}", MyConfig::VERSION, MyConfig::MAX_SIZE); }