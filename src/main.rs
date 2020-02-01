use std::io;
use std::string::String;

fn main() {
    println!("convert temperatures!");

    let temp = loop {
        println!("how many degrees?");
        // get user supplied degrees
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("failed to read line");

        // convert temp to int
        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break temp;
    };

    let unit = loop {
        println!("what unit? please enter F or C");
        // get user supplied unit
        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("failed to read line");

        // make sure unit is uppercase and "clean"
        let unit = match unit.as_str().to_uppercase().trim() {
            "F" => "F",
            "C" => "C",
            _ => continue,
        };
        break unit;
    };

    println!("converting {} degrees {}", temp, unit);
    loop {
        // match against unit
        match unit {
            "F" => {
                println!("that's {} degrees C", to_c(temp));
                break;
            }
            "C" => {
                println!("that's {} degrees F", to_f(temp));
                break;
            }
            _ => {
                println!("exiting");
                break;
            }
        }
    }
}

fn to_c(f: i32) -> i32 {
    // convert F to C
    // (32°F − 32) × 5/9 = 0°C
    (f - 32) * 5 / 9
}

fn to_f(c: i32) -> i32 {
    // convert C to F
    // (0°C × 9/5) + 32 = 32°F
    (c * 9 / 5) + 32
}
