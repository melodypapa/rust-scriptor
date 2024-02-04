use std::fs::File;

use xml::{writer::XmlEvent, EmitterConfig, EventWriter};

const ELEMENT_NAME: &str = "Name";
const ELEMENT_DESC: &str = "Description";
const ELEMENT_EXPRESSION: &str = "Expression";

pub struct ScriptorDoc {
    name: &'static str,
    desc: &'static str,
    module: &'static str,
    headings: Vec<&'static str>,
    operations: Vec<Operation>,
}

impl ScriptorDoc {
    pub fn new(name: &'static str, desc: &'static str, module: &'static str) -> Self {
        ScriptorDoc {
            name,
            desc,
            module,
            headings: Vec::new(),
            operations: Vec::new(),
        }
    }

    pub fn add_heading(&mut self, heading: &'static str) -> &mut ScriptorDoc {
        self.headings.push(heading);
        return self;
    }

    fn write_headings(&self, writer: &mut EventWriter<File>) {
        for heading in &self.headings {
            writer
                .write(XmlEvent::comment(&heading))
                .expect("XML failure: write headings");
        }
    }

    fn write_element(&self, writer: &mut EventWriter<File>, name: &str, value: &str) {
        writer
            .write(XmlEvent::start_element(name))
            .expect("XML failure: cannot write start element");
        writer
            .write(XmlEvent::characters(value))
            .expect("XML failure: cannot write character");
        writer
            .write(XmlEvent::end_element())
            .expect("XML failure: cannot write end element");
    }

    fn write_scriptor_head(&self, writer: &mut EventWriter<File>) {
        self.write_element(writer, ELEMENT_NAME, self.name);
        self.write_element(writer, ELEMENT_DESC, self.desc);
        self.write_element(
            writer,
            ELEMENT_EXPRESSION,
            format!("as:modconf('{}')", self.module).as_str(),
        );
    }

    fn write_document(&self, writer: &mut EventWriter<File>) {
        writer
            .write(XmlEvent::start_element("Script"))
            .expect("XML failure: cannot write start element");

        self.write_headings(writer);
        self.write_scriptor_head(writer);

        writer
            .write(XmlEvent::end_element())
            .expect("XML failure: cannot write end element");
    }

    pub fn write_xml(&self, filename: &str) {
        let fs = File::create(filename).expect(format!("Cannot create {}", filename).as_str());

        let mut writer: EventWriter<File> =
            EmitterConfig::new().perform_indent(true).create_writer(fs);

        self.write_document(&mut writer);

        //let element = XmlEvent::start_element("a:hello").attr("a:param", "value").ns("a", "urn:some:document");
    }

    pub fn to_string(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        lines.push(format!("Name: {}", self.name));
        lines.push(format!("Desc: {}", self.desc));
        lines.push(format!("Module: {}", self.module));

        lines.push(String::from("Headings:"));
        for heading in &self.headings {
            lines.push(format!("  {}", heading));
        }

        lines.join("\n")
    }
}

pub enum OperationType {
    Condition,
    ForEach,
    Add,
    Remove,
    SetValue,
    SetEnabled,
    Document(ScriptorDoc),
}

pub struct Operation {
    op_type: OperationType,
    expression: &'static str,
    operations: Vec<Operation>,
}

impl Operation {
    pub fn new(op_type: OperationType, expression: &'static str) -> Self {
        Self {
            op_type,
            expression,
            operations: Vec::new(),
        }
    }

    pub fn condition_op(expression: &'static str) -> Self {
        Self::new(OperationType::Condition, expression)
    }

    pub fn for_each_op(expression: &'static str) -> Self {
        Self::new(OperationType::ForEach, expression)
    }
}

#[cfg(test)]
mod tests {
    use super::ScriptorDoc;

    #[test]
    fn create_scriptor_doc() {
        let mut doc = ScriptorDoc::new("demo", "it is a demo", "MemMap");

        doc.add_heading("heading1").add_heading("heading2");

        assert_eq!(doc.name, "demo");
        assert_eq!(doc.desc, "it is a demo");

        let text = doc.to_string();
        let lines: Vec<&str> = text.split("\n").collect();

        assert_eq!(lines[0], "Name: demo");
        assert_eq!(lines[1], "Desc: it is a demo");
        assert_eq!(lines[2], "Module: MemMap");
        assert_eq!(lines[3], "Headings:");
        assert_eq!(lines[4], "  heading1");
        assert_eq!(lines[5], "  heading2");

        //assert_eq!(doc.to_string(), "");
    }
}
