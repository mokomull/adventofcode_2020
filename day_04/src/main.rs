use prelude::*;

use regex::Regex;

fn main() {
    do_main("inputs/day_04.txt");
}

fn do_main(filename: &str) {
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut this = HashMap::new();
    for line in read_lines_from_file(filename) {
        if line == "" {
            passports.push(this);
            this = HashMap::new();
            continue;
        }

        for datum in line.split(' ') {
            let data: Vec<_> = datum.split(':').collect();
            this.insert(data[0].into(), data[1].into());
        }
    }

    if !this.is_empty() {
        passports.push(this);
    }

    let part1 = passports
        .iter()
        .filter(|&passport| is_valid_passport(passport))
        .count();
    dbg!(part1);
    assert_eq!(part1, 228);

    let part2 = passports
        .iter()
        .filter(|&passport| is_valid_passport_part2(passport))
        .count();
    dbg!(part2);
    assert_eq!(part2, 175);
}

fn is_valid_passport(passport: &HashMap<String, String>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|&k| passport.contains_key(k))
}

fn is_valid_passport_part2(passport: &HashMap<String, String>) -> bool {
    let byr_ok = integer_exists_between(&passport, "byr", 1920, 2002);
    let iyr_ok = integer_exists_between(&passport, "iyr", 2010, 2020);
    let eyr_ok = integer_exists_between(&passport, "eyr", 2020, 2030);
    let hgt_ok = passport
        .get("hgt")
        .and_then(|hgt| {
            if hgt.ends_with("cm") {
                hgt[..hgt.len() - 2]
                    .parse::<u32>()
                    .ok()
                    .map(|i| i >= 150 && i <= 193)
            } else if hgt.ends_with("in") {
                hgt[..hgt.len() - 2]
                    .parse::<u32>()
                    .ok()
                    .map(|i| i >= 59 && i <= 76)
            } else {
                None
            }
        })
        .unwrap_or(false);

    lazy_static::lazy_static! {
        static ref HCL_RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
        static ref ECL_RE: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
    };

    let hcl_ok = matches_regex(passport, "hcl", &HCL_RE);
    let ecl_ok = matches_regex(passport, "ecl", &ECL_RE);
    let pid_ok = matches_regex(passport, "pid", &PID_RE);

    byr_ok && iyr_ok && eyr_ok && hgt_ok && hcl_ok && ecl_ok && pid_ok
}

fn integer_exists_between(
    passport: &HashMap<String, String>,
    key: &str,
    lower: u32,
    upper: u32,
) -> bool {
    passport
        .get(key)
        .and_then(|i| i.parse::<u32>().ok())
        .map(|i| i >= lower && i <= upper)
        .unwrap_or(false)
}

fn matches_regex(passport: &HashMap<String, String>, key: &str, regex: &Regex) -> bool {
    passport
        .get(key)
        .map(|s| regex.is_match(s))
        .unwrap_or(false)
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::{convert_args, hashmap};

    #[test]
    fn valid_passports() {
        assert!(is_valid_passport(&convert_args!(hashmap!(
            "ecl" => "gry",
            "pid" => "860033327",
            "eyr" => "2020",
            "hcl" => "#f",
            "byr" => "1937",
            "iyr" => "2017",
            "cid" => "147",
            "hgt" => "183cm",
        ))));

        assert!(!is_valid_passport(&convert_args!(hashmap!(
            "iyr" => "2013",
            "ecl" => "amb",
            "cid" => "350",
            "eyr" => "2023",
            "pid" => "028048884",
            "hcl" => "#cfa07d",
            "byr" => "1929",
        ))));

        assert!(is_valid_passport(&convert_args!(hashmap!(
            "hcl" => "#ae17e1",
            "iyr" => "2013",
            "eyr" => "2024",
            "ecl" => "brn",
            "pid" => "760753108",
            "byr" => "1931",
            "hgt" => "179cm",
        ))));

        assert!(!is_valid_passport(&convert_args!(hashmap!(
            "hcl" =>"#cfa07d",
            "eyr" =>"2025",
            "pid" =>"166559648",
            "iyr" =>"2011",
            "ecl" =>"brn",
            "hgt" =>"59in",
        ))));
    }

    #[test]
    fn main() {
        super::do_main("../inputs/day_04.txt");
    }
}
