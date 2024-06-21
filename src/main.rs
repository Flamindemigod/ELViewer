mod modules;
use std::{fs::File, io::BufRead};

use anyhow::Result;
use memmap::Mmap;
use modules::parser::Lexer;

fn main() -> Result<()> {
    let file = File::open("Encounter.log").unwrap();
    let mapped_file = unsafe { Mmap::map(&file).unwrap() };
    let file_size = mapped_file.len();
    let mut lexer = Lexer::new(mapped_file.lines());
    while let Some(segment) = lexer.next_segment(){
    println!("{segment:#?}");
    }
    // println!("{:#?}", lexer.data.lines().take(5).collect::<Vec<_>>());
    // for line in mapped_file.lines().take(20) {
    //     println!("{}", line?);
    // }
    // println!("{file_size}");
    Ok(())
}
