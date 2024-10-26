const PAGE_SIZE: usize = 4096;

struct Page {
    offset: usize,
    data: [u8; PAGE_SIZE],
}
