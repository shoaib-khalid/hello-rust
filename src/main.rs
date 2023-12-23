fn main() {
    println!("Hello, world!");
    println! ("Sum of {} and {} is {}",5,6,sum(5,6));
}

fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
}


fn factorial (num: i32) -> i32 {
        let mut result:i32 = 1;
        for i in 1..=num {
            result *= i;
        }
        return result;
}