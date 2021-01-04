use curl::easy::Easy;
use std::io::{stdout, Write};

fn main() {
    get_response_time();
    get_response_code();
    print_page_content();
}

fn print_page_content() {
    let mut easy = Easy::new();
    easy.url("https://www.google.com/").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

fn get_reponse_code() {
    let mut easy = Easy::new();
    easy.url("https://www.google.com/").unwrap();
    easy.write_function(|data| {
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap())
}

fn get_response_time() {
    let mut easy = Easy::new();
    easy.url("https://www.google.com/").unwrap();
    easy.write_function(|data| {
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{:?}", easy.total_time().unwrap());
}
