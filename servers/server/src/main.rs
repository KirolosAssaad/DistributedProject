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

//function that checks the physical time of the machine , if the seconds is zero then it will return true
//this is used to keep track of the time to send the new priority number to the other servers
fn check_time() -> bool {
    let time = std::time::SystemTime::now();
    let time = time.duration_since(std::time::UNIX_EPOCH).unwrap();
    let time = time.as_secs();
    let time = time % 60;
    if time == 0 {
        return true;
    }
    return false;
}


fn main() {
    let socket = UdpSocket::bind("10.40.41.134:8080").expect("couldn't bind to address");
    let socket2 = socket.try_clone().expect("couldn't clone socket");

    let socket3 = UdpSocket::bind("10.40.41.134:8080").expect("couldn't bind to address");
    let socket4 = socket.try_clone().expect("couldn't clone socket");
    
    let mut buf = [0; 30];

    //create a unique ID for the server
    //not necessaarly needed to be random
    let server_id = rand::thread_rng().gen_range(1..101);
    //println!("server id: {}", server_id);

    
     //send data using a thread and the cloned socket
        thread::spawn(move || {
            loop {
                //if check_time() returns true then send the new priority number to the other servers
                if check_time() {
                    let priority_number = generate_priority_number();
                    let priority_number = priority_number.to_string();
                    let priority_number = priority_number.as_bytes();
                    socket2.send_to(priority_number, "xx.xx.xx.xx:8080").expect("couldn't send data");
                    socket2.send_to(priority_number, "xx.xx.xx.xx:8080").expect("couldn't send data");
            }
        });
    
    //create a thread  socket to keep listening (recieving )for data
    thread::spawn(move || {
        loop {
            let (amt, src) = socket.recv_from(&mut buf).expect("Didn't recieve data");
            //another recieve from socket
            let (amt2, src2) = socket.recv_from(&mut buf).expect("Didn't recieve data");
            //println!("{} bytes recieved from {}", amt, src);
            //println!("data recieved: {}", str::from_utf8(&buf).unwrap());
            //store the first recieved data in a string
            let mut data = str::from_utf8(&buf).unwrap();
            //store the second recieved data in a string
            let mut data2 = str::from_utf8(&buf).unwrap();
            //change them to integers
            let data_int = data.parse::<i32>().unwrap();
            let data2_int = data2.parse::<i32>().unwrap();
            //compare the three integers, data and data2 and the priority number
            //and if priority number is greater than both data and data2,sleep the main thread for 5 seconds else do nothing
            if priority_number > data_int && priority_number > data2_int {
                thread::sleep(Duration::from_secs(5));
            }

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

    
    //keep main thread alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }

    //missing
    //nawa'3 l server 

}

