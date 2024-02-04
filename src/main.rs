//use std::io;

use rust_scriptor::{front_of_house, scriptor};

fn main() {
    let mut doc = scriptor::ScriptorDoc::new("test", "this is a test", "module");

    doc.add_heading("heading1").add_heading("heading2");

    println!("{}", doc.to_string());
    front_of_house::hosting::add_to_waitlist();
    doc.write_xml("test.xml");

    another_function();
}

fn another_function() {
}
