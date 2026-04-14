use std::io;
use std::io::Write;


#[derive(Debug)]
struct Admin{
    name: String,
    pass: String
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get_input(prompt: &str) -> Result<String>{
    println!("{}", prompt);

    io::stdout()
        .flush()?;

    let mut input: String = String::new(); 
    io::stdin()
        .read_line(&mut input)?;

    Ok(input.trim().to_string())
}


fn required_input(prompt: &str ) -> Result<String>{
    loop{
        let input = get_input(prompt)?;

        if !input.is_empty(){
            return Ok(input)
        }

        println!("Please input a password");
    }
}

fn main() -> Result<()>{
    let mut admin_db: Vec<Admin> = Vec::new();
    let admin_name = get_input("Enter Admin Name.")?;

    let admin_name = match admin_name.is_empty(){
        true => String::from("admin"),
        false => admin_name
    };

    let pass = required_input("Please Enter A Password")?;

    admin_db.push(Admin{
        name: admin_name,
        pass
    });

    println!("{:#?}", admin_db);
    Ok(())
}