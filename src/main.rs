use std::env;

fn main() {
    let argc = env::args().count();

    if argc == 1 {
        loop {
            println!("y");
        }
    } else {
        loop {
            for i in 1..argc {
                match env::args().nth(i) {
                    Some(x) => print!("{}", x),
                    None => continue,
                }
            }

            print!("\n");
        }
    }
}
