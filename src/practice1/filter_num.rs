use rand::Rng;

fn main(){
    let numbers: Vec<i32> = (0..10)
        .map(|_| rand::thread_rng().gen_range(0..=100))
        .collect();

    let mut low_vals = Vec::new();
    let mut hi_vals = Vec::new();

    for num in numbers{
        if num % 2 == 0 || num % 6 == 0{
            low_vals.push(num);
        }else{
            hi_vals.push(num);
        }
    }

    println!("{:?}", low_vals);
    println!("{:?}", hi_vals);
}

