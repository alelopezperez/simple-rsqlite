use std::{fs::File, io::Write, os::unix::fs::FileExt};

use lib_rsqlite::btree::{BTree, Cell, DataType, LeafCell, Record};

fn main() {
    let rowid = 5;
    let id_input = DataType::Integer(rowid);
    let num_input = DataType::Integer(10);

    let record = Record::new(vec![id_input, num_input]);

    let leaf_cell = Cell::LeafCell(LeafCell::new(record, rowid as u16));

    let btree = BTree::new(vec![leaf_cell], true);

    let buffer_bytes = btree.to_big_endian_bytes();

    let mut file = File::create("./bar.rsqlite").unwrap();

    let res = file.write_all(&buffer_bytes);
    println!("{:?}", res);

    println!("{:?}", btree);

    let read_file = File::open("./bar.rsqlite").unwrap();
    let mut read_buffer = [0; 4096];
    read_file.read_at(&mut read_buffer, 0).unwrap();

    let btree_read = BTree::from(&read_buffer);
}
