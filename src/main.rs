extern crate imagefmt;
mod format;
mod merge;
use clap::{Arg, Command, ArgAction};
use std::time::Instant;

fn main() {
    let matches = Command::new("convert")
        .about("picture process")
        .arg(
            Arg::new("input")
                .help("输入文件")
                .long("input")
                .short('i')
                .value_delimiter(',')
                .required(true),
        )
        .arg(
            Arg::new("output")
                .help("输出地址")
                .long("output")
                .short('o')
                .required(true),
        )
        .arg(Arg::new("transparent").help("是否透明背景").long("transparent").action(ArgAction::SetTrue))
        .arg(Arg::new("over").help("是否叠加").long("over").action(ArgAction::SetTrue))
        .get_matches();

    let inputs = matches
        .get_many::<String>("input")
        .unwrap()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    // println!("{:?}", inputs);

    // 获取输入值
    let output = matches.get_one::<String>("output").unwrap();

    let &over = matches.get_one::<bool>("over").unwrap();
    let &transparent = matches.get_one::<bool>("transparent").unwrap();
    let src: &str = inputs[0];

    let start = Instant::now();

    if over {
        merge::over_multi(inputs, output, transparent);
    } else {
        println!("convert {} => {}", src, output);
        format::convert(src, output, transparent);
    }

    let duration = start.elapsed();
    println!("convert time: {:?}", duration);
}
