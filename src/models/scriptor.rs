use std::fs::File;

use xml::{writer::XmlEvent, EmitterConfig, EventWriter};

//const ELEMENT_DESC:String = "Description";
#[derive(Debug)]
pub struct ScriptorDoc {
    name: String,
    desc: String,
    headings: Vec<String>,
}

impl ScriptorDoc {
    pub fn new(name: &str, desc: &str) -> Self {
        let empty_headings = Vec::new();
        ScriptorDoc {
            name: String::from(name),
            desc: String::from(desc),
            headings: empty_headings,
        }
    }
  
    pub fn add_heading(&self, heading: &str) {
        //self.headings.push(String::from(heading));
    }

    fn write_headings(&self, writer: &EventWriter<File>) {
        //for heading in self.headings{
        //    writer.write(XmlEvent::comment(&heading));
        //}
    }

    fn write_document(&self, writer:&EventWriter<File>){
        //writer.write(XmlEvent::start_element("Script")).expect("Write heading error");

        //writer.write(XmlEvent::end_element());
    }

    pub fn write_xml(&self, filename: &str) {
        let fs = File::create(filename).expect(format!("Cannot create {}", filename).as_str());
        
        let mut writer: EventWriter<File> = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(fs);

        self.write_document(&writer);
        
        //let element = XmlEvent::start_element("a:hello").attr("a:param", "value").ns("a", "urn:some:document");
    }

    pub fn to_string(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        lines.push(format!("Name: {}", self.name));
        lines.push(format!("Desc: {}", self.desc));

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::ScriptorDoc;

    #[test]
    fn create_scriptor_doc() {
        let doc = ScriptorDoc::new("demo", "it is a demo");

        assert_eq!(doc.name, "demo");
        assert_eq!(doc.desc, "it is a demo");
        //assert_eq!(doc.to_string(), "");
    }
}
