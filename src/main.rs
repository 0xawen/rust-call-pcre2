use rust_call_pcre2::{find, send_by_udp};
use std::fs;

fn main(){
    let pattern = r"\d{4}[^\d\s]{3,11}";
    let subject = fs::read_to_string("./test.txt").unwrap();

    let data = find(pattern , subject.as_str());
    send_by_udp("127.0.0.1:8381", "127.0.0.1:8383",data);
}