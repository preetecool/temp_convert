### Building a simple program to convert temperature units

#### main() function:

```
fn main() { let unit = unit_type();

if unit == "Invalid" { return; } else { println!("You have chosen to convert from {}",
unit); convert_temp(unit); } }
```

#### unit_type() function: This function expects an input and returns the selected unit if given a valid character. If the character input by the user is not 'f' or 'c', count goes up. If count reaches 3 (starting from 0) the program exits.

```
fn unit_type() -> &'static str  {
    let mut count = 0;
    let mut unit = String::new();
    while count < 3 {
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
    return "Invalid";
}

```

#### convert_temp() function: Expects a unit as parameter and expects user input for temperature (number). It also has error handlign and gives the user 3 tries before exiting. The result is returned.

```
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
```

#### Retrospect: The validation in the unit_type() and convert_temp() functions could have been extracted into a seperate function. In addition, the convert_temp() should have a return value in its declaration.
