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

const DATA_HEADER_SIZE: usize = (u8::BITS / 8) as usize + (usize::BITS / 8) as usize;

pub struct HeaderData {
    pub tipo: u8,
    pub len: usize,
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
        page.append(&mut self.len.to_be_bytes().to_vec());
        page.append(&mut self.payload.to_bytes());

        page
    }
}

enum Btree {
    InteriorNode(InteriorNode),
    LeafNode(LeafNode),
}
struct HeaderNode {
    num_rows: usize,
    rows_name: Vec<String>,
}
struct InteriorNode {
    is_head: bool,
    keys: [Option<u64>; 3],
    ptr_as_offset: [Option<usize>; 3],
}

pub struct LeafNode {
    pub key: u64,
    pub data: Vec<Data>,
}

impl LeafNode {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut page = Vec::new();

        page.append(&mut self.key.to_be_bytes().to_vec());

        self.data
            .iter()
            .for_each(|node| page.append(&mut node.to_bytes()));

        page
    }
}
