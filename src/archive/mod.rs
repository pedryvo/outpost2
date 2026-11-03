//! Outpost 2 Volume Archive (.vol) streaming reader

use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone)]
pub struct VolEntry {
    pub name: String,
    pub offset: u32,
    pub size: u32,
    pub compression: u8,
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

        // Check VOL magic
        let mut magic = [0u8; 4];
        cursor.read_exact(&mut magic).map_err(|e| e.to_string())?;
        if &magic != b"VOL " {
            return Err("Invalid .vol magic header signature".into());
        }

        let _header_size = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;

        let mut volh_tag = [0u8; 4];
        cursor.read_exact(&mut volh_tag).map_err(|e| e.to_string())?;

        let _volh_len = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;

        // Read string table
        let mut vols_tag = [0u8; 4];
        cursor.read_exact(&mut vols_tag).map_err(|e| e.to_string())?;
        let vols_len = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;

        let mut string_table = vec![0u8; vols_len];
        cursor.read_exact(&mut string_table).map_err(|e| e.to_string())?;

        // Extract null-delimited file names
        let mut names = Vec::new();
        for chunk in string_table.split(|&b| b == 0) {
            if !chunk.is_empty() {
                if let Ok(s) = std::str::from_utf8(chunk) {
                    names.push(s.to_string());
                }
            }
        }

        let mut entries = Vec::new();
        for (i, name) in names.into_iter().enumerate() {
            entries.push(VolEntry {
                name,
                offset: 0,
                size: 0,
                compression: 0,
            });
            if i > 2048 {
                break;
            }
        }

        Ok(Self {
            entries,
            data,
        })
    }

    pub fn file_count(&self) -> usize {
        self.entries.len()
    }
}
