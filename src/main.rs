use std::env;

fn main() {
    let arg_count = env::args().count();

    if arg_count == 1 {
        loop {
            println!("y");
        }
    } else {
        loop {
            for index in 1..arg_count {
                if let Some(arg) = env::args().nth(index) {
                    print!("{}", arg);
                }
            }

            print!("\n");
        }
    }
}
