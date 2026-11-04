//! Outpost 2 Sprite and PRT (Palette & Resource Table) Metadata Parser
#![allow(dead_code)]

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// Raw 20-byte sprite header in PRT file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrtSpriteHeader {
    pub scan_line_width: u32,
    pub pixel_offset: u32,
    pub height: u32,
    pub width: u32,
    pub image_type: u16,
    pub palette_index: u16,
}

impl PrtSpriteHeader {
    pub fn is_shadow(&self) -> bool {
        (self.image_type & 4) != 0
    }

    pub fn to_frame(&self) -> PrtFrame {
        PrtFrame {
            width: self.width,
            height: self.height,
            scan_line_width: self.scan_line_width,
            pixel_offset: self.pixel_offset,
            palette_index: self.palette_index,
            is_shadow: self.is_shadow(),
            image_type: self.image_type,
        }
    }
}

/// Decoded sprite frame metadata with convenience properties
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrtFrame {
    pub width: u32,
    pub height: u32,
    pub scan_line_width: u32,
    pub pixel_offset: u32,
    pub palette_index: u16,
    pub is_shadow: bool,
    pub image_type: u16,
}

pub type SpriteMeta = PrtFrame;

#[derive(Debug, Clone, Copy)]
pub struct PrtLayer {
    pub bitmap_index: u16,
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone)]
pub struct PrtAnimation {
    pub frames: Vec<Vec<PrtLayer>>,
}

#[derive(Debug, Clone)]
pub struct PrtFile {
    pub palettes: Vec<[[u8; 3]; 256]>,
    pub sprites: Vec<SpriteMeta>,
    pub animations: Vec<PrtAnimation>,
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
                image_type: img_type,
            });
        }

        // The numbered building and vehicle IDs refer to animations. Each frame
        // contains several bitmap layers, including shadows and moving parts.
        let animation_count = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
        let _frame_count = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
        let _layer_count = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
        let _unknown_count = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
        let mut animations = Vec::with_capacity(animation_count);
        for _ in 0..animation_count {
            let mut header = [0u8; 32];
            cursor.read_exact(&mut header).map_err(|e| e.to_string())?;
            let frame_count = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
            let mut frames = Vec::with_capacity(frame_count);
            for _ in 0..frame_count {
                let layer_flags = cursor.read_u8().map_err(|e| e.to_string())?;
                let other_flags = cursor.read_u8().map_err(|e| e.to_string())?;
                if layer_flags & 0x80 != 0 {
                    let mut optional = [0u8; 2];
                    cursor.read_exact(&mut optional).map_err(|e| e.to_string())?;
                }
                if other_flags & 0x80 != 0 {
                    let mut optional = [0u8; 2];
                    cursor.read_exact(&mut optional).map_err(|e| e.to_string())?;
                }
                let mut layers = Vec::with_capacity((layer_flags & 0x7f) as usize);
                for _ in 0..layer_flags & 0x7f {
                    let bitmap_index = cursor.read_u16::<LittleEndian>().map_err(|e| e.to_string())?;
                    let mut unused = [0u8; 2];
                    cursor.read_exact(&mut unused).map_err(|e| e.to_string())?;
                    let x = cursor.read_i16::<LittleEndian>().map_err(|e| e.to_string())?;
                    let y = cursor.read_i16::<LittleEndian>().map_err(|e| e.to_string())?;
                    layers.push(PrtLayer { bitmap_index, x, y });
                }
                frames.push(layers);
            }
            let unknown_items = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
            let mut unused = vec![0u8; unknown_items * 16];
            cursor.read_exact(&mut unused).map_err(|e| e.to_string())?;
            animations.push(PrtAnimation { frames });
        }

        Ok(Self { palettes, sprites, animations })
    }

    pub fn get_sprite(&self, id: usize) -> Option<&SpriteMeta> {
        self.sprites.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::PrtFile;

    #[test]
    fn parses_original_composite_art_groups() {
        let art = PrtFile::parse(include_bytes!("../../assets/op2_art.prt")).unwrap();
        assert_eq!(art.sprites.len(), 5390);
        assert_eq!(art.animations.len(), 2079);
        assert!(art.animations[1845].frames[0].len() > 1);
        assert!(art.animations[1202].frames[0].len() > 1);
    }
}
