use std::env;

fn main() {
    let args_count = env::args().count();

    if args_count == 1 {
        loop {
            println!("y");
        }
    } else {
        loop {
            for index in 1..args_count {
                if let Some(arg) = env::args().nth(index) {
                    print!("{}", arg);
                }
            }

            print!("\n");
        }
    }
}
