use std::io;

fn eval(expr: &str) -> f64{
    let body : Vec<&str> = expr.trim().split_whitespace().collect();
    if body.len() != 3 {return 0.0};

    let num1 = body[0].trim().parse::<f64>().unwrap();
    let op = body[1].trim();
    let num2 = body[2].trim().parse::<f64>().unwrap();

    match op{
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => 0.0,
    }
}


fn main(){
    println!("Start Your Calculation!");
    loop{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let results = eval(&input);
        println!("The answer is {results}");
    }   
}
