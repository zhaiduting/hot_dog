fn main() {}

trait DataSource {
    type Item;

    fn get(&self) -> Self::Item;
}

struct MyDb;

impl DataSource for MyDb {
    type Item = u32;

    fn get(&self) -> Self::Item {
        42
    }
}
// impl DataSource for MyDb {
//     type Item = i32;
// 
//     fn get(&self) -> Self::Item {
//         42
//     }
// }