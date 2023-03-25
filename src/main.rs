fn main() {
    // let favorite_color: Option<&str> = None;
    // let is_tuesday = false;
    // let age: Result<u8, _> = "34".parse();

    // if let Some(color) = favorite_color {
    //     println!("Using your favorite color, {color}, as the background");
    // } else if is_tuesday {
    //     println!("Tuesday is green day!");
    // } else if let Ok(age) = age {
    //     if age > 30 {
    //         println!("Using purple as the background color");
    //     } else {
    //         println!("Using orange as the background color");
    //     }
    // } else {
    //     println!("Using blue as the background color");
    // }

    // let mut stack = Vec::new();

    // stack.push(1);
    // stack.push(2);
    // stack.push(3);

    // while let Some(top) = stack.pop() {
    //     println!("{top}");
    // }

    // let v = vec!['a', 'b', 'c'];

    // for (index, value) in v.iter().enumerate() {
    //     println!("{value} is at index {index}");
    // }

    //! Error
    // let (x, y) = (1, 2, 3);

    // let point = (3, 5);
    // print_coordinates(&point);

    
    // let some_option_value: Option<i32> = None;
    // let Some(x) = some_option_value; //! Error

    // if let Some(x) = some_option_value {
    //     println!("{x}");
    // }

    // if let x = 5 {
    //     println!("{x}");
    // }

    // let x = 1;

    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    // let x = Some(5);
    // let y = 10;

    // match x {
    //     Some(50) => println!("Got 50"),
    //     Some(y) => println!("Match, y = {y}"),
    //     _ => println!("Default case, x = {:?}", x),
    // }

    // println!("at the end: x = {:?}, y = {y}", x);

    // let x = 1;

    // match x {
    //     1 | 2 => println!("one or two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    // let x = 5;

    // match x {
    //     1..=5 => println!("one through five"),
    //     _ => println!("somethings else"),
    // }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("Current locationL ({x}, {y})");
// }