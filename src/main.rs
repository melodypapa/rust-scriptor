use rust_scriptor::scriptor;
use rust_scriptor::scriptor::Operable;

fn main() {
    let mut doc = scriptor::ScriptorDoc::new("test", "this is a test", "module");

    //doc.add_heading("heading1").add_heading("heading2");

    doc.for_each_op("as:modconf('MemMap')[1]/MemMapAddressingModeSet");

    doc.write_xml("test.xml");

    println!("{}", doc);

    another_function();
}

fn another_function() {}
