//use std::io;

use rust_scriptor::scriptor::ScriptorDoc;

fn main() {
    println!("Hello, world!");

    //let mut guess = String::new();
    //io::stdin().read_line(&mut guess).expect("Failed to read line");
    //println!("{}", guess);

    let doc = ScriptorDoc::new("test", "test");

    println!("{:?}", doc);

    another_function();
}

fn another_function() {

}
