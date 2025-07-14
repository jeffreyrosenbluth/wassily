use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let decoder = png::Decoder::new(File::open(&args[1]).unwrap());
    let reader = decoder.read_info().unwrap();
    for text_chunk in &reader.info().uncompressed_latin1_text {
        println!("{}", text_chunk.keyword);
        println!("-------------------------");
        println!("{}", text_chunk.text);
        println!();
    }
}
