// importing needed libraries
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
// use std::io::Error;



fn main() {
    // println!("Hello, world!");
    // let path = Path::new("./machine1/test3/server1");

    // let dir = "./mach1/test3/success";
    parse_dir("./mach1/test3/success");
    parse_dir("./mach2/test3/success");
}

// function tgat parses all files in a passed directory
fn parse_dir(dir: &str) {
    // create a path to the desired directory
    let path = Path::new(dir);

    // create a vector to hold the paths of all files in the directory
    let mut files: Vec<PathBuf> = Vec::new();

    // iterate over the paths in the directory
    for entry in std::fs::read_dir(path).unwrap() {
        // get the path of the entry
        let entry = entry.unwrap();
        let path = entry.path();

        // if the path is a file, add it to the vector
        if path.is_file() {
            files.push(path);
        }
    }

    // iterate over the files in the vector
    // let temp = files.clone();
    let mut sum :f32= 0.0;
    let mut counter = 0;
    for file in files {
        // print file name
        // println!("File: {:?}", file);
        // open the file
        let mut f = File::open(file).unwrap();

        // create a string to hold the contents of the file
        let mut contents = String::new();

        // read first line of file
        f.read_to_string(&mut contents).unwrap();

        // get first line of file
        let mut lines = contents.lines();

        // get the first line
        let line = lines.next().unwrap();

        // convert the string to a float
        let num: f32 = line.trim().parse().unwrap();


        // println!("{}", num);

        // add the number to the sum
        sum += num;

        counter += 1;

        // println!("sum: {}", sum);
        // println!("count: {}", counter);
        // print the contents of the file
        // println!("{}", contents);
    }

    // print the sum
    println!("{}", sum);

    // print the average
    println!("average {}", sum / 500.0);





}

