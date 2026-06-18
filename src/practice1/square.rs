#[derive(Debug)]
struct Square{
    height: u32,
    width: u32
}

impl Square{
    fn area(&self) -> u32{
        return self.height * self.width;
    }
    fn sq_root(&self) -> u32{
        return self.height * self.height;
    }
    fn get_measurement(&self) -> Self{
        return Self{
            height: self.height,
            width: self.width
        };
    }
    fn can_hold(&self, other: &Square) -> bool{
        return self.area() > other.area();
    }
}


fn main(){
    let sq1 = Square{height: 20, width: 60};
    let sq2 = Square{height: 50, width: 70};

    println!("Can sq1 hold sq2: {}", sq1.can_hold(&sq2));
    println!("Can sq2 hold sq1: {}", sq2.can_hold(&sq1));
    println!("{}", sq2.area());
    println!("{}", sq1.area());
    println!("{}", sq1.sq_root());

}
