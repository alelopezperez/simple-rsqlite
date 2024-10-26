use std::{io::Read, usize};

pub enum DataTypes {
    Text(String),
    Integer(i64),
    Real(f64),
    Blob(Vec<u8>),
    Null,
}

impl DataTypes {
    pub fn type_id(&self) -> u8 {
        match self {
            DataTypes::Text(_) => 0,
            DataTypes::Integer(_) => 1,
            DataTypes::Real(_) => 2,
            DataTypes::Blob(_) => 3,
            DataTypes::Null => 4,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            DataTypes::Text(text) => text.as_bytes().to_owned(),
            DataTypes::Integer(num) => num.to_be_bytes().to_vec(),
            DataTypes::Real(float) => float.to_be_bytes().to_vec(),
            DataTypes::Blob(blob) => blob.clone(),
            DataTypes::Null => vec![],
        }
    }
}

pub struct Data {
    pub tipo: u8,
    pub len: usize,
    pub payload: DataTypes,
}

impl Data {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut page = Vec::new();

        page.push(self.tipo);
        page.extend(self.len.to_be_bytes());
        page.extend(self.payload.to_bytes());

        page
    }
}

const PAGE_NODE_HEADER_SIZE: u32 = usize::BITS / 8 + u8::BITS / 8 + u8::BITS / 8 + usize::BITS / 8;
impl PageNode {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.page_count
            .to_be_bytes()
            .into_iter()
            .chain(self.payload_size.to_be_bytes())
            .chain(self.node.to_bytes())
            .chain(std::iter::once(self.is_root as u8))
            .collect()
    }
}
pub struct PageNode {
    pub page_count: usize,
    pub is_root: bool,
    pub node_type: u8,
    pub payload_size: usize,
    pub node: Node,
}

impl Node {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Node::InteriorNode(node) => node.to_bytes(),
            Node::LeafNode(node) => node.to_bytes(),
        }
    }
}

pub enum Node {
    InteriorNode(InteriorNode),
    LeafNode(LeafNode),
}

impl InteriorNode {
    fn to_bytes(&self) -> Vec<u8> {
        let keys_bytes = self.keys.iter().flat_map(|k| match k {
            Some(key) => key.to_be_bytes(),
            None => 0_u64.to_be_bytes(),
        });
        let page_offset_bytes = self.page_offset.iter().flat_map(|p| match p {
            Some(offset) => offset.to_be_bytes(),
            None => 0_usize.to_be_bytes(),
        });
        self.num_keys
            .to_be_bytes()
            .into_iter()
            .chain(keys_bytes)
            .chain(page_offset_bytes)
            .collect()
    }
}

pub struct InteriorNode {
    num_keys: usize,
    keys: [Option<u64>; 3],
    page_offset: [Option<usize>; 4],
}

pub struct KeyValuePair {
    pub key: u64,
    pub row_len: usize,
    pub row: Vec<Data>,
}

impl KeyValuePair {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes
    }
}

impl LeafNode {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for kv in self.kv_pairs.iter() {
            bytes.extend(kv.to_bytes())
        }

        bytes
    }
}
pub struct LeafNode {
    pub kv_pairs: Vec<KeyValuePair>,
}
