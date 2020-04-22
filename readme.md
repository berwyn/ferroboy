# Ferroboy
A Gameboy emulator, in Rust

## Motivation
Let's be real here, I was bored one day and was interested in how the Gameboy (and emulation in general) works. I love writing Rust, and used this as an excuse to embark on a big long-term project that uses Rust. Nearly as soon as I dipped my toes into the water of emulation, however, I was dissatisifed with the learning materials at-hand.

While nearly all of them are of high-quality and I do dearly love them all, I didn't like having to build and manage an entire library of bookmarks in order to cross-reference and learn about the Gameboy hardware and how it worked. Moreover, several pages that talk about the DMG-01 link out to generic Zilog Z80 documentation, even though the Gameboy's CPU isn't a complete Z80. Several documents also assumed the reader was familiar with emulation and the status-quo of such, or thought the appropriate learning material was arcane C code. Personally, I don't think this is a good foundation for new developers or developers in other fields looking to learn about and join the emulation scene.

My personal goal here, other than the satisfaction of building something nearly from scratch, is to create a one-stop-shop for new learners to learn about emulation and DMG-01.

## Goals
- [ ] Robust documentation
- [ ] DMG-01 Emulation
- [ ] SGB Emulation
- [ ] SGB2 Emulation
- [ ] Gameboy Camera Emulation
- [ ] Link Cable Emulation
    - [ ] LAN-based multiplayer
    - [ ] WAN-based multiplayer

### Stretch Goals
- [ ] Unlicensed memory mappers
- [ ] One-off accessories (e.g. pocket sonar, Barcode Boy)

### Non-goals
- CGB Emulation
    - Maybe an eventual Ferroboy Color
- AGB-001/AGS-001/OXY-001
    - Ferroboy Advance?
- Gameboy Player (DOL-017)

## Artefacts
### [Ferroboy](ferroboy/readme.md)
A Rust library that implements the emulation

### [Ferroboy Core](ferroboy-core/readme.md)
A [libretro][libretro] core powered by Ferroboy

### [ferroboy-dasm](ferroboy-dasm/readme.md)
A disassembler for DMG-01 ROMs that _attempts_ to output 6502 assembly.

### libferroboy (TODO)
A set of C bindings for Ferroboy. Eventually.


[libretro]: https://www.libretro.com/
