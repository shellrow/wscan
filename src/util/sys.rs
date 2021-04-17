use std::fs;

pub const SPACE4: &str = "    ";

#[allow(dead_code)]
pub enum FillStr{
    Hyphen,
    Equal,
}

pub fn print_fix32(msg: &str, fill_str: FillStr){
    if msg.len() >= 32 {
        println!("{}", msg);
        return;
    }
    match fill_str {
        FillStr::Hyphen => {
            println!("-{}{}",msg,"-".repeat(31 - msg.len()));
        },
        FillStr::Equal => {
            println!("={}{}",msg,"=".repeat(31 - msg.len()));
        },
    }
}

pub fn save_file(file_path: String, data: String){
    let msg = format!("Unable to write file: {}", &file_path);
    fs::write(file_path, data).expect(&msg);
}
