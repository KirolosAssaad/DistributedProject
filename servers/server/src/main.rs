//create server that communicates with another server using udp 

use std::net::UdpSocket;
use std::str;
use std::thread;
use std::time::Duration;
use std::string::String;
use rand::Rng;


fn generate_priority_number() -> i32 {
    let mut rng = rand::thread_rng();
    let priority_number: i32 = rng.gen_range(1..101);
    return priority_number;
}

//this server acts as a client to another server recievies data through connection and sends it to another server
//without reading from a file
fn main() {
    let socket = UdpSocket::bind("10.40.41.134:8080").expect("couldn't bind to address");
    let socket2 = socket.try_clone().expect("couldn't clone socket");

    let socket3 = UdpSocket::bind("10.40.41.134:8080").expect("couldn't bind to address");
    let socket4 = socket.try_clone().expect("couldn't clone socket");
    
    let mut buf = [0; 30];

    //create a unique ID for the server
    //not necessaarly needed to be random
    let server_id = rand::thread_rng().gen_range(1..101);
    println!("server id: {}", server_id);
    
   
    //create a thread  socket to keep listening (recieving )for data
    thread::spawn(move || {
        loop {
            let (amt, src) = socket.recv_from(&mut buf).expect("Didn't recieve data");
            println!("{} bytes recieved from {}", amt, src);
            println!("data recieved: {}", str::from_utf8(&buf).unwrap());
        }
    });
      //send data using a thread and the cloned socket
    thread::spawn(move || {
        loop {
            let data = "hello from client";
            socket2.send_to(data.as_bytes(), "xx.xx.xx.xx:8080").expect("couldn't send data"); //server 1
            socket2.send_to(data.as_bytes(), "xx.xx.xx.xx:8080").expect("couldn't send data"); //server 2
            thread::sleep(Duration::from_secs(1));
        }
    });

    


     //loop to recieve the priority number from the other servers within a sepreate thread and compares the priority numbers 
    //to determine which server has the highest priority number
     //recieve from the agent
     //recieve from the other servers
     //compare the priority numbers
     thread::spawn(move || {
        loop {
            let (amt, src) = socket4.recv_from(&mut buf).expect("Didn't recieve data");
            println!("{} bytes recieved from {}", amt, src);
            println!("data recieved: {}", str::from_utf8(&buf).unwrap());
            //store the data recieved from the other servers in a variable
           // let priority_numberB = str::from_utf8(&buf).unwrap();
        }
    });
    //send the priority number to the other servers

    thread::spawn(move || {
        loop {
            let priority_number = generate_priority_number();
            //send i32 as a string
            socket3.send_to(priority_number.to_string().as_bytes(), "xx.xx.xx.xx:8080").expect("couldn't send data"); //send to server 1
            socket3.send_to(priority_number.to_string().as_bytes(), "xx.xx.xx.xx:8080").expect("couldn't send data"); //send to server 2
            thread::sleep(Duration::from_secs(60));
        }
    });
    //keep main thread alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }

    //missing
    //nawa'3 l server 

}

