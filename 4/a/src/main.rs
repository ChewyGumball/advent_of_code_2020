use std::env;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

fn parse_passport(lines: &mut std::iter::Peekable<Lines<io::BufReader<File>>>) -> Passport {
    let mut passport =  Passport {
        birth_year: None,
        issue_year: None,
        expiration_year: None,
        height: None,
        hair_colour: None,
        eye_colour: None,
        passport_id: None,
        country_id: None,
    };
    
    let passport_lines: Vec<String> = lines.by_ref().take_while(|line| !line.as_ref().unwrap().is_empty()).collect::<Result<_,_>>().unwrap();
    let passport_line = passport_lines.join(" ");

    for field in passport_line.split(' ') {
        let (field_type, value) = field.split_at(3);
        match field_type {
            "byr" => passport.birth_year = Some(String::from(value)),
            "iyr" => passport.issue_year = Some(String::from(value)),
            "eyr" => passport.expiration_year = Some(String::from(value)),
            "hgt" => passport.height = Some(String::from(value)),
            "hcl" => passport.hair_colour = Some(String::from(value)),
            "ecl" => passport.eye_colour = Some(String::from(value)),
            "pid" => passport.passport_id = Some(String::from(value)),
            "cid" => passport.country_id = Some(String::from(value)),
            _ => panic!("Unknown field type {}", field_type)
        }
    }

    return passport;
}

fn parse_file(file_name: &Path) -> Vec<Passport> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file,
    };

    let mut passports: Vec<Passport> = Vec::new();

    let mut lines = io::BufReader::new(file).lines().peekable();
    while lines.peek().is_some() {
        passports.push(parse_passport(&mut lines));
    }

    return passports;
}

fn is_valid_passport(passport: &Passport) -> bool {
    return passport.birth_year.is_some()
        && passport.issue_year.is_some()
        && passport.expiration_year.is_some()
        && passport.height.is_some()
        && passport.hair_colour.is_some() & passport.eye_colour.is_some()
        && passport.passport_id.is_some();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let passports = parse_file(&input_file);
    let valid_count = passports.iter().filter(|passport| is_valid_passport(*passport)).count();
    print!("Valid Passports: {}", valid_count);
}
