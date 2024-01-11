use std::io;

fn main() {
let unit = get_unit_type();

if unit == "Invalid" {
    return;
}
else {
    println!("You have chosen to convert from {unit}");
    convert_temp(unit);
}


}

fn get_unit_type() -> &'static str  {
    let mut count = 0;
    let mut unit = String::new();
    while count < 3 {
        println!("Please enter the unit type you would like to convert from. Type 'c' for Celsius or 'f' for Fahrenheit");
        io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

        let mut unit = unit.trim().to_lowercase();

        match validate_unit_type(&unit) {
            Ok(valid_unit) => return valid_unit,
            Err(message) => {
                println!("{}", message);
                println!("You have {} attempts remaining", 2 - count);
                count += 1;
                unit.clear();
            }
        };
     
    }
   return "Invalid"; 
}
fn validate_unit_type(unit: &str) -> Result<&'static str, &'static str> {
    match unit.to_lowercase().as_str() {
        "c" => Ok("Celsius"),
        "f" => Ok("Fahrenheit"),
        _ => Err("Please enter a valid unit type"),
    }
}


fn convert_temp(unit: &str){
    let mut count = 0;

    println!("Please enter the temperature you would like to convert");

    while count < 3 {
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
       match unit {
        // Temperature in degrees Celsius (°C) = (Temperature in degrees Fahrenheit (°F) - 32) * 5/9   
            "Celsius" => {
                let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
                println!("{} degrees Celsius is {} degrees Fahrenheit", temp, fahrenheit);
                return;
            }
        // Temperature in degrees Fahrenheit (°F) = (Temperature in degrees Celsius (°C) * 9/5) + 32
            "Fahrenheit" => {
                let celsius = (temp - 32.0) * 5.0 / 9.0;
                println!("{} degrees Fahrenheit is {} degrees Celsius", temp, celsius);
                return;
            }
            _ => {
                println!("Invalid unit type");
                return;
            }
        }
    }
  
}







