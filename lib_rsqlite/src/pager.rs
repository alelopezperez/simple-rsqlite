use crate::btree::PageNode;

const PAGE_SIZE: usize = 4096;

pub struct Page {
    pub data: [u8; PAGE_SIZE],
}
impl From<PageNode> for Page {
    fn from(value: PageNode) -> Self {
        Self {
            data: value.to_bytes().try_into().unwrap(),
        }
    }
}
