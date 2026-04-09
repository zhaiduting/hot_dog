use dioxus::prelude::*;

fn main() {
    dioxus::logger::initialize_default();
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { "body {{ font-family: sans-serif; padding: 20px; line-height: 1.6; }}" }
        div {
            MyTitle {}
            section { MyCount {} }
            hr {}
            section { MyUser {} }
            hr {}
            section { MyAction {} }
        }
    }
}

#[component]
fn MyCount() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        p { "当前计数: {count}" }
        button { onclick: move |_| *count.write() += 2, "数字 +2" }
        button { onclick: move |_| count.with_mut(|c| *c += 1), "数字 +1" }
    }
}

#[component]
fn MyTitle() -> Element {
    let mut title = use_signal(|| "Dioxus 状态实验，点击颠倒 🎯".to_string());
    rsx! {
        h1 {
            onclick: move |_| {
                let mut t = title.write();
                *t = t.chars().rev().collect();
            },
            "{title}"
        }
    }
}

struct User {
    name: String,
    age: u32,
}
#[component]
fn MyUser() -> Element {
    // 结构体：复杂对象
    let mut user = use_signal(|| User {
        name: "Alice".into(),
        age: 25,
    });
    rsx! {
        h3 { "结构体 (Struct)" }
        p { "姓名: {user.read().name}" }
        p { "年龄: {user.read().age}" }
        button { onclick: move |_| user.write().age += 1, "年龄 +1" }
    }
}

#[component]
fn MyAction() -> Element {
    // 函数：必须包装在 Box<dyn Fn()> 中，解决闭包匿名类型不匹配问题
    let mut action = use_signal::<Box<dyn Fn()>>(|| {
        Box::new(|| info!("初始动作：日志已打印"))
    });
    rsx! {
        h3 { style: "color:red", "闭包函数 (Boxed Closure)" }
        button { onclick: move |_| action.read()(), "输出信息到控制台" }
        button {
            // 写：替换函数时也需要 Box::new
            onclick: move |_| action.set(Box::new(|| info!("新动作：新日志！"))),
            "切换为新函数"
        }
    }
}
