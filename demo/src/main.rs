fn main() {
    let int: i32 = -15;
    let uint: u32 = 256;
    let float: f32 = 1.0;
    let double: f64 = -15.103E3;
    let b: bool = true;
    let c: char = "🦀";
    let i = 15; // type is inferred
    // these are all immutable

    let mut k = 1;
    println!("k: {}", k);
    k += 1;
    println!("k: {}", k);

    let input = String::from("Hello World");
    if input == "Hello World" {
        println!("Classic")
    } else {
        println!("Unexpected");
    }

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
}

enum Message {
    Skipped,
    Ok(u32),
    Error { code: u32, msg: String },
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
