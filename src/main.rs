
use std::io::{BufRead, BufReader};
// use once_cell::sync::OnceCell;
use std::{net::TcpStream, io::Write};

mod credentials;
fn main() {
    let socket : TcpStream = TcpStream::connect("irc.chat.twitch.tv:6667").unwrap();
    let credentials = credentials::get_credentials();
    println!("{}", credentials.oauth);
    send_raw(&socket, &format!("PASS {}", &*credentials.oauth));
    send_raw(&socket, &format!("NICK {}", &*credentials.botname));
    send_raw(&socket, &format!("JOIN {}", &*credentials.channel));


    send_message(&socket, &*credentials.channel,"Connected to chat".to_string());
    println!("Connection to the channel #{} has been established!\n",&*credentials.channel);

    let mut reader = BufReader::new(socket.try_clone().unwrap());

    let mut line = String::new();
    loop {
        reader.read_line(&mut line);
        println!("{}", line);
    }
}

fn send_raw(socket : &TcpStream, data : &str) {
    let msg = String::from(format!("{}\r\n", data));
    let _result = (&*socket).write(msg.as_bytes()).expect("failed");
}

fn send_message(socket: &TcpStream, channel : &str, data : String){
    let msg = String::from(format!("PRIVMSG #{} :{}\r\n", &*channel, data));
    let _result = (&*socket).write(msg.as_bytes()).expect("send_raw failed!"); 
}