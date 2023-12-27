#![allow(dead_code, unused)]

fn main() {
    let html_string =
        r#"<table attribute="21 hio pstro" attribute2=1 attribute3='hioh groch pstroch' attribute4='hioh' attribute5>wdwe<tbody><tr a1=2 a2 a3><td arg><input hio=pstro barachlo /></td><td>##TEXT##</td></tr><tr><td>hiopop</td><td groho paproho=sio>heh</td></tr></tbody></table>"#;
    // let html_string =
    // "<table><tbody><tr a1=2 a2 a3><td arg><input hio=pstro barachlo /></td><td>##TEXT##</td></tr><tr><td>hiopop</td><td groho paproho=sio>heh</td></tr></tbody></table>";
    let output =
        "<table attribute=\"21 hio pstro\" attribute2=1 attribute3='hioh groch pstroch' attribute4='hioh' attribute5>wdwe<tbody><tr a1=2 a2 a3><td arg><input hio=pstro barachlo/></td><td>##TEXT##</td></tr><tr><td>hiopop</td><td groho paproho=sio>heh</td></tr></tbody></table>";
    let parsed: Node = parse(html_string);
    let stringified = parsed.to_html();
    dbg!(&stringified);
}

impl Attribute {
    fn to_string(&self) -> String {
        let result = match self.value {
            Some(value) => format!("{}={}", self.name, value),
            _ => self.name.to_string(),
        };
        result
    }
}

impl Node {
    fn to_html(&self) -> String {
        let attributes = match &self.attributes {
            Some(attributes) => {
                format!(
                    " {}",
                    attributes
                        .iter()
                        .map(|attribute| attribute.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            None => "".to_string(),
        };
        if self.self_closing {
            return format!("<{}{}/>", self.name, attributes);
        }
        let text_content = match &self.text_content {
            Some(text) => text,
            None => "",
        };

        let children = match &self.children {
            None => "".to_string(),
            Some(children) =>
                children
                    .iter()
                    .map(|node| node.to_html())
                    .collect(),
        };
        let close_tag = format!("</{}>", self.name);
        format!("<{}{}>{}{}</{}>", self.name, attributes, text_content, children, self.name)
        // let mut child = format!("<");
    }
}

fn parse(html_string: &'static str) -> Node {
    let mut nesting_stack: Vec<Node> = Vec::new();
    let mut html_string = html_string;
    while !html_string.is_empty() {
        let (node, html_string_local) = get_next_node(html_string).unwrap();
        let self_closing = node.ends_with("/>");
        let (tag_name, node) = get_tag_name(node).unwrap();
        let closing_tag = tag_name.starts_with('/');
        let (attributes, node) = get_attributes(node);
        let (text_content, html_string_local) = get_text_content(html_string_local);
        match closing_tag {
            false =>
                nesting_stack.push(Node {
                    name: tag_name,
                    attributes,
                    text_content,
                    children: None,
                    self_closing,
                }),
            true => manage_child_node(&mut nesting_stack),
        }
        if self_closing {
            manage_child_node(&mut nesting_stack);
        }
        html_string = html_string_local;
    }
    nesting_stack.pop().unwrap()
}

fn manage_child_node(nesting_stack: &mut Vec<Node>) {
    if nesting_stack.len() > 1 {
        let mut nesting_stack = nesting_stack;
        let mut node = nesting_stack.pop().unwrap();
        let parent = nesting_stack.pop().unwrap();
        let parent_updated = Node {
            children: match parent.children {
                None => Some([node].into()),
                Some(mut other_children) => {
                    other_children.push(node);
                    Some(other_children)
                }
            },
            ..parent
        };
        nesting_stack.push(parent_updated);
    }
}

fn get_next_node(html_string: &str) -> Option<(&str, &str)> {
    let opening_bracket = html_string.find('<');
    let closing_bracket = html_string.find('>');
    match (opening_bracket, closing_bracket) {
        (Some(start), Some(end)) => Some((&html_string[start..end + 1], &html_string[end + 1..])),
        _ => None,
    }
}

fn get_tag_name(node: &str) -> Option<(&str, &str)> {
    let opening_bracket = node.find('<');
    let first_space = node.find(' ');
    let ending_bracket = node.find('>');
    match (opening_bracket, first_space, ending_bracket) {
        (Some(start), Some(end), _) => Some((&node[start + 1..end], &node[end + 1..])),
        (Some(start), _, Some(end)) => Some((&node[start + 1..end], &node[end + 1..])),
        _ => None,
    }
}

fn get_attributes(node: &'static str) -> (Option<Vec<Attribute>>, &str) {
    let mut attributes: Vec<Attribute> = Vec::new();
    let mut node = node.trim();
    while !node.is_empty() {
        let next_break = node.find(|char| (char == ' ' || char == '/' || char == '>'));
        //just name, no value
        if next_break.is_some() && !node[..next_break.unwrap()].contains('=') {
            attributes.push(Attribute {
                name: &node[..next_break.unwrap()],
                value: None,
            });
            node = &node[next_break.unwrap()..];
        }
        let next_eq_sign = node.find('=');
        //name and value, no quotation marks
        if
            next_break.is_some() &&
            next_eq_sign.is_some() &&
            next_eq_sign.unwrap() < next_break.unwrap() &&
            !node[..next_break.unwrap()].contains(|char| (char == '"' || char == '\''))
        {
            attributes.push(Attribute {
                name: &node[..next_eq_sign.unwrap()],
                value: Some(&node[next_eq_sign.unwrap() + 1..next_break.unwrap()]),
            });
            node = &node[next_break.unwrap()..];
        }
        //name and value or values, quotation marks

        if
            next_break.is_some() &&
            next_eq_sign.is_some() &&
            next_eq_sign.unwrap() < next_break.unwrap() &&
            node.get(..next_break.unwrap()).is_some() &&
            node[..next_break.unwrap()].contains(|char| (char == '"' || char == '\''))
        {
            let first_quote = node.find(|char| (char == '"' || char == '\''));
            let second_quote_relative = match first_quote {
                Some(first_quote) =>
                    node[first_quote + 1..].find(|char| (char == '"' || char == '\'')),
                None => None,
            };
            if let Some(index) = second_quote_relative {
                let second_quote = index + first_quote.unwrap();
                let value = Some(&node[first_quote.unwrap()..second_quote + 2]);
                let name = &node[..next_eq_sign.unwrap()];

                attributes.push(Attribute {
                    name,
                    value,
                });
                node = &node[second_quote + 2..];
            }
        }
        while node.starts_with(|char| (char == ' ' || char == '/' || char == '>')) {
            node = node.trim_start_matches(|char| (char == ' ' || char == '/' || char == '>'));
        }
    }
    if attributes.is_empty() {
        return (None, node);
    }
    (Some(attributes), node)
}

fn get_text_content(html_string: &str) -> (Option<&str>, &str) {
    if html_string.starts_with('<') || html_string.is_empty() {
        return (None, html_string);
    }
    let text_end = html_string.find('<');
    let text_content = &html_string.get(..text_end.unwrap()).unwrap();
    let html_string = &html_string[text_end.unwrap()..];
    (Some(text_content), html_string)
}

#[derive(Debug)]
struct Node {
    name: &'static str,
    attributes: Option<Vec<Attribute>>,
    text_content: Option<&'static str>,
    children: Option<Vec<Node>>,
    self_closing: bool,
}

#[derive(Debug)]
struct Attribute {
    name: &'static str,
    value: Option<&'static str>,
}
