use std::net::UdpSocket;
use std::sync::mpsc::channel;
use std::thread;
// use std::time::Duration;

fn main() {
    agent();
}


fn agent() {

        // struct to hold from address, to address and message
    // struct Message {
    //     from: String,
    //     to: String,
    //     message: String,
    // }

    // create a dummy message
    // let message = Message {
    //     from: "Alice".to_string(),
    //     to: "Bob".to_string(),
    //     message: "Hello Bob".to_string(),
    // };

    // // create a dummy message
    // let message2 = Message {
    //     from: "Alice".to_string(),
    //     to: "Bob".to_string(),
    //     message: "Hello Bob".to_string(),
    // };
    // // create a dummy message
    // let message3 = Message {
    //     from: "Alice".to_string(),
    //     to: "Bob".to_string(),
    //     message: "Hello Bob".to_string(),
    // };

    let mut handles = vec![];



    let socket = UdpSocket::bind("localhost:8080").unwrap();


    // create sender and receiver for 3 threads
    let (sender1, receiver1) = channel();
    let (sender2, receiver2) = channel();
    let (sender3, receiver3) = channel();

    // let (child_sender, child_receiver) = channel();


    // let sender_1 = child_sender.clone();
    handles.push(thread::spawn(move || {

        loop
        {

            let message = receiver1.recv();

            let mes = message.unwrap();

            println!("Thread 1 received: {:?}", mes);  
        }
        // sender_1.send("Thread 1 is done").unwrap();

    }));



    // let sender_2 = child_sender.clone();

    handles.push(thread::spawn(move || {

        loop
        {
            let message = receiver2.recv();

            let mes = message.unwrap();

            println!("Thread 2 received: {:?}", mes);  
        }
        // sender_2.send("Thread 2 is done").unwrap();
    }));



    // let sender_3 = child_sender.clone();

    handles.push(thread::spawn(move || {

        loop
        {
            let message = receiver3.recv();

            let mes = message.unwrap();

            println!("Thread 3 received: {:?}", mes);
        }
        // sender_3.send("Thread 3 is done").unwrap();
    }));


    let mut counter = 0;
    loop{
        let send1 = sender1.clone();
        let send2 = sender2.clone();
        let send3 = sender3.clone();


        let mut buf = [0; 2048];
        let (amt, _src) = socket.recv_from(&mut buf).expect("Could not read data from socket");
        // copy the data into a new buffer
        let message = String::from_utf8_lossy(&buf[..amt]);

        let message = message.to_string();
        

        if counter %3 == 0{
            send1.send(message.to_string()).unwrap();
        }
        else if counter %3 == 1{
            send2.send(message.to_string()).unwrap();
        }
        else if counter %3 == 2{
            send3.send(message.to_string()).unwrap();
        }
        counter += 1;

    }

    // // receive message from child thread
    // drop(child_sender);
    // for receive in child_receiver {
    //     println!("Main thread received: {}", receive);
    // }



    // // wait for the threads to finish
    // for handle in handles {
    //     handle.join().unwrap();
    // }
}




