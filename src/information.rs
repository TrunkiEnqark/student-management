use std::io;
use rand::Rng;

// const INFI: usize = 1_000_000_000;
const MAX_STUDENT_ID: usize = 999_999;

#[derive(Debug, Clone)]
pub enum GenderState { Male, Female, Sexless, Nothing }

#[derive(Clone, Copy, Debug)]
pub enum MajorState {
    SE,
    SS,
    SA,
    Nothing
}

#[derive(Debug, Clone)]
pub struct Birth { year: u16, month: u16, day: u16 }

#[derive(Debug, Clone)]
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
        let is_leap_year: bool = self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0);
        if (1 <= self.day && self.day <= 31) && (self.month == 1 || self.month == 3 || self.month == 5 || self.month == 7 || self.month == 8 || self.month == 10 || self.month == 12) {
            return true;
        }
        if (1 <= self.day && self.day <= 30) && (self.month == 4 || self.month == 6 || self.month == 9 || self.month == 11) {
            return true;
        }
        return (is_leap_year && self.day == 29) || (is_leap_year && self.day == 28);
    }

    pub fn enter_birthday() -> Birth {
        let yr: u16 = loop {
            let mut temp: String = String::new();
            println!("Enter your birth YEAR: ");
            io::stdin().read_line(&mut temp).expect("Please press your Birth YEAR!");
            temp.retain(|c| !c.is_whitespace());
            if !temp.chars().any(|c| matches!(c, 'a'..= 'z')) {
                break temp.parse().unwrap();
            }
            println!("Wrong input!");
        };

        let mon: u16 = loop {
            let mut temp: String = String::new();
            println!("Enter your birth MONTH: ");
            io::stdin().read_line(&mut temp).expect("Please press your Birth MONTH!");
            temp.retain(|c| !c.is_whitespace());
            if !temp.chars().any(|c| matches!(c, 'a'..= 'z')) {
                let num: u16 = temp.parse().unwrap();
                if 1 <= num && num <= 12 {
                    break num;
                }
            }
            println!("Wrong input!");
        };

        let dayy: u16 = loop {
            let mut temp: String = String::new();
            println!("Enter your birth DAY: ");
            io::stdin().read_line(&mut temp).expect("Please press your Birth DAY!");
            temp.retain(|c| !c.is_whitespace());
            if !temp.chars().any(|x| matches!(x, 'a' ..= 'z')) {
                let num = temp.parse().unwrap();
                let tmp: Birth = Birth {
                    year: yr,
                    month: mon,
                    day: num
                };

                if tmp.check_day() {
                    break num;
                }
            }
            println!("Wrong input!");
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
        // println!("BUG!");
        let enter_name = Student::enter_name();
        // println!("BUG!");
        let enter_birth = Birth::enter_birthday();
        let enter_major = Student::enter_major();
        let enter_gender = Student::enter_gender();
        let enter_address = Student::enter_address();
        let enter_phone_number = Student::enter_phone_number();
        let enter_email = Student::enter_email();
        let enter_id = Student::make_student_id(enter_major.clone());
        return Student {
            name: enter_name,
            id: enter_id,
            major: enter_major,
            birthday: enter_birth,
            gender: enter_gender,
            address: enter_address,
            phone_number: enter_phone_number,
            email: enter_email
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    // pub fn get_name(&self) -> String {
    //     self.name.clone()
    // }

    pub fn enter_name() -> String {
        let result = loop {
            let mut temp = String::new();
            println!("Enter your full name: ");
            io::stdin().read_line(&mut temp).expect("Please press your NAME!");
            temp.retain(|c| !c.is_whitespace());
            if !temp.chars().any(|c| matches!(c, '0'..='9')) {
                break temp;
            }
        };
        return result;
    }

    pub fn enter_major() -> MajorState {
        println!("Major list: ");
        println!("1. SE");
        println!("2. SS");
        println!("3. SA");
        let num = loop {
            println!("Choose your major: ");
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Input Error");
            temp.retain(|c| !c.is_whitespace());
            let x: i32 = temp.parse().unwrap();
            if x == 1 || x == 2 || x == 3 {
                break x;
            }
            println!("Wrong input! Please press one from 1 to 3: ");
        };

        match num { 
            1 => MajorState::SE,
            2 => MajorState::SS,
            3 => MajorState::SA,
            _ => MajorState::Nothing
        }
    }

    pub fn enter_gender() -> GenderState {
        println!("Gender: ");
        println!("1. Male");
        println!("2. Female");
        println!("3. Sexless");
        let mut tmp = String::new();
        let num = loop {
            println!("Choose your gender: ");
            io::stdin().read_line(&mut tmp).expect("Input Error");
            tmp.retain(|c| !c.is_whitespace());
            let x: i32 = tmp.parse().unwrap();
            if x == 1 || x == 2 || x == 3 {
                break x;
            }
            println!("Wrong input! Please press one from 1 to 3: ");
        };

        match num { 
            1 => GenderState::Male,
            2 => GenderState::Female,
            3 => GenderState::Sexless,
            _ => GenderState::Nothing
        }
    }

    pub fn enter_address() -> String {
        let mut temp = String::new();
        println!("Press your address: ");
        io::stdin().read_line(&mut temp).expect("Wrong input!");
        temp.retain(|c| !c.is_whitespace());
        return temp;
    }

    pub fn enter_phone_number() -> String {
        let result = loop {
            let mut temp = String::new();
            println!("Enter your phone number: ");
            io::stdin().read_line(&mut temp).expect("Please press your NAME!");
            temp.retain(|c| !c.is_whitespace());
            if !temp.chars().any(|b| matches!(b, 'a'..='z')) {
                break temp;
            }
        };
        return result;
    }

    pub fn enter_email() -> String {
        let mut temp = String::new();
        println!("Press your Email: ");
        io::stdin().read_line(&mut temp).expect("Wrong input!");
        temp.retain(|c| !c.is_whitespace());
        temp
    }

    pub fn make_student_id(major: MajorState) -> String {
        let maj = match major {
            MajorState::SA => "SA",
            MajorState::SE => "SE",
            MajorState::SS => "SS",
            MajorState::Nothing => "??" 
        };
        let num = rand::thread_rng().gen_range(0 ..= MAX_STUDENT_ID);
        let result = maj.to_owned() + num.to_string().as_str();
        return result;
    }
}