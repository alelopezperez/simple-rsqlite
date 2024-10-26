use std::fs::File;
use std::io::{Read, Write};

use lib_rsqlite::btree::{Node, PageNode};

fn main() {
    let page = PageNode {
        table_header_info: None,
        is_root: true,
        node_type: todo!(),
        payload_size: todo!(),
        node: todo!(),
    };
}
