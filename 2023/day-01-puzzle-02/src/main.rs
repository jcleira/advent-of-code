use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = match File::open("input-02.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut total = 0;

    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();

        let mut first_digit: u32 = 0;
        let mut final_digit: u32 = 0;
        let mut first_digit_found = false;
        let mut final_digit_found = false;

        let mut word_number = String::new();

        for mut ch in line.chars() {
            if ch.is_alphabetic() {
                word_number.push(ch);

                match string_to_number(&word_number) {
                    Ok(number) => {
                        ch = (b'0' + number as u8) as char;

                        word_number = (&word_number[word_number.len() - 1..]).to_string();
                    }
                    Err(_error) => {
                        continue;
                    }
                }
            }

            if !first_digit_found {
                first_digit = ch.to_digit(10).unwrap();
                first_digit_found = true;
                continue;
            }

            final_digit_found = true;
            final_digit = ch.to_digit(10).unwrap();
        }

        if !final_digit_found {
            final_digit = first_digit;
        }

        total += (first_digit * 10) + final_digit;

        println!("{}", line);
        println!("{}{}", first_digit, final_digit);
    }

    println!("Total: {}", total);
}

fn string_to_number(s: &str) -> Result<i32, String> {
    if s.contains("zero") {
        Ok(0)
    } else if s.contains("one") {
        Ok(1)
    } else if s.contains("two") {
        Ok(2)
    } else if s.contains("three") {
        Ok(3)
    } else if s.contains("four") {
        Ok(4)
    } else if s.contains("five") {
        Ok(5)
    } else if s.contains("six") {
        Ok(6)
    } else if s.contains("seven") {
        Ok(7)
    } else if s.contains("eight") {
        Ok(8)
    } else if s.contains("nine") {
        Ok(9)
    } else {
        Err(format!("'{}' does not contain a valid number word", s))
    }
}
