use std::io;

fn main() {
let unit = unit_type();

if unit == "Invalid" {
    return;
}
else {
    println!("You have chosen to convert from {}", unit);
    convert_temp(unit);
}
// Temperature in degrees Fahrenheit (°F) = (Temperature in degrees Celsius (°C) * 9/5) + 32
// Temperature in degrees Celsius (°C) = (Temperature in degrees Fahrenheit (°F) - 32) * 5/9   
}

fn unit_type() -> &'static str  {
    let mut count = 0;
    let mut unit = String::new();
    loop {
        if count == 3 {
            println!("You have exceeded the number of attempts");
            break "Invalid";
        }
        println!("Please enter the unit type you would like to convert from. Type 'c' for Celsius or 'f' for Fahrenheit");
        io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

        let unit = match unit.trim().to_lowercase().as_str() {
            "c" => "Celsius",
            "f" => "Fahrenheit",
            _ => {
                println!("Please enter a valid unit type");
                println!("You have {} attempts remaining", 2 - count);
                count += 1;
                continue;
              
            }
        };
        return unit;
    }    
}
fn convert_temp(unit: &str){
    let mut count = 0;
    println!("Please enter the temperature you would like to convert");
    loop {
        if count == 3 {
            println!("You have exceeded the number of attempts");
            break;
        }
        let mut temp = String::new();
        
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                println!("You have {} attempts remaining", 2 - count);
                count += 1;
                continue;
            }
        };
        if unit == "Celsius" {
            let fahrenheit = (temp * 9.0/5.0) + 32.0;
            return println!("{} degrees Celsius is {} degrees Fahrenheit", temp, fahrenheit);
        } else if unit == "Fahrenheit" {
            let celsius = (temp - 32.0) * 5.0/9.0;
            return println!("{} degrees Fahrenheit is {} degrees Celsius", temp, celsius);
        } else {
            return println!("Invalid unit type");
        }
    }
  
}







