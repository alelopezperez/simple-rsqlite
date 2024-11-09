use lib_rsqlite::btree::{BTree, BTreeHeader, Cell, DataType, LeafCell, Record, RecordFomatHeader};

fn main() {
    let id_input = DataType::Integer(5);
    let num_input = DataType::Integer(10);

    let body_record = vec![id_input, num_input];
    let record_header = RecordFomatHeader {
        header_size: 3,
        serialtype: vec![0, 0],
    };
    let record = Record {
        header: record_header,
        body: body_record,
    };

    let leaf_cell = Cell::LeafCell(LeafCell {
        rowid: 5,
        record_payload_bytes_size: 3,
        payload: record,
    });

    let btree_header = BTreeHeader {
        node_type: 1,
        cell_number: 1,
    };

    let btree = BTree {
        header: btree_header,
        cell_pointer_offsets_arr: vec![1],
        arr_cell: vec![leaf_cell],
    };
}
