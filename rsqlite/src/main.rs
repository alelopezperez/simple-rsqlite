use lib_rsqlite::{
    btree::{InteriorNode, Node, PageNode},
    pager::Page,
};

fn main() {
    let interior_node = InteriorNode {
        num_keys: 3,
        keys: [Some(1), Some(2), Some(3)],
        page_offset: [Some(2), Some(3), Some(4), Some(5)],
    };
    let node = Node::InteriorNode(interior_node);
    let page_node = PageNode {
        page_count: 1,
        is_root: true,
        node_type: 0,
        payload_size: 10,
        node: node,
    };

    let page = Page::from(page_node);

    println!("size");
}
