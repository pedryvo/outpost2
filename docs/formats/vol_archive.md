# Outpost 2 Volume Archive (`.vol`) Specification

The `.vol` file format is the primary packaging container used by *Outpost 2: Divided Destiny* to bundle art, sound, maps, and technology data.

## Header Structure

Every `.vol` file starts with a fixed signature:

| Offset | Size (Bytes) | Field | Description |
|---|---|---|---|
| `0x00` | 4 | `magic` | ASCII `"VOL "` (`0x56 0x4F 0x4C 0x20`) |
| `0x04` | 4 | `header_size` | Size of the volume header metadata block |
| `0x08` | 4 | `tag_volh` | Block tag `"volh"` (`0x76 0x6F 0x6C 0x68`) |
| `0x0C` | 4 | `volh_size` | Length of the volume header structure |
| `0x10` | 4 | `tag_vols` | String table tag `"vols"` (`0x76 0x6F 0x6C 0x73`) |
| `0x14` | 4 | `vols_size` | Byte length of the null-delimited filename table |

## Filename String Table (`vols`)
The `vols` section contains null-terminated (`\0`) ASCII filenames packed sequentially.

## Index Table (`voli` / `volf`)
Following the string block, volume files index each embedded entry with:
- Filename offset into the string table
- File data byte offset in the archive
- Uncompressed size
- Compressed size
- Compression method (0 = uncompressed, 1 = RLE, 2 = LZ, 3 = LZH)

## Modern Usage
In Outpost 2 v1.3.x+, community patches predominantly store uncompressed files inside `.vol` archives, simplifying high-speed streaming directly into WebAssembly memory buffers.
