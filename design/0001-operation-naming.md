# Operation Naming

* Status: accepted
* Deciders: berwyn
* Date: 2020-08-20

## Context and problem statement

In implemnting a gameboy CPU, we're opting for high-level emulation and as part of that we need to intercept and handle
syscalls. In the purusit of that, we've opted to model each opcode as its own discreet struct with a trait
implementation. To that end, these structs need to be named _something_, and the question becomes whether to keep the
opcode's assembly name (e.g. `ADD`, `SUB`, `RLC`, etc.), or whether to use a more common, friendly name, (e.g.
`Addition`, `Subtraction`, `RotateLeftWithCarry`, etc.).

## Decision drivers

* Vocabulary between emulation code and assembly code
* Ergonomics of the code as a learning resource
* Avoidance of unfortunate contractions

## Options considered

* Using assembly names
* Using descriptive names

## Decision outcome

For the time being, we'll continue to use the assmebly names for operations in the code. The purpose of this code is
to be a reasonable resource to learn about the Gameboy and emulation for someone unfamiliar with the concept, and to
that end it makes more sense to keep the code as close to the system it's emulating as possible.

## Pros and cons of the options

### Using assembly names

e.g. `AddOperation`, `SubOperation`, `RlcOperation`

* Pro: Shared vocabulary between assembly, system manual, and emulation code
* Pro: Concise names
* Con: Some names are arcane, e.g. `RLC`, `RRCA`
* Con: Some names are unfortunate contractions, e.g. `CP`

### Using descriptive names

e.g. `AdditionOperation`, `SubtractionOperation`, `RotateLeftWithCarryOperation`

* Pro: Names describe what the operation does at a glance
* Pro: Fewer contractions, which can help ESL persons looking at the code
* Con: Requires the reader to map the operation name to the equivalent assembly instruction
* Con: Can make it harder to find which operation implements a given opcode without referencing the table
