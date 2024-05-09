use std::io;
// use crate::information::Student;

use information::{
    Student,
    Birth,
    MajorState,
    GenderState
};
mod information;

fn main() {
    println!("{:?}", Student::enter_gender());
}
