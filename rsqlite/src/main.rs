use lib_rsqlite::btree::{self, DataTypes};
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let data_type = DataTypes::Text("hola".to_string());

    let data = btree::Data {
        tipo: data_type.type_id(),
        payload: data_type,
        len: 4,
    };

    let leaves = btree::LeafNode {
        key: 3,
        data: vec![data],
    };

    println!("{:?}", leaves.to_bytes());

    let page = leaves.to_bytes();
    let mut file = File::create("foo.rsqlite").unwrap();
    file.write_all(&page).unwrap();

    println!("Hello, world!");

    let mut buf = Vec::new();

    let mut read = File::open("foo.rsqlite").unwrap();
    read.read_to_end(&mut buf).unwrap();

    println!("{:?}", buf);

    let key_buf: [u8; 8] = buf[..8].try_into().unwrap();
    let key = u64::from_be_bytes(key_buf);
    println!("{}", key);

    let tipo_buf: [u8; 1] = buf[8..9].try_into().unwrap();
    let tipo = u8::from_be_bytes(tipo_buf);
    println!("{}", tipo);

    let size_buf: [u8; 8] = buf[9..17].try_into().unwrap();
    let size = usize::from_be_bytes(size_buf);
    println!("{}", size);

    let payload_buf = buf[17..17 + size].to_vec();
    let payload = String::from_utf8(payload_buf).unwrap();
    println!("{}", payload);
}
