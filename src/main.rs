use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut file = File::open("data/BATTLE1.LFD").expect("should open");

    let mut buffer = [0; 10];

    let read_amount = file.read(&mut buffer[..])?;

    println!("read_amount {read_amount}");
    println!("read bytes: {:x?}", &buffer[..read_amount]);

    Ok(())
}
