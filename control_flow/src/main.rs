use std::io;

fn main() {
    let mut buf = String::new();

    loop {
        loop {
            buf.clear();
            io::stdin().read_line(&mut buf).expect("sirr error");

            println!("this what buf have {buf}");

            // let number: i32 = buf.trim().parse();
            // if returning a Result we need to extract the Err and Ok
            // so we need a match or using expect()
            let number: i32 = buf.trim().parse().expect("please set a number");

            if number < 10 {
                println!("less of expected!");
            } else {
                println!("this bigger! ");
                break;
            }
        }
        println!("we exit here?");
    }
}
