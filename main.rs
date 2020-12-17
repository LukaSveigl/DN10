mod fns;

fn main() {
    let mut _s = String::from("Hello");
    let mut _w = String::from("world!");

    //println!("{}", s);
    //change(&mut s);
    //println!("{}", s);

    let r1 = &_s; // multiple immutable references are allowed to exist
    let r2 = &_s;

    println!("{} {}", r1, r2);

    let r3 = &mut _s; // r1 and r2 are out of scope after being used

    fns::change(r3);
    println!("{}", r3);

    let r4 = String::from("The quick brown fox jumps over the lazy dog.");

    let word1 = &r4[..5]; // if a slice from 0 to x, we can ommit 0
    let word2 = &r4[5..]; // if a slice from x to 0, we can ommit 0
    let word3 = &r4[..]; // if a slice from whole string (0..len), we can ommit both numbers

    println!("{}\n{}\n{}", word1, word2, word3);

    println!("The first word is {}", fns::first_word(&r4));

    println!("The second word is {}", fns::second_word(&r4));

    let i1 = [1, 2, 3, 4, 5];

    let i_slice = &i1[1..4]; // with slices, the interval is closed on left side and open on right side: [x, y)

    for is in i_slice.iter() {
        print!("{} ", is);
    }
    println!("\n");

    let usr1 = fns::build_user("KlemenHoci".to_string(), "klemen666@gmail.com".to_string());
    let usr2 = fns::User {
        username: String::from("Janja Blenkuš"),
        email: String::from("janjaja@gmail.com"),
        ..usr1 // update operator
    };

    usr1.display();
    usr2.display();

    //fns::display_user(&usr1); //replaced function with User struct method
    //fns::display_user(&usr2);

    println!("{:?}", usr1); // debug display (can be used because of the derive directive)
    println!("{:#?}", usr2); // pretty-print debug display

    let _home_address: fns::IpAddr = fns::IpAddr::V4(String::from("127.0.0.1"));
    let _loopback_address: fns::IpAddr = fns::IpAddr::V6(String::from("::1"));

    let msg1 = fns::Message::Write(String::from("Testing stuff!"));
    msg1.call();

    let _msg2 = fns::Message::Quit;
    let _msg3 = fns::Message::Move { x: 64, y: 32 };
    let _msg4 = fns::Message::ChangeColor(255, 255, 255);

    print_type_of(&(255, 255, 255));

    let coins = [
        fns::Coin::Cent5,
        fns::Coin::Euro2(1991),
        fns::Coin::Euro1,
        fns::Coin::Cent20,
        fns::Coin::Cent1,
    ];

    for cn in coins.iter() {
        println!("Coin is worth {} cents.", fns::value_in_cents(cn));
    }

    let five = Some(5);
    println!(
        "\nPlus something: {:#?}\nPlus none: {:#?}\n",
        fns::plus_one(five),
        fns::plus_one(None)
    );

    if let Some(5) = five {
        println!("Petkicaaa!");
    }

    let mut coin: u8 = 1;
    for cn in coins.iter() {
        fns::test_if_let(&mut coin, cn);
        println!("{}. kovanec ki ni 20 centov.", coin);
    }

    println!(
        "Fibonacci of num {num} is {fib}.",
        num = 13,
        fib = fns::test_fibonacci(13)
    );
}

fn print_type_of<T>(_: &T) {
    println!("{}\n", std::any::type_name::<T>());
}
