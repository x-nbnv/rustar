#![no_std]
#![allow(dead_code)]

use core::fmt::Result;

extern crate alloc;

pub fn oct2bin(s: &[u8]) -> usize {
    let mut n = 0;
    for &c in s {
        if (b'0'..=b'7').contains(&c) {
            n = n * 8 + (c - b'0') as usize;
        }
    }
    n
}


pub struct TarHeader<'a> {
    pub name: &'a str,   // cleaned &str, no NUL padding
    pub size: usize,
    pub file_start: usize,
    pub file_end: usize,
}


fn parse_header<'a>(archive: &'a [u8], offset: usize) -> Option<TarHeader<'a>> {
    if offset + 512 > archive.len() {
        return None;
    }

    let header = &archive[offset..offset + 512];
    if &header[257..257 + 5] != b"ustar" {
        return None;
    }

    let size = oct2bin(&header[0x7c..0x7c + 11]);

    // filename is first 100 bytes, trim at NUL
    let raw_name = &header[..100];
    let nul_pos = raw_name.iter().position(|&b| b == 0).unwrap_or(raw_name.len());
    let name = core::str::from_utf8(&raw_name[..nul_pos]).ok()?; // fail if not UTF-8

    Some(TarHeader {
        name,
        size,
        file_start: offset + 512,
        file_end: offset + 512 + size,
    })
}


pub struct TarIter<'a> {
    archive: &'a [u8],
    offset: usize,
}

impl<'a> TarIter<'a> {
    pub fn new(archive: &'a [u8]) -> Self {
        Self { archive, offset: 0 }
    }
}

impl<'a> Iterator for TarIter<'a> {
    type Item = TarHeader<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let header = parse_header(self.archive, self.offset)?;
        // move to next header
        let size = header.size;
        let blocks = (size + 511) / 512;
        self.offset += 512 + blocks * 512;
        Some(header)
    }
}
    
pub fn tar_lookup<'a>(archive: &'a [u8], filename: &str) -> Option<&'a [u8]> {
    for header in TarIter::new(archive) {
        if header.name == filename {
            if header.file_end <= archive.len() {
                return Some(&archive[header.file_start..header.file_end]);
            }
        }
    }
    None
}


pub fn extract_file(archive: &[u8], filename: &str) -> Option<alloc::vec::Vec<u8>> {
    tar_lookup(archive, filename).map(|data| data.to_vec())
}

// you would use
//for header in TarIter::new(archive) {
    // println!(header.name.split(|&b| b == 0).next().unwrap_or(b"")); or something like that
//}

pub trait TarReader {
    fn read_block(&mut self, lba: u64, buf: &mut [u8]) -> Result<>;
}