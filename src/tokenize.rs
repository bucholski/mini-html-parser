#![allow(unused)]
use crate::structs::*;
const START_TOKEN: char = '<';
const END_TOKEN: char = '>';
const CLOSE_TOKEN: char = '/';
const ATT_VALUE_TOKEN: char = '=';
const SPACE: char = ' ';
const VOID_TAGS: [&str; 13] = [
    "area",
    "base",
    "br",
    "col",
    "embed",
    "hr",
    "img",
    "input",
    "link",
    "meta",
    "source",
    "track",
    "wbr",
];

fn deserialize(html_string: &str) -> Node {
    loop {
        let next_tag = get_next_tag(html_string);
        if let None = next_tag {
            break todo!();
        }
        let residual = next_tag.unwrap();
        let (tag_name, residual) = get_tag_name(residual);
        todo!();
    }
}
fn get_next_tag(residual: &str) -> Option<&str> {
    residual.strip_prefix(START_TOKEN)
}

fn get_tag_name(residual: &str) -> (&str, &str) {
    let trimmed = residual.trim_start();
    let tag_name = &residual[0..residual.find(|c| (c == SPACE || c == CLOSE_TOKEN)).unwrap()];
    todo!("return (tag_name, unparsed_string")
}
fn is_self_closing(tag_name: &str) -> bool {
    VOID_TAGS.contains(&tag_name)
}
fn get_attributes(residual: &str) -> (Option<Vec<Attribute>>, &str) {
    todo!()
}
fn get_tag_end(residual: &str) -> usize {
    todo!()
}

fn get_text_content(residual: &str) -> usize {
    todo!()
}
//find start with < and strip
//get name and strip
//loop
//match
//  /selfclosing tag
// text -> attribute -> check for =  -> strip it -> get " or ' -> remember which and strip it -> get the second one
// > -> end of tag -> break the loop

#[cfg(test)]
mod test {
    use super::*;
    const TEST_HTML: &str = "<div></div>";
    #[test]
    fn tokenize() {}
}
