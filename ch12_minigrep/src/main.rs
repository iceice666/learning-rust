use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = arg_parser(&args);

    println!("搜尋 {}", args.query);
    println!("目標檔案為 {}", args.file_path);

    let contents = fs::read_to_string(args.file_path).expect("應該要能夠讀取檔案");

    println!("文字內容：\n{contents}");
}

struct Args {
    query: String,
    file_path: String,
}

fn arg_parser(args: &[String]) -> Args {
    let query = args[1].to_string();
    let file_path = args[2].to_string();
    Args { query, file_path }
}
