use anyhow::{anyhow, Ok, Result};

use clap::Parser;
use fs_extra::dir::{copy as copy_dir, create_all, CopyOptions};
use html_editor::{
    operation::{Htmlifiable, Selector},
    parse, Element, Node,
};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
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

fn get_header_node(file: &Path) -> Result<Node> {
    let node = parse(fs::read_to_string(file)?.as_ref())
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

fn replace_elements(html: String, selector: &str, header_node: Node) -> String {
    let mut nodes = parse(&html).unwrap();

    nodes.query_replace(&Selector::from(selector), &header_node);

    nodes.html()
}

fn backup_directory(directory_to_backup: &Path) -> Result<()> {
    let directory = Path::new("./backup").join(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .to_string(),
    );

    create_all(directory.clone(), false)?;
    copy_dir(
        directory_to_backup,
        directory.clone(),
        &CopyOptions::new().content_only(true),
    )?;
    println!(
        "Backup created \"{}\"",
        directory
            .canonicalize()
            .or(Ok(directory))?
            .to_str()
            .unwrap()
    );

    Ok(())
}

#[derive(Parser)]
struct Cli {
    header_file: std::path::PathBuf,
    html_files_dir: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let header_node = get_header_node(&args.header_file)?;
    backup_directory(&args.html_files_dir)?;

    for entry in WalkDir::new(args.html_files_dir) {
        let path = entry?.path().to_owned();
        if path.is_dir() {
            println!("Entering in directory {}", path.display());
            continue;
        }

        if path.extension().unwrap() != "htm" {
            println!("File {:?} has no .htm extension", path.file_name().unwrap());
            continue;
        }

        let new_file_content = replace_elements(
            fs::read_to_string(path.clone())?,
            "head",
            header_node.clone(),
        );
        OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(path.clone())?
            .write_all(new_file_content.as_bytes())?;

        println!("Done {:?}", path);
    }

    Ok(())
}
