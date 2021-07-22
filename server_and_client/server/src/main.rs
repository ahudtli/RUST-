use std ::net::{TcpListener,TcpStream};
use std::thread;
use std::time;
use std::io;
use std::io::{Write,Read};

//一个简单的模式匹配函数，判断一个数是否大于0
fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) }
    else { Err(false) }
}

//处理客户端请求，并进行回复
fn handle_client(mut stream: TcpStream) -> io::Result<()>{
 
    let mut buf = [0;512];
    for _ in 0..1000{
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0{
            return Ok(());
        }
        
        //将buf转为vec,再转为string，打印
        let message = String::from_utf8(buf.to_vec()).expect("Found invalid UTF-8");
        println!("get client message:{}\n",message);
        println!("send meaasge to client...\n");

        //将buf写入stream中
        stream.write(&buf[..bytes_read])?;
        
        //在两个块中调用简单的匹配函数
    {
        let r = f(10000);
        if let Ok(v) = r {
            println!("Ok: f(-1) = {}", v);
        } else {
            println!("Err");
        }
    }

    {
        let r = f(-10000);
        if let Ok(v) = r {
            println!("Ok: f() = {}\n", v);
        } else {
            println!("Err\n");
        }
    }
        thread::sleep(time::Duration::from_secs(1));
 
    }
    Ok(())
}
 

//main函数入口
fn main() -> io::Result<()>{
    //监听端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    
    for stream in listener.incoming() {
        let stream = stream.expect("failed");
        //
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
        });
        thread_vec.push(handle);
    }
 
    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
 
 
}