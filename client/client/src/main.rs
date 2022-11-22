use std::net::UdpSocket;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
// use std::time::Duration;

fn main() {
    let mut new_handles = vec![];

    new_handles.push(thread::spawn(move || {
        agent();
    }));

    let mut counter = 4000;
    let address = "localhost:";
    for _ in 0..5 {
        new_handles.push(thread::spawn(move || {
            let new_address = address.to_string() + &counter.to_string();
            // println!("{}", new_address);
            loop {
                let socket =
                    UdpSocket::bind(new_address.clone()).expect("couldn't bind to address");
                let data = "hello world";
                socket
                    .send_to(data.as_bytes(), "localhost:8080")
                    .expect("couldn't send data");

                // receive data
                let mut buf = [0; 1024];
                let (_amt, _src) = socket.recv_from(&mut buf).expect("Didn't receive data");
            }
        }));
        counter += 1;
    }

    for handle in new_handles {
        handle.join().unwrap();
    }
}

fn agent() {
    let thread1_addr = "localhost:8001";
    let thread2_addr = "localhost:8002";
    let thread3_addr = "localhost:8003";

    let agent_address = "localhost:8080";
    let agent_address2 = "localhost:8081";

    let server1_addr = "localhost:9001";
    let server2_addr = "localhost:9002";
    let server3_addr = "localhost:9003";

    let mut handles = vec![];

    // create sender and receiver for 3 threads
    let (sender1, receiver1): (_, Receiver<String>) = channel();
    let (sender2, receiver2): (_, Receiver<String>) = channel();
    let (sender3, receiver3): (_, Receiver<String>) = channel();

    // create sender and receiver for 1 thread
    let (sender4, receiver4): (_, Receiver<String>) = channel();

    handles.push(thread::spawn(move || {
        loop {
            let client = receiver1.recv();
            let message = receiver1.recv();

            let client = client.unwrap();
            let mes = message.unwrap();

            let socket = UdpSocket::bind(thread1_addr).unwrap();

            socket.send_to(mes.as_bytes(), server1_addr).unwrap();

            // listen for response
            let mut buf = [0; 1024];
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            let response = std::str::from_utf8(&buf[..amt]).unwrap();

            println!(
                "client {:?} received response from server {:?}",
                client, src
            );

            // send response back to client
            socket.send_to(response.as_bytes(), client).unwrap();
        }
    }));

    handles.push(thread::spawn(move || {
        loop {
            let client = receiver2.recv();
            let message = receiver2.recv();

            let client = client.unwrap();
            let mes = message.unwrap();

            let socket = UdpSocket::bind(thread2_addr).unwrap();

            socket.send_to(mes.as_bytes(), server2_addr).unwrap();

            // listen for response
            let mut buf = [0; 1024];
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            let response = std::str::from_utf8(&buf[..amt]).unwrap();

            println!(
                "client {:?} received response from server {:?}",
                client, src
            );

            // send response back to client
            socket.send_to(response.as_bytes(), client).unwrap();
        }
    }));

    handles.push(thread::spawn(move || {
        loop {
            let client = receiver3.recv();
            let message = receiver3.recv();

            let client = client.unwrap();
            let mes = message.unwrap();

            let socket = UdpSocket::bind(thread3_addr).unwrap();

            socket.send_to(mes.as_bytes(), server3_addr).unwrap();

            // listen for response
            let mut buf = [0; 1024];
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            let response = std::str::from_utf8(&buf[..amt]).unwrap();

            println!(
                "client {:?} received response from server {:?}",
                client, src
            );

            // send response back to client
            socket.send_to(response.as_bytes(), client).unwrap();
        }
    }));

    let socket = UdpSocket::bind(agent_address).unwrap();

    let mut counter = 0;

    let mut a_up = true;
    let mut b_up = true;
    let mut c_up = true;

    // clone sender 4
    let sender4_clone = sender4.clone();
    handles.push(thread::spawn(move || {
        // send message to parent throufh channel

        // bind on agentAdd2
        let socket2 = UdpSocket::bind(agent_address2).unwrap();
        loop {
            let mut buf = [0; 1024];
            let (amt, src) = socket2.recv_from(&mut buf).unwrap();
            let message = std::str::from_utf8(&buf[..amt]).unwrap();

            println!("received message {:?} from {:?}", message, src);

            // send message to parent thread
            sender4_clone.send(message.to_string()).unwrap();
            sender4_clone.send(message.to_string()).unwrap();
        }
    }));

    loop {
        // check if receiver is empty
        if receiver4.try_recv().is_ok() {
            println!("receiver4 is not empty");

            // read message from receiver
            let response = receiver4.recv().unwrap();

            println!("response: {:?}", response);

            if response == "a up" {
                a_up = true;
            } else if response == "b up" {
                b_up = true;
            } else if response == "c up" {
                c_up = true;
            } else if response == "a down" {
                a_up = false;
            } else if response == "b down" {
                b_up = false;
            } else if response == "c down" {
                c_up = false;
            }
        }

        println!("a_up: {:?}\n", a_up);

        let send1 = sender1.clone();
        let send2 = sender2.clone();
        let send3 = sender3.clone();

        let mut buf = [0; 2048];

        let (amt, src) = socket
            .recv_from(&mut buf)
            .expect("Could not read data from socket");
        // copy the data into a new buffer
        let message = String::from_utf8_lossy(&buf[..amt]);

        let message = message.to_string();

        if a_up == true && b_up == true && c_up == true {
            if counter % 3 == 1 {
                send1.send(src.to_string()).unwrap();
                send1.send(message).unwrap();
            } else if counter % 3 == 2 {
                send2.send(src.to_string()).unwrap();
                send2.send(message).unwrap();
            } else if counter % 3 == 0 {
                send3.send(src.to_string()).unwrap();
                send3.send(message).unwrap();
            }
        } else if a_up == true && b_up == true && c_up == false {
            if counter % 2 == 1 {
                send1.send(src.to_string()).unwrap();
                send1.send(message).unwrap();
            } else if counter % 2 == 0 {
                send2.send(src.to_string()).unwrap();
                send2.send(message).unwrap();
            }
        } else if a_up == true && b_up == false && c_up == true {
            if counter % 2 == 1 {
                send1.send(src.to_string()).unwrap();
                send1.send(message).unwrap();
            } else if counter % 2 == 0 {
                send3.send(src.to_string()).unwrap();
                send3.send(message).unwrap();
            }
        } else if a_up == false && b_up == true && c_up == true {
            if counter % 2 == 1 {
                send2.send(src.to_string()).unwrap();
                send2.send(message).unwrap();
            } else if counter % 2 == 0 {
                send3.send(src.to_string()).unwrap();
                send3.send(message).unwrap();
            }
        }

        // if thread_a_flag == false && a_up == true {
        //     send1.send(src.to_string()).unwrap();
        //     send1.send(message.to_string()).unwrap();
        //     thread_a_flag = true;
        // }
        // else if thread_b_flag == false && b_up == true {
        //     send2.send(src.to_string()).unwrap();
        //     send2.send(message.to_string()).unwrap();
        //     thread_b_flag = true;
        // }
        // else if thread_c_flag == false && c_up == true {
        //     send3.send(src.to_string()).unwrap();
        //     send3.send(message.to_string()).unwrap();
        //     thread_c_flag = true;
        // }
        counter += 1;

        println!("counter: {}", counter);
    }
}
