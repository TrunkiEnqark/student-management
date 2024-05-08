use std::io;

const INFI: usize = 1_000_000_000;
enum GenderState { Male, Female }
enum MajorState {
    SE,
    SS,
    SA
}
pub struct Birth { year: u16, month: u16, day: u16 }
pub struct Student {
    id: String,
    name: String,
    major: MajorState,
    birthday: Birth,
    gender: GenderState,
    address: String,
    phone_number: String,
    email: String
}

impl Birth {
    fn check_day(&self) -> bool {
        let is_leap_year: bool = (self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0));
        if self.day == 31 && (self.month == 1 || self.month == 3 || self.month == 5 || self.month == 7 || self.month == 8 || self.month == 10 || self.month == 12) {
            return true;
        }
        if self.day == 30 && (self.month == 4 || self.month == 6 || self.month == 9 || self.month == 11) {
            return true;
        }
        return (is_leap_year && self.day == 29) || (is_leap_year && self.day == 28);
    }

    pub fn enter_birthday() -> Birth {
        let mut temp: String = String::new();
        let yr = loop {
            print!("Enter your birth YEAR: ");
            loop {
                io::stdin().read_line(&mut temp).expect("Please press your Birth YEAR!");
                if temp.chars().all(char::is_numeric) {
                    break;
                }
                println!("Wrong input!");
            }
            let x: u16 = temp.parse().unwrap();
            break x;
        };

        let mon = loop {
            print!("Enter your birth MONTH: ");
            loop {
                io::stdin().read_line(&mut temp).expect("Please press your Birth MONTH!");
                if temp.chars().all(char::is_numeric) {
                    let num = temp.parse().unwrap();
                    if 1 <= num && num <= 12 {
                        break;
                    }
                    break;
                }
                println!("Wrong input!");
            }
            let x: u16 = temp.parse().unwrap();
            break x;
        };

        let dayy = loop {
            print!("Enter your birth DAY: ");
            loop {
                io::stdin().read_line(&mut temp).expect("Please press your Birth DAY!");
                if temp.chars().all(char::is_numeric) {
                    let num = temp.parse().unwrap();
                    let tmp: Birth = Birth {
                        year: yr,
                        month: mon,
                        day: num
                    };

                    if tmp.check_day() {
                        break;
                    }
                }
                println!("Wrong input!");
            }
            break temp.parse().unwrap();
        };

        Birth {
            year: yr,
            month: mon,
            day: dayy
        }
    }
}

impl Student {
    pub fn new_student() -> Student {
        let enter_name = Student::enter_name();
        let enter_birth = Birth::enter_birthday();
        let enter_major = Student::enter_major();
    }

    fn enter_name() -> String {
        let mut temp = String::new();
        let result = loop {
            print!("Enter your full name: ");
            io::stdin().read_line(&mut temp).expect("Please press your NAME!");
            if temp.chars().all(char::is_alphanumeric) {
                break temp;
            }
        };
        temp;
    }

    fn enter_major() -> String {
        println!("Choose your major: ");
        println!("1. SE");
        println!("2. SS");
        println!("3. SA");

    }
}