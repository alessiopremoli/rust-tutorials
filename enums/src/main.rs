enum IpAddrKind {
    V4,
    V6,
}

// we can put data inside an enum variant - treated as a function
enum IpAddrKindBetter {
    V4(String),
    V6(String),
}

enum IpAddrKindEvenBetter {
    // you can put any kind of data inside an enum variant
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// similar to defining different kinds of struct definitions,
// except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// you can define methods on enums
impl Message {
    fn call(&self) {
        println!("call!")
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // each of the same type
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddrKindBetter::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddrKindBetter::V6(String::from("::1"));

    let home3 = IpAddrKindEvenBetter::V4(127, 0, 0, 1);

    let m = Message::Write(String::from("Hello"));
    m.call();

    let x = Some(5);
    let y = 6;
    let some_string = Some("w");

    let absent_number: Option<i32> = None;

    let v = value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
        // other => move_player(other),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn route(ip_kind: IpAddrKind) {}

// match are esaustive: if we remove the None arm we'll end up with a compile error but ther's the _ placeholder
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
