//! Outpost 2 Sprite and PRT (Palette & Resource Table) Metadata Parser

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

#[derive(Debug, Clone)]
pub struct SpriteMeta {
    pub width: u32,
    pub height: u32,
    pub scan_line_width: u32,
    pub pixel_offset: u32,
    pub palette_index: u16,
    pub is_shadow: bool,
}

#[derive(Debug, Clone)]
pub struct PrtFile {
    pub palettes: Vec<[[u8; 3]; 256]>,
    pub sprites: Vec<SpriteMeta>,
}

impl PrtFile {
    /// Parse raw bytes of an op2_art.prt file
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 36 {
            return Err("PRT file data too short".into());
        }

        let mut cursor = Cursor::new(data);

        // 1. CPAL section
        let mut cpal_tag = [0u8; 4];
        cursor.read_exact(&mut cpal_tag).map_err(|e| e.to_string())?;
        if &cpal_tag != b"CPAL" {
            return Err("Missing CPAL tag in PRT".into());
        }

        let palette_count = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
        let mut palettes = Vec::with_capacity(palette_count);

        for _ in 0..palette_count {
            // PaletteHeader is 28 bytes
            let mut pal_hdr = [0u8; 28];
            cursor.read_exact(&mut pal_hdr).map_err(|e| e.to_string())?;

            // 256 colors * 4 bytes (B, G, R, 0)
            let mut pal = [[0u8; 3]; 256];
            for i in 0..256 {
                let b = cursor.read_u8().map_err(|e| e.to_string())?;
                let g = cursor.read_u8().map_err(|e| e.to_string())?;
                let r = cursor.read_u8().map_err(|e| e.to_string())?;
                let _pad = cursor.read_u8().map_err(|e| e.to_string())?;
                pal[i] = [r, g, b];
            }
            palettes.push(pal);
        }

        // 2. ImageMetadata array
        let sprite_count = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
        let mut sprites = Vec::with_capacity(sprite_count);

        for _ in 0..sprite_count {
            let scan_w = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
            let p_off = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
            let h = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
            let w = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
            let img_type = cursor.read_u16::<LittleEndian>().map_err(|e| e.to_string())?;
            let pal_idx = cursor.read_u16::<LittleEndian>().map_err(|e| e.to_string())?;

            let is_shadow = (img_type & 4) != 0;

            sprites.push(SpriteMeta {
                width: w,
                height: h,
                scan_line_width: scan_w,
                pixel_offset: p_off,
                palette_index: pal_idx,
                is_shadow,
            });
        }

        Ok(Self { palettes, sprites })
    }

    pub fn get_sprite(&self, id: usize) -> Option<&SpriteMeta> {
        self.sprites.get(id)
    }
}
