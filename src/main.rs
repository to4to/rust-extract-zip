use std::fs;
use std::io;

fn main() {
    // println!("Hello, world!");

    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return 1;
    }

    


    let fname=std::path::Path::new(&*args[1]);

}
