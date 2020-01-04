use std::io;
use std::string::String;

fn main() {
        println!("convert temperatures!");
    
    loop {
        println!("how many degrees?");
        // get user supplied degrees
        let mut temp = String::new();
        io::stdin().read_line(&mut temp)
            .expect("failed to read line");
    
        // convert temp to int
        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("what unit? please enter F or C");
        // get user supplied unit
        let mut unit = String::new();
        io::stdin().read_line(&mut unit)
            .expect("failed to read line");
    
    
        println!("converting {} degrees {}", temp, unit);
        // match against unit
        match unit.trim().to_lowercase() {
            "f" => 
            { println!("that's {} degrees {}", to_c(temp), unit);
                break},
            "c" => 
            { println!("that's {} degrees {}", to_f(temp), unit);
                break},
            _ => continue,
        }

    }
}

fn to_c(f: i32) -> i32 {
    // (32°F − 32) × 5/9 = 0°C
    (f - 32) * 5/9
}

fn to_f(c: i32) -> i32 {
    // (0°C × 9/5) + 32 = 32°F
    (c * 9/5) + 32
}
