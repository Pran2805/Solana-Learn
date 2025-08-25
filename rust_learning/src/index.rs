use std::fs;

// mutability handles the race condition also in threads
pub fn mutability() {
    // by default everything in rust is immutable
    // let x = 2;
    //x =4; // it will throw error Error because of x is immutable

    // you can also do not update string array etc.

    let mut y = 6; // now it is mutable
    println!("{}", y);
    y = 10;

    println!("{}", y);
}

pub fn pointers() {
    let mut string1 = String::from("Hello");
    let string2 = &mut string1;

    string2.push_str(", World!");
    println!("{}", string2);
    println!("{}", string1);
}

struct User {
    id: i8,
    name: String,
    age: i8

}

// struct Person;
pub fn structs(){
    let user1 = User {
        id: 1,
        name: String::from("Pranav Shinde!"),
        age:20
    };

    // let person = Person; // you can also do this
    println!("{:?}", user1.id);
    println!("{:?}", user1.name);
    println!("{:?}", user1.age);

    let rect = Rect{
        width:30,
        height:50
    };

    print!("\nThe area of reactange is {}, where width is {} and height is {}", rect.area(), rect.width, rect.height);

}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// pub fn enums (){
//     let my_direction = Direction::North;
//     let new_direction = my_direction; // no error, because direction is copy
// }

// enum Direction {
//     North,
//     East,
//     South,
//     West
// }

enum Shape{
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}

fn calculate_area(shape: Shape) -> f32{
    match shape {
        Shape::Circle(radius) => std::f32::consts::PI * radius * radius,
        Shape::Square(side_lenght) => side_lenght * side_lenght,
        Shape::Rectangle(width, height) => width * height,
    }
}

pub fn pattern_match(){
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0,6.0);

    println!("\nArea of Circle: {}", calculate_area(circle));
    println!("Area of Square: {}", calculate_area(square));
    println!("Area of Rectangle: {}", calculate_area(rectangle));

}

pub fn error_handling(){
    // rust is also has error handling

    let greeting_file_result = fs::read_to_string("Rust.md".to_string());
    match greeting_file_result {
        Ok(content) =>{
            println!("File Content: {}", content)
        },
        Err(err) =>{
            println!("Error: {}", err)
        },
    }

    print!("Still run this")
}

pub fn find_first_a(s: String) -> Option<i32> {
    for (index, chars) in s.chars().enumerate(){
        if chars == 'a'{
            return Some(index as i32)
    }
}
    return None;
}