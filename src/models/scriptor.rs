
//const ELEMENT_DESC:String = "Description";
#[derive(Debug)]
pub struct ScriptorDoc {
    name: String,
    desc: String,
}

impl ScriptorDoc {
    pub fn new(name: &str, desc: &str) -> Self {
        ScriptorDoc {name: String::from(name), desc: String::from(desc)}
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
    }
}