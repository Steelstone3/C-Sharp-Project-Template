pub mod presenters {
    use std::string::String;
    use std::io;

    pub fn write_message(message: String) {
        println!("{}", message);
    }

    pub fn read_numeric_i32(message: &str, lower_bound: i32, upper_bound: i32) -> i32 {
        let mut result = -1;

        while result == -1
            || result > upper_bound
            || result < lower_bound {
            let input = read_string(message);

            result = match input.as_str().trim().parse::<i32>() {
                Ok(result) => result,
                Err(_e) => -1,
            };
        }

        return result;
    }

    pub fn read_boolean(message: &str) -> bool {
        let input = read_string(message);

        return if input.eq_ignore_ascii_case("yes") || input.eq_ignore_ascii_case("y") {
            true
        } else {
            false
        }
    }

    pub fn read_string(message: &str) -> String {
        let mut input = String::new();

        println!("{}", message);

        match io::stdin().read_line(&mut input) {
            Ok(_) => print!(""),
            Err(_e) => println!("{}", _e),
        };

        return input;
    }
}
