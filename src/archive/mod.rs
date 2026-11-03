//! Outpost 2 Volume Archive (.vol) streaming reader

pub mod prt;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    Uncompressed = 0x0100,
    Lzh = 0x0101,
}

#[derive(Debug, Clone)]
pub struct VolEntry {
    pub name: String,
    pub data_offset: u32,
    pub file_size: u32,
    pub compression: CompressionType,
}

#[derive(Debug)]
pub struct VolArchive {
    pub entries: Vec<VolEntry>,
    data: Vec<u8>,
}

impl VolArchive {
    /// Parse a .vol container from raw bytes
    pub fn parse(data: Vec<u8>) -> Result<Self, String> {
        if data.len() < 32 {
            return Err("File too small to be a valid .vol archive".into());
        }

        let mut cursor = Cursor::new(&data);

        // 1. Tag "VOL "
        let mut magic = [0u8; 4];
        cursor.read_exact(&mut magic).map_err(|e| e.to_string())?;
        if &magic != b"VOL " {
            return Err("Invalid .vol magic header signature".into());
        }
        let raw_hdr_len = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
        let _hdr_len = raw_hdr_len & 0x7FFFFFFF;

        // 2. Tag "volh"
        let mut volh_tag = [0u8; 4];
        cursor.read_exact(&mut volh_tag).map_err(|e| e.to_string())?;
        if &volh_tag != b"volh" {
            return Err("Missing volh section header".into());
        }
        let _volh_len = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? & 0x7FFFFFFF;

        // 3. Tag "vols" (string table)
        let mut vols_tag = [0u8; 4];
        cursor.read_exact(&mut vols_tag).map_err(|e| e.to_string())?;
        if &vols_tag != b"vols" {
            return Err("Missing vols section header".into());
        }
        let padded_vols_len = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? & 0x7FFFFFFF;
        let actual_str_len = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;

        let mut string_table = vec![0u8; actual_str_len];
        cursor.read_exact(&mut string_table).map_err(|e| e.to_string())?;

        // Position cursor right at start of voli section
        cursor.set_position((8 + 8 + 8 + padded_vols_len) as u64);

        // 4. Tag "voli" (index table)
        let mut voli_tag = [0u8; 4];
        cursor.read_exact(&mut voli_tag).map_err(|e| e.to_string())?;
        if &voli_tag != b"voli" {
            return Err("Missing voli section header".into());
        }
        let voli_len = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? & 0x7FFFFFFF;
        let entry_count = (voli_len / 14) as usize;

        let mut entries = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            let fn_offset = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
            let data_block_offset = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
            let file_size = cursor.read_i32::<LittleEndian>().map_err(|e| e.to_string())? as u32;
            let comp_code = cursor.read_u16::<LittleEndian>().map_err(|e| e.to_string())?;

            let name = if fn_offset < string_table.len() {
                let bytes = &string_table[fn_offset..];
                let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                String::from_utf8_lossy(&bytes[..end]).to_string()
            } else {
                "unknown".to_string()
            };

            let compression = match comp_code {
                0x0101 => CompressionType::Lzh,
                _ => CompressionType::Uncompressed,
            };

            entries.push(VolEntry {
                name,
                data_offset: data_block_offset + 8, // Skip 8-byte VBLK header
                file_size,
                compression,
            });
        }

        Ok(Self { entries, data })
    }

    /// Retrieve raw bytes of an uncompressed archived member
    pub fn get_file(&self, name: &str) -> Option<&[u8]> {
        for entry in &self.entries {
            if entry.name.eq_ignore_ascii_case(name) {
                let start = entry.data_offset as usize;
                let end = start + entry.file_size as usize;
                if end <= self.data.len() {
                    return Some(&self.data[start..end]);
                }
            }
        }
        None
    }

    pub fn file_count(&self) -> usize {
        self.entries.len()
    }
}
