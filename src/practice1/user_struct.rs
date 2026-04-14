use std::io;
use std::io::Write;

#[derive(Debug)]
struct User{
    index: usize,
    name: String,
    username: String,
}

// Creating a Type to limit how verbose Result syntax, since it's being reused
type Result<T> = Result<T, Box<dyn std::error::Error>>;

fn get_input(prompt: &str) -> Result<String>{
    println!("{}", prompt);

    // Error propagation "?"  instead of using match or unwrap.
    // Error propagation when Ok<T> returns the unwrapped value.
    // Whereas if Err it short curcuits the function possibly crashing application.
    io::stdout()
        .flush()?;

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)?;

    return Ok(input.trim().to_string());
}


fn main() -> Result<()>{
    let mut db: Vec<User> = Vec::new();
    loop{
        let name = get_input("Please Input Your Name.")?;
        let username = get_input("Please Input Your Username")?; 

        if name == "stop" || username == "stop"{
            break
        };

        db.push(User{
            index: db.len() +1,
            name,
            username
        });

        println!("{:#?}", db)
    }
    Ok(())
}
