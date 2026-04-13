

fn first_letter(string: &String) -> &str{
    let bytes = string.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &string[0..i];
        }
    }
    return &string[..]
}


fn main(){
    let string: String = String::from("HelloWorld");
    let res = first_letter(&string);
    println!("{}", res);
}