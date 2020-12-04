use maplit::{convert_args, hashmap};

use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    do_main("inputs/day_04.txt");
}

fn do_main(filename: &str) {
    let file = std::fs::File::open(filename).expect("could not open the input");
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut this = HashMap::new();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.expect("could not read line");

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
}

fn is_valid_passport(passport: &HashMap<String, String>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|&k| passport.contains_key(k))
}

#[cfg(test)]
mod test {
    use super::*;

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
}
