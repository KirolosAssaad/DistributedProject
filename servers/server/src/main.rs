//create server that communicates with another server using udp

//use std::borrow::Borrow;
use std::net::UdpSocket;
use std::str;
use std::thread;
use std::time::Duration;
//use std::string::String;
use rand::Rng;
use std::sync::mpsc::{ channel, Receiver };

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
    let socket = UdpSocket::bind("10.7.57.176:8080").expect("couldn't bind to address");
    let socket2 = socket.try_clone().expect("couldn't clone socket");

    let socket3 = UdpSocket::bind("10.7.57.176:8081").expect("couldn't bind to address");
    let socket4 = socket3.try_clone().expect("couldn't clone socket");

    let mut socket5 = UdpSocket::bind("10.7.57.176:8082").expect("couldn't bind to address");
    let socket6 = socket5.try_clone().expect("couldn't clone socket");

    let mut buf = [0; 300];

    //create a vector to hold the threads created
    let mut threads = vec![];

    //create a unique ID for the server
    //not necessaarly needed to be random
    let server_id = rand::thread_rng().gen_range(1..101);
    //println!("server id: {}", server_id);
    // let mut priority_number = generate_priority_number();

    let (sender, receiver): (_, Receiver<String>) = channel();
    let (sender3, receiver3): (_, Receiver<i32>) = channel();
    let (ipagents_sender, ipagents_receiver) = channel();
    //let agent_ips = agent_ip;
    //clone agent_ip vector to be used in the threads
    //let agent_ips = agent_ip.clone();

    //recieve from agent its ip address and stores it in a vector
    threads.push(
        thread::spawn(move || {
            loop {
                //recieve from agent its ip address and stores it in a vector

                let (amt, src) = socket3.recv_from(&mut buf).expect("Didn't recieve data");
                ipagents_sender.send(src);
                println!("sent agent ip: {:?}", src);
            }
        })
    );

    //send data using a thread and the cloned socket
    //push to threads vector

    threads.push(
        thread::spawn(move || {
            loop {
                //if check_time() returns true then send the new priority number to the other servers
                if check_time() {
                    let mut priority_number = generate_priority_number();
                    sender3.send(priority_number);
                    let priority_number = priority_number.to_string();
                    let priority_number = priority_number.as_bytes();
                    socket2
                        .send_to(priority_number, "10.7.57.73:8080")
                        .expect("couldn't send data");
                    socket2
                        .send_to(priority_number, "10.7.57.80:8080")
                        .expect("couldn't send data");
                    thread::sleep(Duration::from_secs(1));
                }
            }
        })
    );

    //create a thread  socket to keep listening (recieving )for data
    let mut buf2 = [0; 300];

    threads.push(
        thread::spawn(move || {
            loop {
                let (amt, src) = socket.recv_from(&mut buf2).expect("Didn't recieve data");

                let data = str::from_utf8(&buf2[..amt]);
                thread::sleep(Duration::from_secs(1));

                let mut buf4 = buf2.clone();
                //another recieve from socket
                let (amt2, src2) = socket.recv_from(&mut buf4).expect("Didn't recieve data");
                let data2 = str::from_utf8(&buf4[..amt2]);
                //println!("{} bytes recieved from {}", amt, src);
                //println!("{} bytes recieved from {}", amt2, src2);
                //println!("data recieved: {}", str::from_utf8(&buf).unwrap());
                //store the first recieved data in a string
                //store the second recieved data in a string

                //print data and data2
                println!("data recieved: {:?}", data);
                println!("data recieved: {:?}", data2);
                //change them to integers
                let data_int = data.expect("error parsing").parse::<i32>().unwrap();
                let data2_int = data2.expect("error parsing").parse::<i32>().unwrap();
                //compare the three integers, data and data2 and the priority number
                //and if priority number is greater than both data and data2,sleep the main thread for 5 seconds else do nothing
                let priority_number = receiver3.recv().unwrap();
                println!("prio {:?}", priority_number);
                if priority_number > data_int {
                    if priority_number > data2_int {
                        println!("should go down");
                        //create a channel

                        //send message through sender
                        sender.send("sleep".to_string());
                        //send to the ip addresses in the vector agent_ip that this server "server_id" is down
                    }
                }
            }
        })
    );

    //thread that recieves from an agent and send back to the same ip address it recieved from
    //clone reciver channel to be used in the thread
    let mut buf3 = [0; 300];
    threads.push(
        thread::spawn(move || {
            let mut agent_ip = vec![];
            loop {

                // if agent_ip.len() != 0
                // {
                //     println!("{:?}", agent_ip[0]);
                // }
                //if recive try is okay then sleep
                match ipagents_receiver.try_recv() {
                    Ok(response) => {
                        agent_ip.push(response);
                    }
                    Err(e) => {
                        // println!("error receiving agents ip: {:?}", e);

                    }
                }

                let (amt, src) = socket5.recv_from(&mut buf3).expect("Didn't recieve data");
               
                if receiver.try_recv().is_ok() {
                    println!("here");
                     for agent in &agent_ip {
                    socket5
                        .send_to("a down".as_bytes(), agent)
                        .expect("couldn't send data");
                        println!("down loop:{:?}",agent);
                    }

                    println!("going down");
                    // socket5.shutdown(std::net::Shutdown::Both);
                    // drop(socket5);
                    thread::sleep(Duration::from_secs(15));
                    // socket5 = UdpSocket::bind("10.7.57.80:8082").unwrap();
                    println!("going up");
                    for agent in &agent_ip {
                    socket5
                        .send_to("a up".as_bytes(), agent)
                        .expect("couldn't send data");
                        println!("up loop:{:?}",agent);

                    }
                    continue;
                }
                //recieve from agent a message and send it back to the same ip address it recieved from
                //send back a message to the agent " message recived"
                socket5.send_to("message recieved".as_bytes(), src).expect("couldn't send data");
            }
        })
    );

    //join all the threads
    for thread in threads {
        thread.join();
    }
}