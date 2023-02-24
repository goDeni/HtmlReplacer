use anyhow::{anyhow, Result};

use html_editor::{
    operation::{Htmlifiable, Selector},
    parse, Element, Node,
};
use std::{
    fs::{self, File},
    io::Write,
};

use walkdir::WalkDir;

trait ReplaceQuery {
    fn query_replace(&mut self, selector: &Selector, new_node: &Node);
}

impl ReplaceQuery for Vec<Node> {
    fn query_replace(&mut self, selector: &Selector, new_node: &Node) {
        let new_vec: Vec<Node> = self
            .iter()
            .map(|node| {
                if !node.is_element() {
                    return node.clone();
                }
                let mut element = node.clone().into_element();
                if selector.matches(&element) {
                    return new_node.clone();
                }

                element.query_replace(selector, new_node);
                return Node::Element {
                    name: element.name,
                    attrs: element.attrs,
                    children: element.children,
                };
            })
            .collect();

        self.clear();
        self.extend(new_vec);
    }
}

impl ReplaceQuery for Element {
    fn query_replace(&mut self, selector: &Selector, new_node: &Node) {
        self.children.query_replace(selector, new_node)
    }
}

fn get_header_node() -> Result<Node> {
    let node = parse(fs::read_to_string("../header.html")?.as_ref())
        .unwrap()
        .get(0)
        .unwrap()
        .clone();

    if !node.is_element() {
        return Err(anyhow!("node is not element"));
    }

    let element_name = node.clone().into_element().name;
    if element_name != "head" {
        return Err(anyhow!("Element \"{}\" is not header", element_name));
    }

    return Ok(node);
}

fn main() -> Result<()> {
    let header_node = get_header_node()?;

    for entry in WalkDir::new("../input") {
        let path = entry?.path().to_owned();
        if path.is_dir() {
            println!("Entering in directory {}", path.display());
            continue;
        }

        if path.extension().unwrap() != "htm" {
            println!("File {:?} has no .htm extension", path.file_name().unwrap());
            continue;
        }

        let mut dom = parse(fs::read_to_string(path.clone())?.as_ref()).unwrap();
        dom.query_replace(&Selector::from("head"), &header_node);
        File::create("output.html")?.write_all(dom.html().as_bytes())?;

        println!("Done {:?}", path);
    }

    Ok(())
}
