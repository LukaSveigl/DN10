//------------STRING MANIPULATION-------------

pub fn change(some_s: &mut String) {
    some_s.push_str(", world");
}

pub fn first_word(some_s: &str) -> &str {
    // &str indicates string slice
    let bytes = some_s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_s[..i];
        }
    }
    &some_s[..]
}

pub fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_index: usize = 0;
    let mut first_found: bool = false;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if !first_found {
                first_found = true;
                first_index = i + 1;
            } else {
                return &s[first_index..i];
            }
        }
    }
    &s[..]
}

// -----------STRUCTS-----------------
#[derive(Debug)] // signal that struct derives the default implementation of Debug display
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

impl User {
    fn _init(&self, username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }

    pub fn display(&self) {
        println!(
            "User {}, with email {}: \n + Is active: {} \n + User was logged in {} times.\n",
            self.username, self.email, self.active, self.sign_in_count
        );
    }
}

pub fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

/*pub fn display_user(user: &User) {
    println!(
        "User {}, with email {}: \n + Is active: {} \n + User was logged in {} times.\n",
        user.username, user.email, user.active, user.sign_in_count
    );
}*/

//----------------ENUM---------------
pub enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
pub enum Message {
    Quit,                    // empty struct
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        println!("{:#?}", self);
    }
}

pub enum Coin {
    Cent1,
    _Cent2,
    Cent5,
    _Cent10,
    Cent20,
    _Cent50,
    Euro1,
    Euro2(u32),
}

pub fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Cent1 => 1,
        Coin::_Cent2 => 2,
        Coin::Cent5 => 5,
        Coin::_Cent10 => 10,
        Coin::Cent20 => 20,
        Coin::_Cent50 => 50,
        Coin::Euro1 => 100,
        Coin::Euro2(letnica) => {
            println!("Letnica je: {}", letnica);
            200
        }
    }
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn test_if_let(c: &mut u8, coin: &Coin) {
    if let Coin::Cent20 = coin {
        println!("Found 20 cent coin!");
    } else {
        *c += 1;
    }
}

pub fn test_fibonacci(to_calculate: i32) -> i32 {
    if to_calculate == 0 || to_calculate == 1 {
        to_calculate
    } else {
        test_fibonacci(to_calculate - 1) + test_fibonacci(to_calculate - 2)
    }
}
