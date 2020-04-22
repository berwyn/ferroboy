# Ferroboy

A Rust library that handles Gameboy emulation!

## The Road to 1.0
### API Surface
The initial 1.0 API surface is currently expected to be setup/configuration, startup, and tick functions. Because the current target is libretro, 1.0 is intended to be the minimum to get libretro working.

### Architecture
The current expected architecture primarily revolves around the `State` struct. `State` is intended to fully encapsulate the current state of the system, including the CPU registers, RAM, memory-mapped hardware, interrupts, etc. External actors (like `ferroboy-core`) are expected to maintain a reference to the `State` struct and pass it to the various functions the library exposes.

Internally, each opcode is mapped to a struct that implements `Operation`. The implementation of `Operation` should fully encapsulate all mutations to the `State` that the operation should encapsulate.

The ideal final state is that external actors don't have any mutable properties exposed, and every interaction should be locked to `pub` functions at the root.
