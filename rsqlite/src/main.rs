use lib_rsqlite::btree::{BTree, Cell, DataType, LeafCell, Record};

fn main() {
    let rowid = 5;
    let id_input = DataType::Integer(rowid);
    let num_input = DataType::Integer(10);

    let record = Record::new(vec![id_input, num_input]);

    let leaf_cell = Cell::LeafCell(LeafCell::new(record, rowid as u16));

    let btree = BTree::new(vec![leaf_cell], true);
}
