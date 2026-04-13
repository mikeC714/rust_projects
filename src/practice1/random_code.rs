use std::io;
use rand::Rng;


fn gen_num(low: u32, high: u32) -> (Vec<String>, Vec<String>){
    let mut vec: Vec<String> = Vec::new();
    loop{

        let mut rng = rand::thread_rng();
        let numbers: u32 = rng.gen_range(low..=high);
        let six_digit_codes= format!("{:06}", numbers);

        vec.push(six_digit_codes);

        if vec.len() == 400 {
            let mut batches: Vec<Vec<String>> = vec
                .chunks(200)
                .map(|chunk| chunk.to_vec())
                .collect();
            
            let batch1 = batches.pop().unwrap();
            let batch2 = batches.pop().unwrap();

            return(batch1, batch2);
        }
    }
}



fn main(){
    println!("Please Input the Low Range Value");
    println!("Please Input the High Range Value");
    loop{
        let mut low = String::new();
        let mut high = String::new();

         io::stdin()
            .read_line(&mut low)
            .expect("Failed to read low value");    
        io::stdin()
            .read_line(&mut high)
            .expect("Failed to read high value");
            
        let guessed_high: u32 = match high.trim().parse(){
            Ok(hi) => hi,
            Err(_) => continue
        };
        let guessed_low: u32 = match low.trim().parse(){
            Ok(low) => low,
            Err(_) => continue
        };

        let (batch1, batch2) = gen_num(guessed_low, guessed_high);
        println!("Batch1: {:#?}", batch1);
        println!("Batch2: {:#?}", batch2);
        break;

    }
}

