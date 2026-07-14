fn main() {
    fn borrow_str(s: &str) {
        todo!()
    }

    let s = "hello".to_string();
    move || borrow_str(&s);
    // println!("{s}"); // 无法打印已被移动的 s

    let t = "world".to_string();
    move || {
        borrow_str(&t);
        println!("{t}");
    };
    // println!("{t}"); // 无法打印已被移动的 t

    let pet = Animal {
        name: "x".to_string(),
        skills: vec!["ab", "xy"],
    };
    move || println!("{:?}", pet);
}

#[derive(Debug)]
struct Animal {
    name: String,
    skills: Vec<&'static str>,
}