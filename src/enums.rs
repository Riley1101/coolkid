pub fn main(){
 ip_addr();
 message();
 match_para();
 optional_t()
}

fn optional_t(){
    fn plus_one(x : Option<i32>)->Option<i32>{
       match x {
        None =>  None,
        Some(i)=>Some(i+1)
       } 
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    let dice_roll = 8 ;
    match dice_roll{
        3 => println!("3 is matched"),
        7 => println!("7 is matched"),
        other =>println!("other is match with catch all{}",other)
    }

    let catch_all = 12;
    match catch_all{
        3 => println!("3 is matched"),
        7 => println!("7 is matched"),
        // we can just return a tuple if we want nothing to happern
        _=>println!("other is match with _ placeholder")
    }
}

fn match_para(){
    #[derive(Debug)]
    enum Coin {
        Penny, 
        Nickel,
        Dime,
        Quarter
    }
    fn values_in_cent(coin:Coin)->u8{
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin :: Quarter => 25
            
        }
    }

    let coin = Coin::Dime;
    println!("{:?}", values_in_cent(coin));

    enum UsState {
        Alaska,
        Alabama,
        NYC
    }

    enum Estate {
        House(UsState),
        Hotel,
    }

    fn houses_in_state(state:Estate) ->u8{
        match state {
            Estate::House(state)=>{
                12
            },
            Estate::Hotel=>13
        }
    }
}

fn message(){

    struct Quit;
    struct Move {
        x : u32,
        y: u32
    }
    struct Write(String);
    struct Color(i32,i32,i32) ;

    enum Message {
        Quit,
        Move,
        Write,
        Color
    };

    struct Chat {
        message : Message
    }

    let quit = Quit;
    let loc = Move{
        x:12,
        y:12
    };
    let wt = Write(String::from("write"));
    let clr :Color = Color(123, 121, 11);
    let some_number = Some(5);
    let some_char = Some('e');
    println!("{:?}", some_char);
    println!("{:?}", some_number);
    let x = 5;
    let y : Option<i8> = Some(5);


    // let sum = x + y;
    // print!("{}", sum);
  
    
}

fn ip_addr(){

    #[derive(Debug)]
    enum IpAddressKind {
        V4,
        V6,
    };

    let four =  IpAddressKind::V4;
    let five  = IpAddressKind::V6;
   
    fn route (ip_kind:IpAddressKind)->IpAddressKind{
        ip_kind
    }

    #[derive(Debug)]
    struct Address {
        kind : IpAddressKind,
        address: String
    }

    let home = Address{
        kind:IpAddressKind::V4,
        address:String::from("Hello string")
    };
    let loop_add = Address{
        kind: IpAddressKind::V6,
        address:String::from("::1")
    };

    print!(" kind and {} address {:?}",  home.address,home);
    println!("Loop_add has a weird syntax in address {}", loop_add.address);

    #[derive(Debug)]
    enum Brand {
        V1(String),
        V2(String)
    }
    let brand1 = Brand::V1(String::from("Volvo"));
    let brand2 = Brand::V2(String::from("Honda"));

    println!("{:#?} {:#?}", brand1, brand2);

    #[derive(Debug)]
    enum Ip{
        V4(u8, u8, u8, u8),
        V6(String)
    }
    
    let address1 = Ip::V4(127,0,0,1);
    let name :Ip = Ip::V6(String::from("Home"));

    #[derive(Debug)]
    struct Location {
        address : Ip,
        name : String
    }
    let position = Location{
        address:Ip::V4(127, 0, 0, 1),
        name:String::from("Home"),
    };

    println!("enums rocks {:?}",position);
}