use std::time::{UNIX_EPOCH, SystemTime};
use regex::Regex;

pub fn get_time() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("can't get time!")
    }
}

pub fn parse_time_string(time: &String) -> Result<u64, ()> {
    let re = Regex::new(r"^([0-9]+h)?([0-9]+m)?([0-9]+s)?$").unwrap();

    if !re.is_match(time) {
        return Err(())
    }

    let mut current_num: Vec<char> = vec![];
    let mut current_seconds: u64   = 0;

    // custom parser for time strings
    for ch in time.chars() {
        if ch >= '0' && ch <= '9' {
            // if we know we have a char, then add it to the string
            current_num.push(ch);
        } else {
            // convert to a u64 if we have another specifier
            let s: String = current_num.iter().collect();
            let mut num: u64 = s.parse::<u64>().unwrap();

            // scale based on time factor
            match ch {
                'h' => num *= 60*60,
                'm' => num *= 60,
                's' => (),
                _   => ()
            }

            // add to the total count
            current_seconds += num;

            // clear the vec
            current_num.clear();

        }
    }

    // println!("parsed {} to {} seconds", time, current_seconds);

    return Ok(current_seconds); 
}

pub fn create_time_string(time: u64) -> String {
    let mut output: String = "".to_string();

    let mut time_f = time as f64;

    let hours = (time_f / (60.0*60.0)).floor();
    if hours > 0.0 {
        output = format!("{}{}h", output, hours)
    }
    time_f -= hours * (60.0*60.0);

    let mins = (time_f / 60.0).floor();
    if mins > 0.0 {
        output = format!("{}{}m", output, mins)
    }
    time_f -= mins.floor() * 60.0;

    let secs = time_f.floor();
    if secs != 0.0 {
        output = format!("{}{}s", output, secs);
    }

    // println!("converted {} to {}", time, output);

    return output;
}
