fn arrange_nums(rng: &[i32]) -> (Vec<i32>, Vec<i32>, Vec<i32>){
    let mut divided_by_two: Vec<i32> = Vec::new();
    let mut divided_by_three: Vec<i32> = Vec::new();
    let mut rest_numbers: Vec<i32> = Vec::new();

    for &nums in rng{
        match nums{
            n if n % 2 == 0 => divided_by_two.push(n),
            n if n % 3 == 0 => divided_by_three.push(n),
            _ => rest_numbers.push(nums)
        }
    }
    return (divided_by_two, divided_by_three, rest_numbers);
}

fn main(){
    let rng: Vec<i32> = (0..=100).collect();
    let results = arrange_nums(&rng);

    println!("{:#?}", results.0);
    println!("{:#?}", results.1);
    println!("{:#?}", results.2);
}
