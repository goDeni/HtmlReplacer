use anyhow::{anyhow, Error, Result};

use html_editor::{
    operation::{Editable, Htmlifiable, Selector},
    parse, Element, Node,
};
use std::{
    fs::{self, File},
    io::Write,
};

use walkdir::WalkDir;

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

fn find_head_vec(nodes: & Vec<Node>) {
    let mut elements = Vec::<Element>::from_iter(nodes.iter().filter_map(|node| {
        if node.is_element() {
            return Some(node.into_element());
        }
        return None;
    }));

    while elements.len() > 0 {
        // let element = elements.pop();
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");

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

        // let res = dom.insert_to(&Selector::from("head"), header_node.clone()).html();

        File::create("output.html")?.write_all(dom.html().as_bytes())?;
        break;
    }

    Ok(())
}
