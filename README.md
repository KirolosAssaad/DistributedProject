# Distributed Systems Project

Submitted by: 
| Name | ID |
| --- | --- |
| [Nour Montasser](https://github.com/nourmontasser) | 900191101 |
| [Mariam Ramadan](https://github.com/mariamramadan1) | 900191779 |
| [Kirolos Mikhail](https://github.com/KirolosAssaad) | 900191250 |

Submitted to: Dr. Amr Elkadi

***
## Project Description

The project is a distributed system that simulates 2 clients and 3 servers. The clients are responsible for sending requests to the servers through an agent, which is responsible for ballancing the load and the servers are responsible for processing the requests and sending the response back to the clients. The project is implemented using Rust Programming Language and the communication between the clients and the servers is done using UDP protocol.

***
## Project Structure

The project is divided into 3 main parts:

1. [The client part](client/client/src/main.rs)
2. [The agent part](client/client/src/main.rs)
3. [The server part](servers/server/src/main.rs)
4. [machine 1 test results](stats/mach1)
5. [machine 2 test results](stats/mach2)
6. [The result parser](stats/src/main.rs)
7. [The technical report](Distributed_ProjectReport.pdf)



***

## Client Part

The client part is responsible for sending requests to the servers through the agent. The client sends a request to the agent and the agent sends the request to the server using the round robin algorithm. The client waits for the response from the server and then sends it to the user.

***

## Agent Part

The agent is responsible for ballancing the load between the servers. The agent receives the request from the client and sends it to the server using the round robin algorithm. The agent also receives the response from the server and sends it to the client.

***

## Server Part

The server is responsible for processing the requests and sending the response back to the client. The server receives the request from the agent and processes it. The server then sends the response back to the agent and the agent sends it to the client. The server also keeps track of the number of requests it receives and the number of requests it processes. It also starts an election algorithm once a minute to elect which server will fail for 15 seconds.

***

## Test Results

The test results are stored in the [stats](stats) folder. The results of each machine is stored in a separate folder under the names [mach1](stats/mach1) and [mach2](stats/mach2).

***

## Result Parser

The parser parses the avg, failed and success folders which have 500 files each per machine as each thread writes to a separate file. The parser calculates the sum and average of each.

***

## Technical Report

The technical report is stored in the [Distributed_ProjectReport.pdf](Distributed_ProjectReport.pdf) file.

***

## How to run the project

1. Clone the project from [here](https://github.com/KirolosAssaad/DistributedProject.git)
2. Open the project in [vscode](https://code.visualstudio.com/)
3. Setup 5 machines (3 servers, 2 clients)
4. Install [rust](https://www.rust-lang.org/tools/install) on all machines
5. Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) on all machines
6. Install [git](https://git-scm.com/downloads) on all machines
7. get the server code on 3 machines
8. get the client code on 2 machines
9. get the IP of the 3 machines
10. Setup the IP of the 3 machines in the server code and the client code.
11. Run the server code on 3 machines using the command `cargo run`
12. Finally, run the client code on 2 machines using the command `cargo run`
13. Code runs infinitely, to stop it, press `ctrl + c` on all machines

***

## How to run the parser
run the parser using the command `cargo run` in the stats folder

