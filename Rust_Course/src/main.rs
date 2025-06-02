use std::collections::HashMap;
use std::ops::Add;

mod lab1;
mod lab2;
mod lab3;

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
struct Pair<T> {
    x: T,
    y: T
}

struct Pair2<T, U> {
    x: T,
    y: U
}

enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

impl <T> Pair<T> {
        fn extract_x(self) -> T {
            self.x
        }



}

// impl <T: Add + Copy> Pair<T> {
//     fn add(&self, other: &Pair<T>) -> Pair<T> {
//         Pair{x: self.x + other.x, y: self.y + other.y}
//     }
// }

impl<T: PartialOrd + Copy> Pair<T> {
    fn bigger(&self) -> T {
        if self.x > self.y {
            self.x
        } else {
            self.y
        }
    }
}
struct Table<T> {
    data: Vec<T>,
}

// impl <T: PartialOrd + Copy> Table<T>{
//     fn max(&self) -> T{
//         let a = self.data.len();
//         if a == 0{
//             return None
//         }
//         if a == 1{
//             return self.data[0]
//         }
//         let mut max = &self.data[0];
//         for i in 1..self.data.len(){
//             if self.data[i] > max {
//                 max = &self.data[i];
//             }
//         }
//         max
//     }
// }

// fn len_longer_array(a : &[i32], b : &[i32]) -> usize {
//     if a.len() > b.len() {
//         a.len()
//     } else {
//         b.len()
//     }
// }
//
// fn longer_array<'a>(a : &'a [i32], b : &'a [i32]) -> &'a [i32] {
//     if a.len() > b.len() {
//         &a
//     } else {
//         &b
//     }
// }
// struct Introduction<'a> {
//     intro : &'a str
// }
//
// impl<'a> Introduction<'a> {
//     fn print(&self) {
//         println!("{}", self.intro);
//     }
// }
//
// fn get_sample_text() -> &'static str {
//     "Just a sample text"
// }

// struct Rectangle {
//     width : f64,
//     height : f64
// }
//
// impl Rectangle {
//     fn new(width : f64, height : f64) -> Rectangle {
//         Rectangle{width, height}
//     }
//
//     fn area(&self) -> f64 {
//         self.width * self.height
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_new_rectangle() {
//         // given
//         let width = 4.5;
//         let height = 5.7;
//
//         // when
//         let r = Rectangle::new(width, height);
//
//         // then
//         assert!((r.width - width).abs() < f64::EPSILON && (r.height - height).abs() < f64::EPSILON);
//     }
// }

fn words_stats(words: &Vec<String>) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for word in words {
        let count = map.entry(word.clone()).or_insert(0);
        *count += 1;
    }
    map
}
fn hash_map_words_stats_poem() {
    let response = reqwest::blocking::get("https://wolnelektury.pl/media/book/txt/pan-tadeusz.txt").expect("Cannot get poem from a given URL");
    let poem = response.text().unwrap();

    let stats = words_stats(&split_to_words(&poem));
    let sorted_stats = sort_stats(&stats);
    println!("{:?}", &sorted_stats[..20]);
}


fn split_to_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    for word in s.split_whitespace() {
        let unified_word = word.trim_matches(|c| char::is_ascii_punctuation(&c)).to_lowercase();
        words.push(unified_word);
    }
    words
}

fn sort_stats(stats : &HashMap<String, i32>) -> Vec<(&str, i32)> {
    let mut sorted_stats : Vec<(&str, i32)> = Vec::new();
    for (word, count) in stats.iter() {
        sorted_stats.push((word, *count));
    }

    sorted_stats.sort_by(|(_, c1), (_, c2)| c2.partial_cmp(c1).expect("FAILED"));

    sorted_stats
}

fn main() {
    // let increment_v1 = |x: u32| -> u32 { x + 1 };
    // let increment_v2 = |x| { x + 1 };
    // let increment_v3 = |x| x + 1; // only if there is only one expression
    //
    // assert_eq!(increment_v1(4), 5); // calling functional object
    // assert_eq!(increment_v2(4), 5);
    // assert_eq!(increment_v3(4), 5);

    // let mut x = vec![1, 2, 3];
    // let equal_to_x = move |y| y == x;  // equal_to_x becomes the owner of x
    //
    // // println!("can't use x here: {:?}", x);
    //
    // let y = vec![1, 2, 3];
    // assert!(equal_to_x(y));
    // let v = vec![1, 2, 3];
    // // transforming each element
    // let res = v.iter().map(|x| x * 2).collect::<Vec<i32>>();
    // assert_eq!(vec![2, 4, 6], res);
    //
    // // filter elements
    // let res = v.iter().filter(|&x| *x % 2 == 0).map(|x| *x).collect::<Vec<i32>>();
    // assert_eq!(vec![2], res);


    // let mut v = vec![1, 2, 3];
    //
    // v.iter_mut().map(|x| *x +=1).collect::<Vec<_>>();
    // assert_eq!(vec![2, 3, 4], v);
    //
    // let v = vec![1, 2, 3];
    // let v2 = v.into_iter().map(|x | x * 2).collect::<Vec<_>>();
    //
    // // v.get(0); // cannot use v anymore - it's moved
    //
    // assert_eq!(vec![2, 4, 6], v2);
    // // hash_map_words_stats_poem();



    // let a: [i32; 0] = [];
    // let b: [i32; 3] = [1, 2, 3];
    // longer_array(&a, &b);

    //
    // let pi = Pair{x : 5, y : 3};
    //
    // let pf = Pair {x: 15f64, y : 12.0f64};
    //
    // let pw = Pair2 {x: 15f64, y : 0}; // compile error; x & y must be the same type

    // let text = String::from("Introduction to a long text. The rest of long text with many sentences.");
    //
    // let intro = text.split('.').next().expect("Could not find a first sentence.");
    //
    // let i = Introduction { intro };
    // let mut v = vec![1, 2, 3];
    //
    // for e in &mut v { // mutable borrow
    //     *e *= 2; // remember to use dereference operator
    // }

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