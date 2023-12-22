//Problems - selfclosing tags, text content after child node, angle brackets in text
// will have to handle <input> somehow - might be the easiest to make an exception for it and handle before parsing

#![allow(dead_code, unused)]

use std::collections::binary_heap::Drain;
fn main() {
    let mut html_string: String =
        "<table attribute attribute2=1 attribute3=\"hioh\" attribute4='hioh' attribute5><tbody><tr a1=2 a2 a3><td arg><input hio=pstro barachlo /></td><td>##TEXT##</td></tr><tr><td>hiopop</td><td groho paproho=sio>heh</td></tr></tbody></table>".into();
    let a = HTMLNode::from(&mut html_string);
    dbg!(&a);
}

#[derive(Debug)]
struct HTMLNode {
    name: String,
    attributes: Option<Vec<HTMLAttribute>>,
    text_content: Option<String>,
    children: Option<Vec<HTMLNode>>,
}

#[derive(Debug)]
struct HTMLAttribute {
    name: String,
    value: Option<String>,
}

impl From<&mut String> for HTMLNode {
  fn from(html_string: &mut String) -> Self {
    let mut children : Vec<HTMLNode> = Vec::new();
    let self_closing = get_selfclosing_tag(html_string);
    if self_closing.is_some(){
      return self_closing.unwrap()
    }
    let name = get_next_tag(html_string);
    let attributes = get_attributes(html_string);

    let text_content = get_text_content(html_string);

    let closing_tag = format!("</{}>",name);
    while  (!html_string.starts_with(&closing_tag) && !html_string.is_empty()) {
      children.push(HTMLNode::from(&mut html_string.clone()));
    }
    html_string.drain(0..closing_tag.len());

    HTMLNode {
      name,
      attributes,
      text_content,
      children: match children.is_empty() {
        true=> None,
        false=> Some(children)
      }
    }
  }
}


fn get_next_tag(html_string: &mut String) -> String {
    let start_of_tag = html_string.find("<");
    let end_of_tag = html_string.find(|char| char == '>' || char == ' ');
    let indices = match (start_of_tag, end_of_tag) {
        (Some(start), Some(end)) => (start, end + 1),
        _ => todo!(),
    };
    let mut tag = html_string
        .drain(indices.0..indices.1)
        .as_str()
        .trim_start_matches('<')
        .trim_end_matches('>')
        .trim_end() //if attribute follows
        .to_string();
    tag
}

fn get_attributes(html_string: &mut String) -> Option<Vec<HTMLAttribute>> {
    let next_opener_bracket = html_string.find('<');
    let next_closing_bracket = html_string.find('>');
    if let (Some(a),Some(b)) = (next_opener_bracket, next_closing_bracket) {
      if a < b {
        return None;
      }
    }
    if html_string.starts_with('<') || html_string.is_empty() {
        return None;
    }
    let end_index = html_string.find('>').unwrap();
    let attribute: Vec<HTMLAttribute> = html_string
        .drain(0..end_index + 1)
        .as_str()
        .trim_end_matches('>')
        .split(' ')
        .map(|attribute| {
            let attribute: Vec<&str> = attribute.split('=').collect();
            match attribute.len() {
                1 =>  HTMLAttribute {
                        name: attribute[0].to_string(),
                        value: None,
                      }, 
                2 =>  HTMLAttribute {
                        name: attribute[0].to_string(),
                        value: Some(attribute[1].trim_matches(|char| char =='\"' || char == '\'').to_string()),
                      },
                _ => panic!(
                    "Parsing Error - Multiple equality signs in a single attribute are not currently handled"
                ),
            }
        })
        .collect();
    Some(attribute)
}

fn get_text_content(html_string: &mut String) -> Option<String> {
    //TODO!!!
    //What if text starts with a '<' character? Consider workaround
    if html_string.starts_with('<') || html_string.is_empty() {
      return None;
    }
    
    let end_index = html_string.find('<').unwrap();
    let text_content = html_string.drain(0..end_index).as_str().to_string();
    Some(text_content)
  }
  
  fn get_selfclosing_tag (html_string: &mut String) -> Option<HTMLNode>{
    if html_string.is_empty() {
      return None;
    }
    
    let next_closing_bracket = html_string.find('>').unwrap();
    if (!html_string[next_closing_bracket-1..=next_closing_bracket].contains("/>")){
      return None;
    }
    let name = get_next_tag(html_string);
    let slash =  html_string.find('/').unwrap();
    let mut attributes: Option<Vec<HTMLAttribute>> = None;
    if !html_string.trim().starts_with("/>") {
      html_string.drain(slash..slash+1);//remove '/'
      println!("Before {}",&html_string);
      attributes = get_attributes(html_string);
      dbg!(&attributes);
      println!("After {}",&html_string);
    }
  
  
  Some(HTMLNode {
    name,
    attributes,
    text_content: None,
    children: None
  })
}