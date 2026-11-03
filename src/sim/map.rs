//! Outpost 2 Map (.map) parser and tile grid representation

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapTile {
    pub cell_type: u8,
    pub tile_mapping: u16,
    pub unit_index: u16,
    pub is_lava: bool,
    pub is_lava_possible: bool,
    pub is_blight: bool,
    pub is_wall_or_building: bool,
}

#[derive(Debug, Clone)]
pub struct GameMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<MapTile>,
}

impl GameMap {
    /// Parse raw bytes of an Outpost 2 .map file
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 20 {
            return Err("Map data too short".into());
        }

        let mut cursor = Cursor::new(data);
        let version_tag = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
        if version_tag < 0x1010 {
            return Err(format!("Unsupported map version: 0x{:x}", version_tag));
        }

        let _b_saved = cursor.read_i32::<LittleEndian>().map_err(|e| e.to_string())?;
        let lg_width = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
        let height = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
        let _tilesets = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;

        let width = 1 << lg_width;
        let total_tiles = (width * height) as usize;

        if data.len() < 20 + total_tiles * 4 {
            return Err("Unexpected end of map data before tile grid".into());
        }

        let mut tiles = Vec::with_capacity(total_tiles);
        for _ in 0..total_tiles {
            let raw = cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
            tiles.push(MapTile {
                cell_type: (raw & 0x1F) as u8,
                tile_mapping: ((raw >> 5) & 0x7FF) as u16,
                unit_index: ((raw >> 16) & 0x7FF) as u16,
                is_lava: ((raw >> 27) & 1) != 0,
                is_lava_possible: ((raw >> 28) & 1) != 0,
                is_blight: ((raw >> 30) & 1) != 0,
                is_wall_or_building: ((raw >> 31) & 1) != 0,
            });
        }

        Ok(Self {
            width,
            height,
            tiles,
        })
    }

    /// Retrieve tile at coordinates (x, y)
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&MapTile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[(y * self.width + x) as usize])
        } else {
            None
        }
    }
}
