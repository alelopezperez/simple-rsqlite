const PAGE_SIZE: usize = 4096;

pub struct Page {
    pub data: [u8; PAGE_SIZE],
}
