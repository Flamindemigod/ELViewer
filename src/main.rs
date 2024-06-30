mod modules;
use std::{fs::File, io::BufRead};

use anyhow::Result;
use memmap::Mmap;
use modules::parser::Lexer;

fn main() -> Result<()> {
    let file = File::open("Encounter2.log").unwrap();
    let mapped_file = unsafe { Mmap::map(&file).unwrap() };
    let mut lexer = Lexer::new(mapped_file.lines());
    let mut segment_array = vec![];
    while let Some(segment) = lexer.next_segment() {
        segment_array.push(segment);
        // println!("{segment:#?}");
    }
    drop(mapped_file);
    drop(file);
    println!("Done Parsing: segment array len = {}", segment_array.len());
    // println!("{:#?}", lexer.data.lines().take(5).collect::<Vec<_>>());
    // for line in mapped_file.lines().take(20) {
    //     println!("{}", line?);
    // }
    // println!("{file_size}");
    Ok(())
}
