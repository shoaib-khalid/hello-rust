
struct Person {
    name : String,
    age : u8,
}

impl Person {
    // This is a method. It has access to the instance of the struct.
    fn greet(&self){
        println!("Hello my name is: {}, and my age is: {}", self.name, self.age);
    }
}

fn main() {
    let person= Person {
        name : String :: from("Asim"),
        age : 38,
    };


    println!("Hello Person, world!");
}
