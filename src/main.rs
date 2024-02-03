//use std::io;

use rust_scriptor::{front_of_house, scriptor};

fn main() {
    //let mut guess = String::new();
    //io::stdin().read_line(&mut guess).expect("Failed to read line");
    //println!("{}", guess);

    let doc = scriptor::ScriptorDoc::new("test", "this is a test");

    println!("{}", doc.to_string());
    front_of_house::hosting::add_to_waitlist();
    doc.write_xml("test.xml");

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
    }

    //println!("{:?}", v);

    another_function();
}

fn another_function() {}
