extern crate imagefmt;
mod format;
mod merge;
use clap::{Arg, Command, ArgAction};

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
        .arg(Arg::new("over").help("叠加").long("over").action(ArgAction::SetTrue))
        .get_matches();

    let inputs = matches
        .get_many::<String>("input")
        .unwrap()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    // println!("{:?}", inputs);

    // 获取输入值
    let output = matches.get_one::<String>("output").unwrap();
    // println!("{:?}", output);

    let &over = matches.get_one::<bool>("over").unwrap();
    let src: &str = inputs[0];

    if over && inputs.len() >= 2 {
        merge::over_multi(inputs, output);
    } else {
        println!("convert {} => {}", src, output);
        format::convert(src, output);
    }
}
