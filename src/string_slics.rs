use std::{vec, fmt::format};

use resturant;

fn main(){
    let  v :Vec<i32> = Vec::new();
    let mut cart = vec!["orange","apple","orange"];
    cart.push("hi");
    println!("{:?}", cart);
    println!("the value is from cart {}",&cart[0]);
    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);

    // array out of bound
    // let does_not_exists = v.get(100);
    // let does_not_here = &v[100];
    match third {
        Some(third)=>println!("{} value is here",third),
        None => ()
    }

    for i in &cart{
       println!("{i} items") 
    }

    let mut salary = vec![100, 32, 57];
    for i in &mut salary {
        *i += 50;
        println!("{}", i)
    }

    #[derive(Debug)]
    enum SpreadSheet {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheet::Int(3),
        SpreadSheet::Float(12.2),
        SpreadSheet::Text(String::from("Hello Spreadsheet"))
    ];

    for i in &row{
        println!("{:?}", i )
    }

    let data = "Initial contents";
    let data_2 = String::from("Hello world");

    let mut str_test = String::from("Hello string");
    let add_one = String::from("Add one");
    str_test.push_str(&add_one);
    let str_slice = "Slice";
    str_test.push_str(&str_slice);

    println!("{str_slice}");
    println!("{str_test}");

    let s1 = String::from("hello, ");
    let s2:String = String::from("World");

    let s3 = s1 + &s2;
    println!("{s3}");

    let one = String::from("tic");
    let two = String::from("tac");
    let three = String::from("toe");

    let game = format!("{one}-{two}-{three}");
    println!("{game} , game name");

    let hello = String::from("Здравствуйте");
    // weird error may occur
    // let s = &hello[0..3];
    // println!("{s} string slice ") ;

    for c in hello.chars(){
        println!("{c}")
    }
    for b in hello.bytes(){
        println!("{b}")
    }
}

