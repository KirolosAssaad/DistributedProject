// use std::net::UdpSocket;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    agent();
}


fn agent() {

    let mut handles = vec![];

    // create sender and receiver for 3 threads
    let (sender1, receiver1) = channel();
    let (sender2, receiver2) = channel();
    let (sender3, receiver3) = channel();

    // create 3 threads
    handles.push(thread::spawn(move || {
        // print the received message
        let message = receiver1.recv();
        // print the received message
        // get values from message
        let (x, y, z) = message.unwrap();

        println!("Thread 1 received: {:?}", x);    }));

    handles.push(thread::spawn(move || {
        // print the received message
        let message = receiver2.recv();
        // print the received message
        // get values from message
        let (x, y, z) = message.unwrap();

        println!("Thread 2 received: {:?}", x);    }));

    handles.push(thread::spawn(move || {
        // print the received message
        // println!("Thread 3 received: {}", receiver3.recv().unwrap());
        // read Message from sender
        let message = receiver3.recv();
        // print the received message
        // get values from message
        let (x, y, z) = message.unwrap();

        println!("Thread 3 received: {:?}", x);
    }));


    // struct to hold from address, to address and message
    struct Message {
        from: String,
        to: String,
        message: String,
    }

    // create a dummy message
    let message = Message {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        message: "Hello Bob".to_string(),
    };

    // send message to thread 1
    sender1.send((message.from, message.to, message.message)).unwrap();

        let message = Message {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        message: "Hello Bob".to_string(),
    };

    // send message to thread 2
    sender2.send((message.from, message.to, message.message)).unwrap();

        let message = Message {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        message: "Hello Bob".to_string(),
    };

    // send message to thread 3
    sender3.send((message.from, message.to, message.message)).unwrap();
    // send the message to the threads
    // sender1.send(message).unwrap();
    // sender2.send(message).unwrap();
    // sender3.send(message).unwrap();

    // print non string message
    // println!("Message: {:?}", message);

    // wait for the threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}






    // let mut handles = vec![];

    // // create 3 threads
    // for i in 0..3 {
    //     let receiver = receiver.clone();

    //     let handle = thread::spawn(move || {
    //         let message = receiver.recv().unwrap();
    //         println!("Thread {} received: {}", i, message);
    //     });
        
        
    //     // put thread handle in vector
    //     handles.push(handle);

    // }
