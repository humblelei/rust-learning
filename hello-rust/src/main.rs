//#![feature(box_syntax)]
fn main() {
    let my_box = box 5;
    print!("{}",my_box);
    // let student = Student{
    //     name: "李寻欢".to_string()
    // };

    // println!("{:#?}", student);
}

use std::fmt::{Debug, Formatter, Result};

struct Student{
    name: String
}

impl Debug for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Student")
         .field("name", &self.name)
         .finish()
    }
}