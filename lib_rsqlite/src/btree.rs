const DB_HEADER_STRING_SIZE: usize = 15;
const DB_HEADER_STRING_VALUE: &str = "SQLite format 30";
const DB_OFFSET_HEADER_STRING: usize = 0;
const DB_HEADER_PAGE_SIZE: u16 = 4096;

const DB_HEADER_SIZE: usize = DB_HEADER_STRING_SIZE + u16::BITS as usize + u32::BITS as usize;

pub enum DataType {
    Integer(i64),
    Real(f64),
}

struct DatabaseHeader {
    header_string: &'static str,
    page_size: u16,
    pages_ammount: u32,
}
pub struct BTreeHeader {
    pub node_type: u8,
    pub cell_number: u16,
}

pub enum Cell {
    LeafCell(LeafCell),
}

pub struct LeafCell {
    pub record_payload_bytes_size: u16,
    pub rowid: u16,
    pub payload: Record,
}

pub struct RecordFomatHeader {
    pub header_size: u16,
    pub serialtype: Vec<u16>,
}

pub struct Record {
    pub header: RecordFomatHeader,
    pub body: Vec<DataType>,
}

impl BTree {
    fn to_big_end_byte(&self) -> Vec<u8> {
        todo!()
    }
}
pub struct BTree {
    pub header: BTreeHeader,
    pub cell_pointer_offsets_arr: Vec<u16>,
    pub arr_cell: Vec<Cell>,
}
