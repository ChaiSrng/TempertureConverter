use std::io;

fn main() {
    let mut input_choice = String::new();
    let input_value: u32;
    loop {
        println!("To convert fahrenheit to celsius, press 1. For the other way press 2");

        io::stdin()
            .read_line(&mut input_choice)
            .expect("Unable to readline");

        let input_choice: u32 = match input_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number input");
                continue;
            }
        };
        if input_choice == 1 || input_choice == 2 {
            input_value = input_choice;
            break;
        } else {
            println!("Provide 1 or 2 as input");
        }
    }
    if input_value == 1 {
        loop {
            println!("Provide fahrenheit value");

            let mut fahrent = String::new();

            io::stdin()
                .read_line(&mut fahrent)
                .expect("Failed to readline");

            let fahrent: f32 = match fahrent.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please provide input number");
                    continue;
                }
            };

            println!("Your input is {fahrent}");

            let celsi: f32 = ((fahrent - 32.00) * 5.00) / 9.00;
            println!("celsi is {celsi}");
            break;
        }
    } else if input_value == 2 {
        loop {
            println!("Provide Degree Celsius value");

            let mut celsius = String::new();

            io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to readline");

            let celsius: f32 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please provide input number");
                    continue;
                }
            };

            println!("Your input is {celsius}");
            let fahrent: f32 = (celsius * (9.00 / 5.00)) + 32.00;
            println!("Fahrenheit is {fahrent}");
            break;
        }
    }
}
