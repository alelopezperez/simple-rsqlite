use std::io::Read;

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

impl DataType {
    fn to_big_endian_bytes(&self) -> Vec<u8> {
        match self {
            DataType::Integer(int) => int.to_be_bytes().into_iter().collect(),
            DataType::Real(float) => float.to_be_bytes().into_iter().collect(),
        }
    }
}
#[derive(Debug)]
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
impl From<&[u8]> for BTreeHeader {
    fn from(value: &[u8]) -> Self {
        println!("len for btree header{}", value.len());
        let mut buffer_node_type = [0_u8; 1];
        buffer_node_type.clone_from_slice(&value[0..1]);
        let node_type = u8::from_be_bytes(buffer_node_type);

        let mut buffer_cell_number = [0_u8; 2];
        buffer_cell_number.clone_from_slice(&value[1..3]);
        let cell_number = u16::from_be_bytes(buffer_cell_number);

        println!("EL HEADER {:?}", value);

        Self {
            node_type,
            cell_number,
        }
    }
}
impl BTreeHeader {
    fn to_big_endian_bytes(&self) -> Vec<u8> {
        self.node_type
            .to_be_bytes()
            .into_iter()
            .chain(self.cell_number.to_be_bytes())
            .collect()
    }
}
#[derive(Debug, Clone)]
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
impl Cell {
    fn to_big_endian_bytes(&self) -> Vec<u8> {
        match self {
            Cell::LeafCell(leaf_cell) => leaf_cell.to_big_endian_bytes(),
        }
    }
}
impl From<&[u8]> for Cell {
    fn from(value: &[u8]) -> Self {
        todo!()
    }
}
#[derive(Debug)]
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

    fn to_big_endian_bytes(&self) -> Vec<u8> {
        self.record_payload_bytes_size
            .to_be_bytes()
            .into_iter()
            .chain(self.rowid.to_be_bytes())
            .chain(self.payload.to_big_endian_bytes())
            .collect()
    }

    fn size_in_bytes(&self) -> u16 {
        2 + 2 + self.record_payload_bytes_size
    }
}
#[derive(Debug)]
pub struct LeafCell {
    pub record_payload_bytes_size: u16,
    pub rowid: u16,
    pub payload: Record,
}

impl RecordFomatHeader {
    fn to_big_endian_bytes(&self) -> Vec<u8> {
        self.serialtype
            .iter()
            .flat_map(|serial| serial.to_be_bytes())
            .chain(self.header_size.to_be_bytes())
            .collect()
    }
}

#[derive(Debug)]
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
impl Record {
    fn to_big_endian_bytes(&self) -> Vec<u8> {
        self.body
            .iter()
            .flat_map(|data| data.to_big_endian_bytes())
            .chain(self.header.to_big_endian_bytes())
            .collect()
    }
}
#[derive(Debug)]
pub struct Record {
    pub header: RecordFomatHeader,
    pub body: Vec<DataType>,
}

impl From<&[u8; 4096]> for BTree {
    fn from(value: &[u8; 4096]) -> Self {
        let header = BTreeHeader::from(&value[..BTREE_HEADER_SIZE]);

        let cell_offset_end = BTREE_HEADER_SIZE + (header.cell_number as usize * 2);
        let cell_offset_bytes = &value[BTREE_HEADER_SIZE..cell_offset_end];

        let cell_pointer_offsets_arr = (0..cell_offset_bytes.len())
            .step_by(2)
            .map(|start| {
                u16::from_be_bytes(cell_offset_bytes[start..start + 2].try_into().unwrap())
            })
            .collect::<Vec<_>>();

        todo!()
    }
}

impl BTree {
    pub fn to_big_endian_bytes(&self) -> [u8; 4096] {
        let header = self.header.to_big_endian_bytes();
        let cell_pointer_offsets_arr = self
            .cell_pointer_offsets_arr
            .iter()
            .flat_map(|offset| offset.to_be_bytes())
            .collect::<Vec<_>>();
        let arr_cell = self
            .arr_cell
            .iter()
            .flat_map(|cell| cell.to_big_endian_bytes())
            .collect::<Vec<_>>();

        let buffer_vec = header
            .into_iter()
            .chain(cell_pointer_offsets_arr)
            .chain(arr_cell)
            .collect::<Vec<_>>();

        let mut page_bytes = [0_u8; 4096];

        for (index, byte) in buffer_vec.into_iter().enumerate() {
            page_bytes[index] = byte;
        }

        page_bytes
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
            println!("el cells arr es {}", cells_arr.len());
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
#[derive(Debug)]
pub struct BTree {
    pub header: BTreeHeader,
    pub cell_pointer_offsets_arr: Vec<u16>,
    pub arr_cell: Vec<Cell>,
}
