fn find_user<'a>(db: &'a Vec<String>, name: &str) -> Option<&'a String>{
    for user in db{
        if user == name{
            return Some(user);
        }
    }
    None
}

fn main(){
    let db = vec![
        String::from("Joe"),
        String::from("Alice")
    ];
    let user = find_user(&db, "Alice");
    println!("{:?}", user);
}