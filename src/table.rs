use crate::row::{COLUMN_EMAIL_SIZE, COLUMN_USERNAME_SIZE, Row};
use std::mem;

pub const PAGE_SIZE: usize = 4096;
pub const TABLE_MAX_PAGES: usize = 100;

pub const ID_SIZE: usize = mem::size_of::<u32>();
pub const USERNAME_SIZE: usize = COLUMN_USERNAME_SIZE;
pub const EMAIL_SIZE: usize = COLUMN_EMAIL_SIZE;

pub const ID_OFFSET: usize = 0;
pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

pub const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    pub num_rows: usize,
    pub pages: [Option<Box<[u8; PAGE_SIZE]>>; TABLE_MAX_PAGES],
}

impl Table {
    pub fn new() -> Self {
        Self {
            num_rows: 0,
            pages: [(); TABLE_MAX_PAGES].map(|_| None),
        }
    }

    /// Returns a mutable pointer to where the row should go
    pub fn row_slot(&mut self, row_num: usize) -> &mut [u8] {
        let page_num = row_num / ROWS_PER_PAGE;
        let page_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = page_offset * ROW_SIZE;

        if self.pages[page_num].is_none() {
            self.pages[page_num] = Some(Box::new([0; PAGE_SIZE]));
        }

        let page = self.pages[page_num].as_mut().unwrap();
        &mut page[byte_offset..byte_offset + ROW_SIZE]
    }
}

/// Serialization
pub fn serialize_row(source: &Row, destination: &mut [u8]) {
    destination[ID_OFFSET..ID_OFFSET + ID_OFFSET].copy_from_slice(&source.id.to_ne_bytes());

    destination[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE].copy_from_slice(&source.username);

    destination[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE].copy_from_slice(&source.email);
}

pub fn deserialize_row(source: &[u8], destination: &mut Row) {
    let mut id_bytes = [0u8; 4];
    id_bytes.copy_from_slice(&source[ID_OFFSET..ID_OFFSET + ID_SIZE]);
    destination.id = u32::from_ne_bytes(id_bytes);

    destination
        .username
        .copy_from_slice(&source[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE]);

    destination
        .email
        .copy_from_slice(&source[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE]);
}
