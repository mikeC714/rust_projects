use std::io;
use std::io::Write;

struct User <'a> {
    name: &'a str,
    username: &'a str,
    email: &'a str
}


fn get_input(prompt: &str) -> String{
    println!("{}", prompt);

    io::stdout()
        .flush()
        .unwrap();
    
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    return input.trim().to_string();
}


fn main(){
    let name = get_input("Please Enter Name.");
    let email = get_input("Please Enter Email");
    let username = get_input("Please Enter A Username");

    let user1 = User{
        name: &name,
        email: &email,
        username: &username
    };

    println!{"{:#?}", user1.name};
    println!{"{:#?}", user1.username};
    println!{"{:#?}", user1.email};
}


