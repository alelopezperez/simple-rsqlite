const DB_HEADER_STRING_SIZE: usize = 15;
const DB_HEADER_STRING_VALUE: &str = "SQLite format 30";
const DB_OFFSET_HEADER_STRING: usize = 0;
const DB_HEADER_PAGE_SIZE: u16 = 4096;

const DB_HEADER_SIZE: usize = DB_HEADER_STRING_SIZE + u16::BITS as usize + u32::BITS as usize;

impl DataType {
    fn to_serial_type(&self) -> u16 {
        match self {
            DataType::Integer(_) => 6,
            DataType::Real(_) => 7,
        }
    }

    fn size_in_bytes(&self) -> u16 {
        match self {
            DataType::Integer(_) => (i64::BITS / 8) as u16,
            DataType::Real(_) => 8,
        }
    }
}

pub enum DataType {
    Integer(i64),
    Real(f64),
}

struct DatabaseHeader {
    header_string: &'static str,
    page_size: u16,
    pages_ammount: u32,
}

const BTREE_HEADER_SIZE: usize = 3;
pub struct BTreeHeader {
    pub node_type: u8,
    pub cell_number: u16,
}

impl Cell {
    fn size_in_bytes(&self) -> u16 {
        match self {
            Cell::LeafCell(leaf_cell) => leaf_cell.size_in_bytes(),
        }
    }
}
pub enum Cell {
    LeafCell(LeafCell),
}

impl LeafCell {
    pub fn new(record: Record, id: u16) -> Self {
        Self {
            rowid: id,
            record_payload_bytes_size: record.size_in_bytes(),
            payload: record,
        }
    }

    fn size_in_bytes(&self) -> u16 {
        2 + 2 + self.record_payload_bytes_size
    }
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

impl Record {
    pub fn new(row_data: Vec<DataType>) -> Self {
        let header = RecordFomatHeader {
            header_size: 1 + row_data.len() as u16,
            serialtype: row_data.iter().map(|r| r.to_serial_type()).collect(),
        };
        Self {
            header,
            body: row_data,
        }
    }

    pub fn size_in_bytes(&self) -> u16 {
        self.body
            .iter()
            .map(|data| data.size_in_bytes())
            .sum::<u16>()
            + (self.header.serialtype.len() * 2) as u16
            + 2
    }
}
pub struct Record {
    pub header: RecordFomatHeader,
    pub body: Vec<DataType>,
}

impl BTree {
    fn to_big_end_byte(&self) -> Vec<u8> {
        todo!()
    }

    pub fn new(cells_arr: Vec<Cell>, is_leaf: bool) -> Self {
        let offsets = cells_arr
            .iter()
            .map(|cell| cell.size_in_bytes())
            .scan(BTREE_HEADER_SIZE as u16, |prev, cell| {
                let offset = cell + *prev;
                *prev = offset;
                Some(offset)
            })
            .collect::<Vec<_>>();
        if is_leaf {
            Self {
                header: BTreeHeader {
                    node_type: 0,
                    cell_number: cells_arr.len() as u16,
                },
                cell_pointer_offsets_arr: offsets,
                arr_cell: cells_arr,
            }
        } else {
            panic!("No");
        }
    }
}
pub struct BTree {
    pub header: BTreeHeader,
    pub cell_pointer_offsets_arr: Vec<u16>,
    pub arr_cell: Vec<Cell>,
}
