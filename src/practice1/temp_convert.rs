use std::io;

fn main(){
    loop{
        println!("Once temp is entered enter a scale");
        println!("Please enter the temp");

        let mut scale = String::new();
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temp");
        io::stdin()
            .read_line(&mut scale).unwrap();

        let temp: f32 = match temp.trim().parse(){
            Ok(temp) => temp,
            Err(_) => {return}
        };
        let scale: Option<char> = scale.trim().chars().next();

        match scale{
            Some('F') => println!("Converted {temp}°F to °C is {:.2}", (temp-32.0) * 5.0/9.0),
            Some('C') => println!("Converted {temp}°C to °F is {:.2}", (temp*9.0/5.0) + 32.0),
            None => println!("Invalid scale type. Please select F (Farenheit) or C (Celcius)"),
            _ => {return}
        };
        break;
    }
}