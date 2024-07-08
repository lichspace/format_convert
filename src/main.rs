use std::env;
extern crate imagefmt;
mod format;
mod merge;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);

    let src = &args[1];
    let src2: &String = &args[2];

    if args.len() > 3 && &args[3] == "--over" {
        merge::over(&src, &src2, &args[4]);
    } else {
        println!("convert {} => {}", src, src2);
        format::convert(&src, &src2);
    }
}
