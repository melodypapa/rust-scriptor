

//const ELEMENT_DESC:String = "Description";
pub struct ScriptorDoc {
    name: String,
    desc: String,
}

impl ScriptorDoc {
    pub fn new(name: &str, desc: &str) -> Self {
        ScriptorDoc {name: String::from(name), desc: String::from(desc)}
    }
}