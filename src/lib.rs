use std::io::{self,Result};
mod resturant {
    pub mod hosting{
        pub fn add_to_waitlist(){
            println!("Added to waitlist");
        }
    }
    mod serving{
        fn take_order(){
            println!("Take order");
        }
        fn give_order(){
            println!("Give order");
        }
    }
}

pub fn eat_at_resturant(){
    // absolute path
    crate::resturant::hosting::add_to_waitlist();
    resturant::hosting::add_to_waitlist();
}

fn deliver(){
    println!("deliver order");
}

mod back_to_house{
    use crate::deliver;

    fn fix_wrong_order(){
        super::deliver();
        cook_order();
    }

    fn cook_order(){

    }
}


mod order_list{
    #[derive(Debug)]
    pub struct Breakfast{
        pub toast : String,
        fruit : String,
    }

     impl  Breakfast {
        pub fn summer(toast:&str)-> Breakfast{
            Breakfast { toast: String::from(toast), fruit: String::from("Food from burma")}
        }
    }
}

pub fn order_at_resturant(){
    let mut meal = order_list::Breakfast::summer("break");
    meal.toast = String::from("Bread with chocolate");
    println!("{:?}",meal);
}

pub mod appetizers{
    pub enum Appetizers {
       Soup, 
       Salad 
    }
}

pub mod eat_appetizers{
    fn eat(food:String,amount:&i32)-> i32 {
        println!("{food}");
        amount * 12 
    }
    fn pay(amount : i32)->i32{
        eat(String::from("Bread"), &21);
        10
    }
}

pub fn eat_ar_resturant(){
    let order1 = appetizers::Appetizers::Soup;
    let order2 = appetizers::Appetizers::Salad;
}