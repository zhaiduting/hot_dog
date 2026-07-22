use dioxus::prelude::*;

#[post("/api/save_dog")]
pub async fn save_dog(image: String) -> Result<()> {
    println!("save_dog");
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")?;
    file.write_fmt(format_args!("{}\n", image))?;

    println!("save_dog success");
    Ok(())
}
