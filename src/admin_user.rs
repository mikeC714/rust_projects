use std::io;
use std::io::Write;

#[derive(Debug)]
struct Admin {
    username: String,
    password: Vec<u8>
}



type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


fn encrypt(pass: &str) -> Result<Vec<u8>>{
    return Ok(pass.as_bytes().to_vec());
}

fn decrypt(pass: &Vec<u8>) -> Result<String>{
    let password = String::from_utf8(pass.to_vec())?;
    return Ok(password);
}


fn get_input(prompt: &str) -> Result<String>{
    println!("{}", prompt);

    io::stdout()
        .flush()?;

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)?;

    return Ok(input.trim().to_string());
}

fn required_input(prompt: &str) -> Result<String>{
    loop{
        let input = get_input(prompt)?;

        if !input.is_empty(){
            return Ok(input);
        }

        println!("{}", prompt);
    }
}

fn main() -> Result<()>{
    let mut admin_db: Vec<Admin> = Vec::new();
    let admin_name = get_input("Enter Admin Name")?;


    let admin_name = match admin_name.is_empty(){
        true => String::from("admin"),
        false => admin_name
    };

    let pass = required_input("Enter Password")?;
    let encrypt_pass = encrypt(&pass)?;
    let decrypted = decrypt(&encrypt_pass)?;

    admin_db.push(Admin{
         username: admin_name,
         password: encrypt_pass
     });


    println!("{:?}", admin_db);
    println!("{:?}", decrypted);
    Ok(())
}