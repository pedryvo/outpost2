"""Extract browser art from the user's original Outpost 2 installation.

Run from the game directory. The source files are not fetched from the network.
"""

from io import BytesIO
from pathlib import Path
import struct

from PIL import Image


ROOT = Path(__file__).resolve().parent.parent
ASSETS = ROOT / "assets"
volume = (ROOT / "art.vol").read_bytes()
map_data = (ASSETS / "eden01.map").read_bytes()


def volume_files(data):
    strings_length = struct.unpack_from("<I", data, 24)[0]
    strings = data[28 : 28 + strings_length]
    index_start = 24 + (struct.unpack_from("<I", data, 20)[0] & 0x7FFFFFFF)
    index_length = struct.unpack_from("<I", data, index_start + 4)[0] & 0x7FFFFFFF
    files = {}
    for i in range(index_length // 14):
        name_offset, block_offset, size, compression = struct.unpack_from(
            "<IIIH", data, index_start + 8 + i * 14
        )
        if compression != 0x100:
            raise ValueError("Compressed art files require a decoder")
        name = strings[name_offset : strings.index(0, name_offset)].decode("ascii")
        files[name] = data[block_offset + 8 : block_offset + 8 + size]
    return files


files = volume_files(volume)
(ASSETS / "op2_art.prt").write_bytes(files["op2_art.prt"])
(ASSETS / "OP2_ART.BMP").write_bytes((ROOT / "OP2_ART.BMP").read_bytes())

marker = map_data.index(b"TILE SET\x1a")
mapping_count = struct.unpack_from("<I", map_data, marker + 10)[0]
mapping_start = marker + 14
atlas = Image.new("RGB", (64 * 32, ((mapping_count + 63) // 64) * 32))
wells = {
    i: Image.open(BytesIO(files[f"well{i:04d}.bmp"])).convert("RGB")
    for i in range(13)
}
for index in range(mapping_count):
    well, tile = struct.unpack_from("<HH", map_data, mapping_start + index * 8)
    image = wells[well]
    if (tile + 1) * 32 > image.height:
        raise ValueError(f"Invalid mapping {index}: well {well}, tile {tile}")
    atlas.paste(image.crop((0, tile * 32, 32, (tile + 1) * 32)),
                ((index % 64) * 32, (index // 64) * 32))
atlas.save(ASSETS / "terrain.png", optimize=True)
print(f"Extracted {mapping_count} original terrain tiles and sprite data")
