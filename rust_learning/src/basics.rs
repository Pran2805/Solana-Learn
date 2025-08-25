pub fn index(){
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

    println!("{}",str);

    let greeting = String::from("Hello, World!");
    let char1 = greeting.chars().nth(0);

    println!("{}", greeting);
    println!("{:?}",char1);
    println!("{}",char1.unwrap()); // easy but best way - My belief

    // other way - Advance level
    match char1{
        Some(c) => println!("{}", c),
        None => print!(""),
    }
}

pub fn loops(num:i128){
    for i in 0..num{
        print!("{} ", i)
    }
}

pub fn some_more_about_loops(){
    let sentence = String::from("My Name is Pranav Shidne");
    println!("{}", get_first_world(sentence));
}


fn get_first_world(sentence: String) -> String {
    let mut ans = String::from("");

    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}
