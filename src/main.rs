#[macro_use]
extern crate markup5ever_rcdom as rcdom;


use rcdom::{NodeData};

use html5ever::tendril::TendrilSink;
use notify;
use notify::{Event, RecursiveMode, Result, Watcher};
use rcdom::Handle;
use std::borrow::{Borrow, BorrowMut};
use std::{env, io};
use std::path::Path;
use std::sync::mpsc::channel;
use html5ever::{parse_document, ParseOpts, };
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();
    if command == "run" {
        println!("Running the server...");
        run().expect("Failed to watch the files")
    }
}

fn run() -> Result<()> {
    let (sender, receiver) = channel();
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(sender)?;

		compile();

    watcher
        .watch(Path::new("./sample"), RecursiveMode::Recursive)
        .expect("S");

    loop {
        match receiver.recv() {
            Ok(event) => handle_event(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn handle_event(event: Result<Event>) {
    match event {
        Ok(_) => {
					compile();
        }
        Err(_) => println!("Error"),
    }
}


fn walk(handle: &Handle) {
	let node = handle;

	match node.data {
		NodeData::Comment { ref contents } => {
			println!("Comment: {}", contents.to_string() );
		},
		NodeData::Document => {
			println!("Document");
		},
		NodeData::Doctype { ref name, ref public_id, ref system_id } => {
			println!("Doctype: {} {} {}", name, public_id, system_id);
		},
		NodeData::Element { ref name, ref attrs, .. } => {
			println!("Element: {} {:?}", name.local, attrs);
		},
		NodeData::ProcessingInstruction { ref target, ref contents } => {
			println!("ProcessingInstruction: {} {}", target, contents);
		},
		NodeData::Text { ref contents } => {
			println!("Text: {}", contents.borrow());
		},
		_ => {
			println!("Other");
		}
	}

	for child in node.children.borrow().iter() {
		walk(child);
	}
}


fn compile() {
	// go through files 
	// let data = "<html><head></head><body><h1>hello world</h1></body></html>";
	let data: String = fs::read_to_string("./sample/index.html").unwrap().parse().unwrap();
	println!("data: {}", data);
	let dom = parse_document(rcdom::RcDom::default(), ParseOpts::default())
		.from_utf8()
		.read_from(&mut data.as_bytes())
		.unwrap();

	walk(&dom.document);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
