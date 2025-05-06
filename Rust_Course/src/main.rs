
mod lab1;
mod lab2;
mod lab3;
mod lab4;

// use UIEvent::*;
// use Direction::*;
// #[derive(Debug)]
// enum UIEvent {
//         Scroll(Direction),
//         ButtonClicked,
//         KeyPressed(char),
//         MouseClicked{x: u32, y:u32}
// }
// impl UIEvent {
//         fn describe(&self){
//                 println!("UIEvent {:?}",self);
//         }
// 
//         fn call(event : UIEvent) {
//                 use UIEvent::*;
//                 match event {
//                         ButtonClicked => println!("Button clicked"), // simple match
//                         Scroll(x) => println!("Scroll {:?}", x), // attribute extraction
//                         KeyPressed(ch) => { // whole block
//                                 let up_ch = ch.to_uppercase();
//                                 println!("Key pressed: {}", up_ch);
//                         },
//                         MouseClicked { x, y } => println!("Mouse clicked at ({}, {})", x, y), // attribute extraction
//                 }
//         }
// }
// #[derive(Debug)]
// enum Direction {
//         Up,
//         Down,
// }

// use UIEvent::*;
// use Direction::*;

// #[derive(Debug)]
// struct Rectangle {
//     x : f32,
//     y : f32
// }
//
// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.x * self.y
//     }
//
//     fn perimeter(&self) -> f32{
//         2.0 * ( self.x + self.y )
//     }
//
//     fn scale(&mut self, factor:f32) {
//         self.x = self.x * factor;
//         self.y = self.y * factor;
//     }
//
//     fn new_square(x : f32) -> Rectangle { // there is no self in the argument list
//         Rectangle{x, y : x}
//     }
//
// }
//
// trait Shape {
//     fn area(&self) -> f32;
//     fn perimeter(&self) -> f32;
//     fn describe(&self) {
//         println!("I'm a general shape.");
//     }
// }
//
// impl Shape for Rectangle {
//     fn area(&self) -> f32 {
//         self.x * self.y
//     }
//
//     fn perimeter(&self) -> f32 {
//         2f32 * (self.x + self.y)
//     }
//
//     fn describe(&self) {
//         println!("I'm a rectangle.");
//     }
// }

// #[derive(Debug)]
// enum UIEvent {
//     Scroll(Direction),
//     ButtonClicked,
//     KeyPressed(char),
//     MouseClicked{x: u32, y:u32}
// }
// impl UIEvent {
//     fn describe(&self){
//         println!("UIEvent {:?}",self);
//     }
//
//     fn call(event : UIEvent) {
//         use UIEvent::*;
//         match event {
//             ButtonClicked => println!("Button clicked"), // simple match
//             Scroll(x) => println!("Scroll {:?}", x), // attribute extraction
//             KeyPressed(ch) => { // whole block
//                 let up_ch = ch.to_uppercase();
//                 println!("Key pressed: {}", up_ch);
//             },
//             MouseClicked { x, y } => println!("Mouse clicked at ({}, {})", x, y), // attribute extraction
//         }
//     }
// }
// #[derive(Debug)]
// enum Direction {
//     Up,
//     Down,
// }
//
// #[derive(Debug)]
// enum Message {
//     Move{x: u32, y: u32},
//     Echo(String),
//     ChangeColor(u8, u8, u8),
//     Quit,
// }
//
// impl Message {
//     fn call(&self) {
//         println!("{:?}", self);
//     }
// }

// use lab3::vector2d::Vector2d;

fn main() {
        // let v1 = Vector2d::new(3.0, 4.0);
        // let v2 = Vector2d::new(1.0, 2.0);
        //
        // println!("Vector v1:");
        // v1.write();
        // println!("Vector v2:");
        // v2.write();
        //
        // let sum = v1 + v2;
        // println!("Sum (v1 + v2):");
        // sum.write();
        //
        // let v1 = Vector2d::new(3.0, 4.0);
        // let v2 = Vector2d::new(1.0, 2.0);
        //
        // let diff = v1 - v2;
        // println!("Difference (v1 - v2):");
        // diff.write();
        //
        // let v1 = Vector2d::new(3.0, 4.0);
        //
        // let unit_v1 = v1.fabricate();
        // println!("Normalized v1:");
        // unit_v1.write();
        //
        // let v1 = Vector2d::new(3.0, 4.0);
        // let v2 = Vector2d::new(1.0, 2.0);
        //
        // let is_equal = v1.compare(&v2);
        // println!("Are v1 and v2 equal? {}", is_equal);


    //let r = Rectangle{x: 1.0, y: 2.0}; // create a new instance using constructor
    //
    // println!("{:?}", r);
    //
    // println!("x: {}, y: {}", r.x, r.y);

    //println!("Area of {:?} is {}", r, r.area());

    // let mut r = Rectangle{ x : 5.0, y : 4.0};
    // // r.scale(2.0);
    // println!("Area of r is {}", r.area());
    // let square = Rectangle::new_square(5.0);
    // println!("square: {:?}", square);
    // let b_pressed_event = KeyPressed('b');
    // let c_pressed_event = KeyPressed('c');
    // c_pressed_event.describe();
    // let scroll_down = Scroll(Down);
    // let mouse_clicked = MouseClicked{x: 10, y: 20};

    // let messages = [
    //     Message::Move { x: 10, y: 30 },
    //     Message::Echo(String::from("hello world")),
    //     Message::ChangeColor(200, 255, 255),
    //     Message::Quit,
    // ];
    //
    // for message in &messages {
    //     message.call();
    // }

    // let clicked = ButtonClicked;
    // let scroll = Scroll(Direction::Down);
    // let key_pressed = KeyPressed('b');
    // UIEvent::call(clicked);
    // UIEvent::call(scroll);
    // UIEvent::call(key_pressed);

    //lab2

    // let a= lab2::password_generator::generate(5);
    // println!("{}",a);
    //
    // let a= lab2::password_generator::generate(7);
    // println!("{}",a);
    //
    // let a= lab2::password_generator::generate(9);
    // println!("{}",a);
    //
    // let a= lab2::password_generator::generate(1);
    // println!("{}",a);
    //
    // let a= lab2::password_generator::generate(3);
    // println!("{}",a);

    // lab1

    //lab1::tictactoe::play();
    // let board = [
    //     [5, 3, 0, 0, 7, 0, 0, 0, 0],
    //     [6, 0, 0, 1, 9, 5, 0, 0, 0],
    //     [0, 9, 8, 0, 0, 0, 0, 6, 0],
    //     [8, 0, 0, 0, 6, 0, 0, 0, 3],
    //     [4, 0, 0, 8, 0, 3, 0, 0, 1],
    //     [7, 0, 0, 0, 2, 0, 0, 0, 6],
    //     [0, 6, 0, 0, 0, 0, 2, 8, 0],
    //     [0, 0, 0, 4, 1, 9, 0, 0, 5],
    //     [0, 0, 0, 0, 8, 0, 0, 7, 9]
    // ];
    // print!("{}",lab1::sudoku::check_sudoku_board(board))
}
