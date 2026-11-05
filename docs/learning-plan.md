# Rust practice plan

This is a twelve-week plan for turning book knowledge into reflexes, at roughly ten hours a week. Because you already know the concepts, the plan never re-teaches them; each topic is a failing test suite you make pass, so the practice is recall under compiler pressure rather than reading. The progress tracker is the Obsidian vault in `vault/`, and every topic there maps to a crate in `crates/`.

The plan has five phases, each chosen because it forces a different skill. 

*Phase 1* (weeks 1 to 4) drills the core language in `crates/p1-core-drills`, one module per topic, so ownership, enums, errors, traits, lifetimes, iterators and smart pointers become automatic. 

*Phase 2* (weeks 5 and 6) turns that into two small command-line tools, minigrep and a todo app, which is where you meet Cargo, modules, testing and file I/O the way real projects do. 

*Phase 3* (weeks 7 and 8) builds data structures, because recursive and self-referential types are where the borrow checker hurts most and teaches most, and then implements standard-library traits such as `IntoIterator`, `Index` and `Deref` on them. 

*Phase 4* (weeks 9 and 10) covers threads, channels and `Arc<Mutex<_>>`, then async with Tokio, which are the two concurrency models you will meet in production code. 

*Phase 5* (weeks 11 and 12) is a typed web service with axum, serde and sqlx, followed by a capstone of your choosing, so you finish with something shippable and with the ecosystem crates that most real Rust jobs use.

Each session follows one loop so that the effort goes into recall instead of reading. Start by reading only the exercise stub and its tests, write the solution without looking anything up, and let the compiler be the first reviewer. When you get stuck, write the error into `vault/Mistakes/` using the template before fixing it, since the recurring mistakes are the ones worth drilling later. Finish by logging hours in a journal note and updating the topic's `status` and `confidence`; the dashboard will then show which finished topics are due for a spaced review after fourteen days, and you redo one exercise from a blank file at that point.

Two rules keep the practice honest. Do not use `clone()` to silence the borrow checker until you can say why the borrow fails, and do not use `unwrap()` outside tests. Run `cargo fmt` and `cargo clippy` before calling a topic done, because reading clippy's suggestions is one of the fastest ways to learn idiomatic style.

The plan deliberately leaves the async, web and capstone crates uncreated, because their dependencies should be added when you get there with `cargo new` and `cargo add`, and a stale pinned version would only get in the way. Setup steps, the weekly schedule and the resource list are in the appendices below.

## Appendix A: setup

Install the toolchain with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`; `rust-toolchain.toml` then pulls rustfmt, clippy and rust-analyzer automatically. Open `vault/` as an Obsidian vault (File, Open folder as vault) and enable the core Templates plugin pointing at `Templates/`; Bases is a core feature in current Obsidian versions. Useful commands: `cargo test -p p1-core-drills` runs one crate, `cargo test -p p1-core-drills ownership` runs one module, `cargo clippy --workspace --all-targets` lints everything, and `cargo run -p p2-minigrep -- needle file.txt` runs a binary. Tests fail on purpose until you replace each `todo!()`, so a red `cargo test --workspace` at the start is expected.

## Appendix B: schedule

| Week | Topic | Crate | Output |
|------|-------|-------|--------|
| 1 | Ownership and borrowing; structs, enums, matching | p1-core-drills | ownership.rs, structs_enums.rs green |
| 2 | Collections; error handling | p1-core-drills | collections.rs, errors.rs green, errors redone with thiserror |
| 3 | Traits and generics; lifetimes | p1-core-drills | traits_generics.rs, lifetimes.rs green |
| 4 | Iterators and closures; smart pointers | p1-core-drills | iterators.rs, smart_pointers.rs green |
| 5 | Modules, Cargo, testing; minigrep | p2-minigrep | CLI with flags, integration test, doc test |
| 6 | Todo CLI | p2-todo-cli | persistence, then serde_json and clap |
| 7 | Data structures | p3-data-structures | stack, linked stack, BST, LRU |
| 8 | Advanced traits and types | p3-data-structures | IntoIterator, Index, Add, FromStr on your own types |
| 9 | Threads, channels, mutex | p4-concurrency | parallel sums, thread pool |
| 10 | Async and Tokio | p4-async (create it) | concurrent fetcher with timeouts |
| 11 | Web service | p5-web-service (create it) | axum + sqlx CRUD API with tracing and tests |
| 12 | Capstone | p5-capstone (create it) | design note, implementation, CI, README |

## Appendix C: workspace layout

```
learn-rust/
  Cargo.toml              workspace, members = crates/*
  rust-toolchain.toml     stable + rustfmt, clippy, rust-analyzer
  crates/
    p1-core-drills/       8 modules of stubbed functions with tests
    p2-minigrep/          lib + bin
    p2-todo-cli/          lib + bin
    p3-data-structures/   stack, linked_stack, bst, lru
    p4-concurrency/       parallel, pool
  docs/learning-plan.md   this file
  vault/                  Obsidian vault (progress tracking)
    Dashboard.md          embeds the Bases below
    Curriculum.base       topics by phase, in progress, review queue, weak spots
    Projects.base  Journal.base  Mistakes.base
    Topics/ Projects/ Journal/ Mistakes/ Templates/
```

Create later crates with `cargo new crates/p4-async --lib` (the workspace glob picks them up) and add dependencies with `cargo add tokio --features full`.

## Appendix D: vault properties

| Property | Where | Meaning |
|----------|-------|---------|
| status | topic, project | not-started, in-progress, done |
| confidence | topic | 0 untouched, 1 lost, 3 workable with docs, 5 could teach it |
| hours | topic, project, journal | time spent; summed in the dashboard |
| last_reviewed | topic | date of last spaced-review pass; drives the review queue |
| crate | topic, project | crate under crates/ where the code lives |
| resolved | mistake | whether you can now explain and avoid it |

## Appendix E: resources

- The Rust Book (https://doc.rust-lang.org/book/), used only as reference now.
- Rustlings (https://github.com/rust-lang/rustlings) for quick per-topic warm-ups.
- Exercism Rust track (https://exercism.org/tracks/rust) for iterator and idiom practice.
- Rust by Example (https://doc.rust-lang.org/rust-by-example/) for syntax lookup.
- Learning Rust With Entirely Too Many Linked Lists (https://rust-unofficial.github.io/too-many-lists/) for phase 3.
- Rust API Guidelines (https://rust-lang.github.io/api-guidelines/) and Clippy lint docs for style.
- Asynchronous Programming in Rust (https://rust-lang.github.io/async-book/) and the Tokio tutorial (https://tokio.rs/tokio/tutorial) for phase 4.
- Jon Gjengset's "Crust of Rust" videos for deep dives on lifetimes, iterators and smart pointers.
