#![allow(unused)]
use crate::structs::*;
const START_TOKEN: char = '<';
const END_TOKEN: char = '>';
const CLOSE_TOKEN: char = '/';
const ATT_VALUE_TOKEN: char = '=';

fn deserialize(html_string: &str) -> Node {
    let mut unparsed_string = html_string;
    let unparsed_string = get_next_tag(html_string);
    todo!();
}
fn get_next_tag(fragment: &str) -> Option<&str> {
    let index = todo!();
}
fn get_tag_name(fragment: &str) -> (&str, &str) {
    //
    todo!("return (tag_name, unparsed_string")
}
fn get_attributes(fragment: &str) -> usize {
    todo!()
}
fn get_tag_end(fragment: &str) -> usize {
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
