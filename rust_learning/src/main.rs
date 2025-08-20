fn main() {
    println!("Hello, world!");

    // number declaration
    let x = -1; // by default i32 
    let y: u32 = 1000;
    let z:f32 = 1000.001;
    println!("x:{}",x);
    println!("y:{}",y);
    println!("z:{}",z);

    let is_admin = true; // you should use snake case other wise it will throw an error

    if is_admin{
        println!("You are an admin")
    }else{
        println!("You are not an admin")
    }

    // String's can change the size at runtime
    let str = "Pranav"; // most complicated

    println!("{}",str)

}
