
mod lab1;
mod lab2;
mod lab3;

use UIEvent::*;
use Direction::*;
#[derive(Debug)]
enum UIEvent {
    Scroll(Direction),
    ButtonClicked,
    KeyPressed(char),
    MouseClicked{x: u32, y:u32}
}
impl UIEvent {
    fn describe(&self){
        println!("UIEvent {:?}",self);
    }

    fn call(event : UIEvent) {
        use UIEvent::*;
        match event {
            ButtonClicked => println!("Button clicked"), // simple match
            Scroll(x) => println!("Scroll {:?}", x), // attribute extraction
            KeyPressed(ch) => { // whole block
                let up_ch = ch.to_uppercase();
                println!("Key pressed: {}", up_ch);
            },
            MouseClicked { x, y } => println!("Mouse clicked at ({}, {})", x, y), // attribute extraction
        }
    }
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
enum Message {
    Move{x: u32, y: u32},
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}


fn main() {
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

    let clicked = ButtonClicked;
    let scroll = Scroll(Direction::Down);
    let key_pressed = KeyPressed('b');
    UIEvent::call(clicked);
    UIEvent::call(scroll);
    UIEvent::call(key_pressed);

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
