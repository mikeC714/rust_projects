use std::io;
use rand::rng;
use std::cmp::Ordering;


fn guessing_game(){
    'guessing_loop:loop{
        let secret_num = rand::thread_rng().gen_range(1..=100);
        println!("Guess a number!");
        println!("Please input a number!");
       
        let mut guess = String::new();

       io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guessed_num: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        
        match guessed_num.cmp(&secret_num){
            Ordering::Less | Ordering::Greater => println!("Good attempt the answer was {secret_num}, not {guessed_num}"),
            Ordering::Equal => {
                println!("Congrats you're correct the number was {guessed_num}");
                break 'guessing_loop;
            }
        } 
    }
}

