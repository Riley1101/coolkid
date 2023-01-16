#[allow(unused_variables)]
fn main() {
    let greetings = "Hello";
    let subject = "World";
    let message = format!("{} {}",greetings,subject);

    println!("{}", message);
    println!("{}, {}!", greetings,subject);

    let mut x = 1.1;
    let mut y = 2.2;

    println!("x times y is {} ",x * y);

    x = 2.2 * 2.2;

    println!("x is changed to {}", x);

    y = 3.1;
    println!("You can change y to same type {}",y);

    let result :f64;
    result = multiply_both(20.2, 32.2);
    println!("Result are {}",result);

    let ninty = 90;
    let one_thousand = 1_000;
    let many_thousand = 1_000_000 / 3;

    println!("{} {} {}",ninty, one_thousand, many_thousand);
    println!("console.log end here");
    println!("{} type changed from i64 to f64",multiply_change_type(4,32.1));

    let should_we_go = true;
    let should_we_go_too = false;

    println!("{} {}",should_we_go,should_we_go_too);
    println!("{}", 1 == 2);

    if 1 > 2 {
        println!("1 is less than to 2");
    } 

    // a expression evalutes to a value
    // a statement does not evalute to a value 

    else if 2 > 1 {
        println!("2 is greater than 1");
    }
    else {
        println!("1 is not equal to 2");
    }

    if true {
        println!("i am true ");
    }

    let animal = "dog";
    let message = if animal == "cat" {
        "Meow"
    } else if animal == "dog"{
        "Woof"
    }else {
        "Hi! "
    };

    println!("It is an {}",message);
    say_loud(21.2);
    sandbox(); 
}

fn sandbox(){
     let result : bool = divisible_by_3(12);
    println!("{} this is the result here ", result);
    println!("five or six {}",five_or_six(true));
    println!("this is the counter value {}",counter(12));
    count_up_down(12);
    say_hello_time(12);
    access_array([12,32,43,54,65]);
    string_sandbox();
    ownership_and_functions();
    references_and_borrow();
    slice_type()
}

fn slice_type(){
    fn first_word(s:&String)->usize{
        let bytes = s.as_bytes();
        for (i,&item) in bytes.iter().enumerate(){
            if item == b' '{
                println!("{i}");
                return i;
            }
        }
        s.len()
    }
    let s1 = String::from("Hello string");
    let slice1 = &s1[..];
    println!("{slice1}");
    println!("{s1}");

    fn first_word_slice(s:&String)->&str{
        let bytes = s.as_bytes();

        for(i , &item) in bytes.iter().enumerate(){
            if item == b' '{
                return &s[0..i];
            }
        }
        &s[..]
    }
    let s2 :String = String::from("Hello string 2 ");

    let word2 = first_word_slice(&s2);
    println!("{word2}");

    fn arr_slices(){
        let a = [1,2,3,4,5];
        let slice = &a[1..3];
    } 

}

fn references_and_borrow(){
    let mut s = String::from("Hello world");
    fn calculate_area (s1:&mut String){
        s1.push_str("Add new value ");
        println!("The length of the string is {}", s1.len());
    }
    calculate_area(&mut s);
    println!("{}",s);
}

fn string_sandbox(){
    // String literals are unknown at runtime
    let mut s = String::from("Hello world");
    s.push_str("is the start");
    println!("{}", s);

    let str = "Hello";
    println!("{str}");

    // copied the stack
    let s1 = String::from("Hello new String with Heap");
    let s2 = s1;
    println!("{s2}");

    // deep copy the entire heap
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2,s3);

    // literals dont have extensive deep copy
    let n1 = 32;
    let n2 = n1;
    println!("n1 is {n1} n2 is {n2}");

    // let reference_to_nothing = dangle();
    // println!(reference_to_nothing)
}

// fn dangle()->&String{
//     let s = String::from("Hello");
//     return  &s
// }
fn ownership_and_functions(){
    fn takes_ownership(str : String){
        println!("{str} , i took the ownership")
    }

    fn make_copy(num: i32){
        println!("{}", num)
    }
    let s = String::from("Hello Owner !");

    let num :i32 = 32;
    make_copy(num);
    takes_ownership(s);

    fn takes_and_gives(str: String)->String{
        str
    }
    let s2 = String::from("Hello string 2 ");
    let s4 = takes_and_gives(s2);
    println!("Takes and gives {}",s4);
    // println!("{s2}, I will throw an error");

    fn calculate_length(s:String)->(String, usize){
        let length = s.len();
        (s, length)
    }
    let (s5, len ) = calculate_length(s4);
    println!(" Return ownership {}",s5)
}

fn counter(limit :i32)->i32{
    let mut number = 0 ;

    let result = loop{  
        number += 1;
        if number ==  limit{
            break number * 10;
        }
    };
    result
}

fn count_up_down(limit :i32){
    let mut counter = 0;

    let mut remain = limit;
    'counting_up: loop {
        println!("Counting up");
        if counter > remain {
            break 'counting_up;
        }
        counter +=1 ;
        remain -=1;

        println!("{remain} couter remaining")
    }
    'counting_down : loop{
        println!("counting down");
        if remain > counter  {
            break 'counting_down;
        }
        remain += 1;
    }
}


fn divisible_by_3 (num : i32) -> bool{
    let res : bool;
    if num % 3 == 0 {
        res = true;
    }else {
        res = false;
    }
    res 
}

fn five_or_six (condition :bool)->i32{
    let number = if condition {5} else {6};
    number
}

fn plus_one(x: i32) ->i32{
    x + 1
}

fn say_hello_time(limit :i32){
    let mut number = limit;

    while number != 0  {
        println!("{number}") ;
        number -=1;
    }
}

fn access_array(arr:[i32; 5],){
    let mut index = 0;
    while index < 5{
        println!("the value is {}",arr[index]);
        index+=1;
    }

    for element in arr{
        println!("there we have {}",element)
    }
    
    for number in 1..4{
        println!("{number} !!!!")
    }
}

// fn find_element(){
//     let a = [1,2,3,4,5,6,7];
//     println!("enter an array index");
//     let mut index = String::new();

//     let index :usize = index.trim().parse().expect("index was not a number");
//     let element = a[index];
//     println!(" the value of the element at index {index} is {element}");
// }


fn multiply_change_type(x : i64, y:f64) -> f64{
    return x as f64 * y;
}

fn say_loud (x : f64){
    println!("It is an {}",x);
}


fn multiply_both(x : f64, y:f64)-> f64 {
    // you dont need return keyword it will
    // if a function ends with a expression
    // rust returns automatically
    // but if you add semi colon it becomes a statement
    x * y
}


