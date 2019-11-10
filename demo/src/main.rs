fn main() {
    let int: i32 = -15;
    let uint: u32 = 256;
    let float: f32 = 1.0;
    let double: f64 = -15.103E3;
    let b: bool = true;
    let c: char = '🦀';
    let i = 15; // type is inferred
    // these are all immutable

    let mut k = 1;
    println!("k: {}", k);
    k += 1;
    println!("k: {}", k);

    // if
    let input = String::from("Hello World");
    if input == "Hello World" {
        println!("Classic")
    } else {
        println!("Unexpected");
    }

    // loop
    let mut counter = 1;
    let loop_ret = loop {
        counter += 1;

        if counter == 15 {
            break counter
        }
    };
    assert_eq!(loop_ret, 15);

    let loop_ret = loop {
        break
    };
    println!("loop_ret: {:?}", loop_ret);

    // while
    let mut counter = 1;
    let while_ret = while counter < 15 {
        counter += 1;
    };
    assert_eq!(counter, 15);
    assert_eq!(while_ret, ());

    // for
    let range = (1..101);
    for elem in range {
        print!("{} ", elem);
    }

    // Tuples and Arrays
    let tuple = (1, true, "Test");
    let array = [1, 2, 3, 4];
    let slice = &array[0..2];

    assert_eq!(tuple.1, true);
    assert_eq!(array[1], 2);
    assert_eq!(array.len(), 4);
    assert_eq!(slice.len(), 2);

    // Borrows
    // 2 mutable borrows
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;
    
    // println!("{}, {}", r1, r2);

    // Mutable borrow with immutable
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &s;
    
    // println!("{}, {}", r1, r2);
}

enum Message {
    Skipped,
    Ok(u32), // tuple
    Error { code: u32, msg: String }, // struct
}

impl Message {
    fn is_ok(&self) -> bool {
        match self {
            Message::Ok(_) => true,
            _ => false
        }
    }

    fn get_error_code(&self) -> Option<&u32> {
        match self {
            Message::Ok (_) => None,
            Message::Skipped => None,
            Message::Error { code, .. } => Some(code)
        }
    }
}

struct MessageQueue {
    name: String,
    messages: Vec<Message>,
    history: Vec<Message>
}
