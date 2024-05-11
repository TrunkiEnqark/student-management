use std::{fs::File, io::Write};

use information::Student;
mod information;

fn main() {
    // let news = Student::student_auto_maker();
    // println!("{:?}", news);
    let mut w = File::create("./test.txt").unwrap();
    for i in 0..100 {
        let new_stu = Student::student_auto_maker();
        writeln!(w, "{} | {} | {} | {} | {} | {} | {} | {}", 
            new_stu.get_id(),
            new_stu.get_name(),
            new_stu.get_gender(),
            new_stu.get_birthday(),
            new_stu.get_phone_number(),
            new_stu.get_email(),    
            new_stu.get_major(),
            new_stu.get_address()
        ).unwrap();
    }
}
