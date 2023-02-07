pub fn main(){
    let area = area_generic(32, 12);
    println!("area is {area}");
    let area2 = area_tuple((21,21));
    println!("{}",area2);
    area_structs();
    area_rectangle();
    methods()
}


fn methods(){
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self)->u32{
            self.width * self.height
        }
        fn width(&self)->bool{
            self.width > 0
        }
        fn can_hold(&self, rect : &Rectangle) -> bool{
            self.width > rect.width && self.height > rect.height
        }
    }

    let rect1 = Rectangle{
        width:21,
        height:31
    };
    let rect2 :Rectangle = Rectangle { width: 12, height: 12 };
    println!("The method can be the same name with property in struct {}", rect1.width());
    println!("the area is {}", rect1.area());
    println!("the rectangle can hold {}", rect1.can_hold(&rect2));

    struct Square {
        width:u32,
        height:u32
    }
    impl Square {
        fn square(size:u32)->Self{
            Self{
                width:size,
                height:size
            }
        }
    }

    let sqrt1 = Square{
        width:12,
        height:12
    };
    println!("The width and height {} , {}",sqrt1.width,sqrt1.height );

    let sqrt2 = Square::square(10);
    println!("the second square is {}" , sqrt2.height)
}   


fn area_generic(width :i32, height : i32)->i32{
    width * height
}

fn area_tuple(dimensions:(i32,i32)) -> i32{
    dimensions.0 * dimensions.1
}

fn area_structs(){
    #[derive(Debug)]
    struct Dimensions{
        width : u32, 
        height : u32,
    }
    fn area(rect : &Dimensions)->u32{
        rect.width * rect.height
    }
    let rect1 = Dimensions{
        width: 32,
        height: 32
    };
    let a = area(&rect1);
    println!("the area of the rectangle is {} and it is calced from ref", a);

    println!("rect1 is {:?}", rect1);
}

fn area_rectangle(){
    #[derive(Debug)]
    struct Triangle{
        width:u32,
        base:u32
    }

    impl Triangle {
       fn area(&self) -> u32{
         self.width * self.base
       }
    }
    let tria1 = Triangle{
        width:21,
        base:21
    };
    print!("{}",tria1.area())
}
