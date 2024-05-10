use std::io;
// use crate::information::Student;

use information::{Student, Birth};
mod information;

fn main() {
    let new_stu = Student::new_student();
    println!("{:?}", new_stu);
}
