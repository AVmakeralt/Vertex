#  Vertex Roadmap

The goal of Vertex is to become a high-performance, statically-typed systems language that remains simple and expressive. This roadmap tracks our progress from the initial alpha stages to a production-ready toolchain.

---

##  Current Phase: Alpha (v0.1.x)
*Focus: Stabilizing the core compiler, refining the bytecode VM, and building developer tools.*

###  Completed Milestones
- **Lexer & Parser**
  - [x] Full keyword and operator support (+, -, *, /, %, ==, etc.)
  - [x] Multi-file project support via recursive flattening imports.
  - [x] **Parallel Lexing**: High-performance tokenization using Rayon.
- **Compiler & VM**
  - [x] AST-to-Bytecode emission for core control flow (If, While, Loops).
  - [x] Robust recursion and nested function scope support.
  - [x] Constant folding optimization.
  - [x] Implicit function exits and memory cleanup (Drop instructions).
- **Developer UX**
  - [x] **Apex**: Initial version of the Vertex project manager.
  - [x] **Detailed Diagnostics**: Color-coded error explanations for all compiler codes (`E0001-E0017`).
  - [x] **Regression Testing**: Python-based automated test suite.
- **Linker**
  - [x] Dependency sorting (BFS) and cyclic import detection.

---

##  Upcoming Milestones

###  Milestone 1: The "Apex" Overhaul (v0.2.0)
*Goal: Transform Apex into a true package manager.*
- [ ] **Dependency Management**: Support for external libraries (Git repos or local paths).
- [ ] **Lockfiles**: Implement `apex.lock` for reproducible builds.
- [ ] **Custom Targets**: Support for specific optimization levels (-O1, -O2).
- [ ] **Standard Library**: Begin `std/` development (Core collections, String manipulation).

###  Milestone 2: Advanced Language Features
- [ ] **Structs & Data Layout**: Complete the implementation of custom types and member access.
- [ ] **Arrays & Slices**: Proper bounds-checked array support.
- [ ] **Pointers/References**: Safe memory referencing.
- [ ] **Generics**: Initial design for parametric polymorphism.

###  Milestone 3: Performance & Optimization
- [ ] **True Symbol Resolution**: Move away from flattening imports to prevent code duplication.
- [ ] **Instruction Optimization**: Dead code elimination and jump threading.
- [ ] **VM JIT**: Researching a Just-In-Time compilation layer for the bytecode VM.
- [ ] **Profiling Integration**: Built-in support for generating flamegraphs via Apex.

---

##  Long-Term Vision (v1.0.0+)
- [ ] **Self-Hosting**: Re-writing the Vertex compiler in Vertex.
- [ ] **Ecosystem**: A central registry for Vertex packages.
- [ ] **Language Server (LSP)**: IDE support for autocomplete and syntax highlighting.

---

*Found a bug or want to contribute? Check out [CONTRIBUTING.md](CONTRIBUTING.md) or open an issue!*
