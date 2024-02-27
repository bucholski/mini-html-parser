#![allow(unused)]
use std::fmt::Error;

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

pub fn deserialize_dom(html_string: &str) -> Node {
    let mut residual = html_string;
    if check_next_tag(&mut residual).is_none() {
        todo!("Return the tree of nodes");
    }
    let tag_name = get_tag_name(&mut residual);
    let self_closing = is_void(&tag_name);
    let attributes = get_attributes_array(&mut residual);
    close_tag(&mut residual);

    todo!();
}

fn check_next_tag(residual: &mut &str) -> Option<()> {
    *residual = residual.strip_prefix(START_TOKEN).unwrap();
    Some(())
}

fn get_tag_name<'a>(residual: &mut &'a str) -> &'a str {
    *residual = residual.trim_start();
    let tag_name = &residual[0..residual.find(|c| (c == SPACE || c == CLOSE_TOKEN)).unwrap()];
    *residual = residual.strip_prefix(tag_name).unwrap();
    tag_name
}
fn is_void(tag_name: &str) -> bool {
    VOID_TAGS.contains(&tag_name)
}
fn get_attributes_array<'a>(residual: &mut &str) -> Option<Vec<Attribute>> {
    let mut attributes_arr: Vec<Attribute> = vec![];
    loop {
        let next_atribute = get_next_attribute(residual);
        if next_atribute.is_none() {
            break;
        }
        // attributes_arr.push(next_atribute);
    }
    match attributes_arr.len() {
        0 => None,
        _ => Some(attributes_arr),
    }
}

fn get_next_attribute(residual: &mut &str) -> Option<Attribute> {
    todo!()
}
fn close_tag(residual: &mut &str) {
    let prefix =
        &residual[..residual.find(CLOSE_TOKEN).expect("All tags need to close eventually")];
    *residual = residual.strip_prefix(prefix).expect("All tags need to close eventually");
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
