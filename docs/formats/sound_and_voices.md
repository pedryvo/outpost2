# Outpost 2 Audio Formats & Sound Subsystem Specification

This document details the audio waveform architecture, volume archive container layouts (`sound.vol` and `voices.vol`), PCM encoding parameters, and the WebAssembly Web Audio API playback engine implemented for the Outpost 2 Divided Destiny modern web port.

---

## 1. Archive Container Architecture (`sound.vol` & `voices.vol`)

Outpost 2 packages all sound effects and computerized tactical voice dispatches into two canonical `.vol` archive containers:

| Volume Archive | File Size | Entries | Category | Primary Content |
|---|---|---|---|---|
| `sound.vol` | 2,867,324 B (2.8 MB) | 100 entries (97 waveforms, 3 padding nulls) | Sound Effects (SFX) | Weapon discharges, vehicle engine acknowledgements, explosions, building deployment chimes, environmental disasters, UI beeps |
| `voices.vol` | 5,374,296 B (5.3 MB) | 150 entries (135 waveforms, 15 padding nulls) | Tactical Voice Dispatches | Bio-electronic Savant computer speech notifications, combat warnings, status reports, planetary evacuation alerts |

### Binary Section Layout

Both volume files adhere strictly to Sierra/Dynamix's 32-bit chunk container standard:

```
+-------------------------------------------------------------+
| Tag: "VOL " (4 B) | Total Header Length: u32 (4 B)          |
+-------------------------------------------------------------+
| Tag: "volh" (4 B) | Header Body Size: u32 (4 B)             |
+-------------------------------------------------------------+
| Tag: "vols" (4 B) | String Section Size: u32 (4 B)          |
| String Table Size: u32 (4 B) | Null-delimited ASCII strings |
+-------------------------------------------------------------+
| Tag: "voli" (4 B) | Index Table Size: u32 (4 B)             |
| Entry records: [Filename Offset: u32, Data Offset: u32,     |
|                 File Size: i32, Compression Code: u16]      |
+-------------------------------------------------------------+
| Member Payload Data Blocks ("VBLK" header + RIFF/WAVE bytes)|
+-------------------------------------------------------------+
```

1. **Volume Header (`VOL ` & `volh`)**: Identifies container integrity and sets overall file boundaries.
2. **String Table (`vols`)**: Houses null-terminated filenames (e.g., `savant4.wav\0`, `truk_sel.wav\0`).
3. **Index Table (`voli`)**: 14 bytes per file entry:
   - `fn_offset` (`u32`): Offset into the `vols` string table.
   - `data_block_offset` (`u32`): Absolute file byte offset to the `VBLK` payload block. (Audio data starts at `data_block_offset + 8` to skip the 8-byte `VBLK` tag and chunk size).
   - `file_size` (`u32`): Size of the encapsulated `.wav` file in bytes.
   - `compression_code` (`u16`): `0x0100` (`256`) indicates uncompressed raw waveform data. All active audio files in Outpost 2 v1.3.x are uncompressed `0x0100`.

---

## 2. PCM Waveform Encoding & Specifications

Every waveform encapsulated inside `sound.vol` and `voices.vol` shares a single unified PCM encoding standard:

```
                +---------------------------------+
                |   RIFF Chunk (4 B: "RIFF")      |
                |   Total File Size - 8 (u32)     |
                |   Format: "WAVE" (4 B)          |
                +---------------------------------+
                |   Format Chunk (4 B: "fmt ")    |
                |   Chunk Size: 16 B (u32)        |
                |   Audio Format: 1 (PCM) (u16)   |
                |   Channels: 1 (Mono) (u16)      |
                |   Sample Rate: 22,050 Hz (u32)  |
                |   Byte Rate: 22,050 B/s (u32)   |
                |   Block Align: 1 B (u16)        |
                |   Bits Per Sample: 8-bit (u16)  |
                +---------------------------------+
                |   Data Chunk (4 B: "data")      |
                |   PCM Data Byte Length (u32)    |
                |   8-bit Unsigned PCM Bytes      |
                +---------------------------------+
```

### Digital Audio Characteristics

- **Sample Rate**: `22,050 Hz` (Nyquist frequency 11,025 Hz, ideal for low-memory 1990s real-time streaming).
- **Channels**: `1` (Mono channel).
- **Format**: `PCM (0x0001)`.
- **Bit Depth**: `8-bit unsigned integer`.
  - Mid-point (silence / zero crossing): `128` (`0x80`).
  - Positive peak: `255` (`0xFF` -> `+0.992` normalized float).
  - Negative peak: `0` (`0x00` -> `-1.000` normalized float).
- **Normalized Float Conversion**:
  $$\text{float\_sample} = \frac{\text{raw\_byte} - 128.0}{128.0}$$

---

## 3. Legacy Engine SoundID & VoiceID Mapping Table

In `Outpost2.exe`, the sound subsystem uses a 252-slot pointer table located in the `.data` section from `VA 0x4e0170` to `0x4e0560`. Game logic triggers audio by index (e.g., `AddGameSound(sound_id, player_mask)`):

### Sound Table Partitioning

| Slot Range | Category | Source Archive | Representative Examples |
|---|---|---|---|
| `0..=2` | Building Demolition Failures | `sound.vol` | `bld_fal3.wav`, `bld_fal1.wav`, `bld_fal2.wav` |
| `3..=51` | Structure & Unit Acknowledgements | `sound.vol` | `bld_make.wav` (ID 3), `ag_sel.wav` (ID 4), `consvhc.wav` (ID 42), `dozr_sel.wav` (ID 43), `combmed.wav` (ID 55) |
| `52..=56` | Recon & Logistics Acknowledgements | `sound.vol` | `scout.wav` (ID 52), `truk_sel.wav` (ID 56) |
| `57..=60` | Major Milestones & Explosions | `sound.vol` | `bld_exp.wav` (ID 57), `uni_grad.wav` (ID 58 - Research fanfare), `tokaexp.wav` (ID 59) |
| `72..=79` | Kinetic & Thermal Detonations | `sound.vol` | `bigexp1.wav`..`bigexp3.wav`, `medexp1.wav`, `smalxp1.wav` |
| `80..=84` | Natural Hazards & Disasters | `sound.vol` | `quake.wav` (ID 80), `tornado2.wav` (ID 81), `thunder.wav` (ID 82), `microbe.wav` (ID 83) |
| `85..=93` | Tactical UI Confirmation Beeps | `sound.vol` | `beep2.wav` (ID 85), `bld_not.wav` (ID 88 - placement error), `beep8.wav` (ID 91), `message2.wav` (ID 93) |
| `94..=227` | Savant Computer Voice Dispatches | `voices.vol` | `savant2.wav` (ID 95 - "Research completed"), `savant3.wav` (ID 96 - "Structure destroyed"), `savant4.wav` (ID 97 - "We are under attack!"), `savant67.wav` (ID 137 - "Warning: Volcanic eruption expected"), `savnt206.wav` (ID 104/164 - "Vehicle destroyed") |
| `228..=251` | Weapon Projectiles & Beam Impacts | `sound.vol` | `laser.wav` (ID 232), `microwve.wav` (ID 234), `railfir.wav` (ID 236), `railtar.wav` (ID 237), `gun1.wav` (ID 250) |

---

## 4. WebAssembly Audio Playback Pipeline

The Rust/WASM audio architecture is implemented in [`src/audio/bank.rs`](file:///home/pedryvo/Downloads/Outpost_2_Divided_Destiny_v1.3.0.8_Win_Preinstalled_EN/Game%20Files/src/audio/bank.rs) and exposed via [`src/audio/mod.rs`](file:///home/pedryvo/Downloads/Outpost_2_Divided_Destiny_v1.3.0.8_Win_Preinstalled_EN/Game%20Files/src/audio/mod.rs):

```mermaid
flowchart TD
    subgraph Storage["Asset Storage"]
        EMBED["Embedded Key WAVs (include_bytes!)"]
        VOL["VolArchive (sound.vol / voices.vol)"]
    end

    subgraph Parser["Decoding Engine (parse_wav)"]
        WAV[RIFF/WAVE Chunk Reader]
        PCM[8-bit Unsigned to 32-bit Float Normalizer]
    end

    subgraph Bank["SoundBank Subsystem (src/audio/bank.rs)"]
        CACHE[RefCell<HashMap<String, SoundSample>>]
        POOL[Channel Voice Pooler & Debounce Filter]
        MAP[LEGACY_SOUND_TABLE ID Resolver]
    end

    subgraph WebAudio["Web Audio API Graph"]
        CTX[AudioContext]
        BUF[AudioBuffer (22,050 Hz Mono)]
        SRC[AudioBufferSourceNode]
        GAIN[Per-Sample GainNode (Volume Attenuation)]
        MGAIN[Master GainNode]
        DEST[AudioDestinationNode (Hardware Output)]
    end

    EMBED --> WAV
    VOL --> WAV
    WAV --> PCM
    PCM --> CACHE
    MAP --> POOL
    CACHE --> POOL
    POOL --> BUF
    CTX --> BUF
    BUF --> SRC
    SRC --> GAIN
    GAIN --> MGAIN
    MGAIN --> DEST
```

### Architectural Guarantees

1. **Synchronous Zero-Allocation Playback**:
   - `SoundBank::create_audio_buffer` synchronously instantiates a Web Audio `AudioBuffer` directly using `buf.copy_to_channel(&pcm_floats, 0)`.
   - Bypasses asynchronous browser `decodeAudioData` promises, preventing frame stuttering during intense combat.
2. **Channel Pooling & Debouncing**:
   - Caps concurrent active hardware voices at `16` channels.
   - Enforces a 40ms duplicate audio debounce per sound key to prevent acoustic phase cancellation when multiple units are issued movement orders simultaneously.
3. **Volume Attenuation & Sound Balancing**:
   - UI beeps attenuated to 35% comfortable ambient level.
   - Tactical Savant voice dispatches elevated to 85% volume for clear operational readability.
   - Explosions and weapon discharges dynamically balanced between 45% and 65%.
4. **Fallback Resilience**:
   - If an un-embedded voice event ID is requested without runtime archive extraction, `SoundBank::play_voice_event` queries `get_event_transcript(event_id)` and falls back seamlessly to the Web Speech Synthesis API (`RadioDispatcher`).
