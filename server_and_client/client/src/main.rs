use std::io::{self,prelude::*,BufReader,Write};
use std::net::TcpStream;
use std::str;
 
 
fn main() -> io::Result<( )> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    for _ in 0..1000 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        stream.write(input.as_bytes()).expect("failed to write");
 
        let mut reader =BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\n',&mut buffer)?;
        println!("read form server:{}",str::from_utf8(&buffer).unwrap());
        println!("");
 
    }
    Ok(())
}
