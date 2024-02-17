#![allow(unused)]

pub struct Attribute {
    name: String,
    value: Option<String>,
}
pub struct Node {
    tag_name: String,
    attribute_list: Option<Vec<Attribute>>,
    text_content: Option<String>,
    self_closing: bool,
}

impl Node {
    pub fn new(
        tag_name: &str,
        attribute_list: &str,
        text_content: Option<&str>,
        self_closing: bool
    ) -> Node {
        todo!()
        // Node {}
    }
}

impl Attribute {
    pub fn new(name: &str, value: Option<&str>) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: value.map(|text| text.to_string()),
        }
    }
}
