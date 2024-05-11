use std::{f32::consts::PI, io};
use rand::Rng;

// const INFI: usize = 1_000_000_000;
const MAX_STUDENT_ID: usize = 999_999;

#[derive(Debug, Clone)]
pub enum GenderState { Male, Female, Sexless }

#[derive(Clone, Copy, Debug)]
pub enum MajorState { SE, SS, SA, Nothing }

#[derive(Debug, Clone)]
pub struct Birth { year: u16, month: u16, day: u16 }

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    major: MajorState,
    id: String,
    birthday: Birth,
    gender: GenderState,
    address: String,
    phone_number: String,
    email: String
}

impl Birth {
    fn leap_year(yy: u16) -> bool {
        yy % 4 == 0 && (yy % 100 != 0 || yy % 400 == 0)
    }

    fn check_day(&self) -> bool {
        let is_leap_year: bool = Birth::leap_year(self.year);
        if (1 <= self.day && self.day <= 31) && (self.month == 1 || self.month == 3 || self.month == 5 || self.month == 7 || self.month == 8 || self.month == 10 || self.month == 12) {
            return true;
        }
        if (1 <= self.day && self.day <= 30) && (self.month == 4 || self.month == 6 || self.month == 9 || self.month == 11) {
            return true;
        }
        return (is_leap_year && 1 <= self.day && self.day <= 29) || (!is_leap_year && 1 <= self.day && self.day <= 28);
    }

    fn birthday_maker() -> Birth {
        let yy: u16 = rand::thread_rng().gen_range(1900..=2024);
        let mm: u16 = rand::thread_rng().gen_range(1..=12);
        let mut dd = 10;
        if mm == 1 || mm == 3 || mm == 5 || mm == 7 || mm == 8 || mm == 10 || mm == 12 {
            dd = rand::thread_rng().gen_range(1..=31);
        } else if mm == 1 || mm == 3 || mm == 5 || mm == 7 || mm == 8 || mm == 10 || mm == 12 {
            dd = rand::thread_rng().gen_range(1..=30);
        } else if mm == 2 {
            if Birth::leap_year(yy) {
                dd = rand::thread_rng().gen_range(1..=29);
            } else {
                dd = rand::thread_rng().gen_range(1..=28);
            }
        }

        Birth {
            year: yy,
            month: mm,
            day: dd
        }
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
        let enter_name = Student::enter_name();
        let enter_birth = Birth::enter_birthday();
        let enter_major = Student::enter_major();
        let enter_gender = Student::enter_gender();
        let enter_address = Student::enter_address();
        let enter_phone_number = Student::enter_phone_number();
        let enter_email = Student::enter_email();
        let enter_id = Student::make_student_id(enter_major.clone());
        return Student {
            name: enter_name,
            major: enter_major,
            id: enter_id,
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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_major(&self) -> String {
        match self.major {
            MajorState::SA => "SA".to_string(),
            MajorState::SE => "SE".to_string(),
            MajorState::SS => "SS".to_string(),
            MajorState::Nothing => "Nothing".to_string()
        }
    }

    pub fn get_birthday(&self) -> String {
        let mut dd = self.birthday.day.clone().to_string();
        let mut mm = self.birthday.month.clone().to_string();
        let yy = self.birthday.year.clone().to_string();
        if dd.len() == 1 {
            dd = "0".to_string() + &dd;
        }
        if mm.len() == 1 {
            mm = "0".to_string() + &mm;
        }
        let result = dd + "/" + &mm + "/" + &yy;
        result
    }

    pub fn get_gender(&self) -> String {
        match self.gender {
            GenderState::Male => "Male".to_string(),
            GenderState::Female => "Female".to_string(),
            GenderState::Sexless => "Sexless".to_string()
        }
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn get_phone_number(&self) -> String {
        self.phone_number.clone().to_string()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn student_auto_maker() -> Student {
        let major_temp = Student::major_random();

        Student {
            name: Student::string_random(),
            major: major_temp,
            id: Student::make_student_id(major_temp),
            birthday: Birth::birthday_maker(),
            gender: Student::gender_random(),
            address: Student::string_random(),
            phone_number: Student::phone_random(),
            email: Student::string_random()
        }
    }

    fn string_random() -> String {
        let mut result = String::new();
        for _ in 0..10 {
            let number_random = rand::thread_rng().gen_range('a'..='z');
            result += &number_random.clone().to_string();
        }
        result
    }

    fn major_random() -> MajorState {
        match rand::thread_rng().gen_range(0..=2) {
            0 => MajorState::SE,
            1 => MajorState::SA,
            2 => MajorState::SS,
            _ => MajorState::Nothing
        }
    }

    fn gender_random() -> GenderState {
        match rand::thread_rng().gen_range(0..=1) {
            0 => GenderState::Male,
            1 => GenderState::Female,
            _ => GenderState::Sexless,
        }
    }

    fn phone_random() -> String {
        let mut result = String::new();
        for _ in 0..10 {
            let number_random = rand::thread_rng().gen_range(0..=9);
            result += &number_random.clone().to_string();
        }
        result
    }

    fn enter_name() -> String {
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

    fn enter_major() -> MajorState {
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

    fn enter_gender() -> GenderState {
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
            _ => GenderState::Sexless,
        }
    }

    fn enter_address() -> String {
        let mut temp = String::new();
        println!("Press your address: ");
        io::stdin().read_line(&mut temp).expect("Wrong input!");
        temp.retain(|c| !c.is_whitespace());
        return temp;
    }

    fn enter_phone_number() -> String {
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

    fn enter_email() -> String {
        let mut temp = String::new();
        println!("Press your Email: ");
        io::stdin().read_line(&mut temp).expect("Wrong input!");
        temp.retain(|c| !c.is_whitespace());
        temp
    }

    fn make_student_id(major: MajorState) -> String {
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