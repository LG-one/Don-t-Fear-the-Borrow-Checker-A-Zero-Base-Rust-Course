# 🦀 Rust Fundamentals v3.0

<h3 align="center"><a href="README.md">中文</a> | <a href="README_en.md">English</a></h3>

> From "Hello World" to building a complete command-line notebook app — 15 lessons to get you comfortable with Rust.

## What is this?

A beginner-friendly course for people who have **never touched Rust before**.

This isn't a "here's a list of topics, go figure it out yourself" kind of course — every lesson comes with a **real, runnable project**: temperature converter, number guessing game, poker card dealer, student grade manager, concurrent file processor... By the end, you'll build your own command-line notebook with JSON persistence.

Each lesson wraps up with 3 exercises and `#[test]` tests you can play with. Write your code, run the tests — green means you got it, red means take another look.

## How do I run it?

```bash
# Install Rust (if you haven't already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run any lesson
cargo run --example lesson00    # Hello World
cargo run --example lesson01    # Variables and Types
cargo run --example lesson14    # Final project: Command-line Notebook

# Run tests (uncomment the exercise sections first)
cargo test --example lesson04

# Async programming extras (requires tokio)
cargo run --example lesson_async --features async_support
```

## What will I learn?

### Phase 1: Building the Foundation 🏗️
Learn to walk before you run. Variables, types, functions, ownership — the core of Rust lives here.

| # | What you'll build | What you'll pick up along the way |
|---|-------------------|-----------------------------------|
| 00 | Print a personal info card | cargo, println!, comments |
| 01 | Build a temperature converter | Variables, shadowing, tuples, String vs &str |
| 02 | Make a number guessing game | if/loop/while/for, functions |
| 03 | Create a string processing tool | Ownership, borrowing, slices |

### Phase 2: Building the Framework 🧱
Start stacking blocks. Structs, enums, error handling — make your code look like real software.

| # | What you'll build | What you'll pick up along the way |
|---|-------------------|-----------------------------------|
| 04 | Write a student management system | Structs, methods, Debug/Drop |
| 05 | Build a text highlighter | Lifetimes (don't worry, it's not that scary) |
| 06 | Make a poker card dealer | Enums, Option, match |
| 07 | Write a config file parser | Result, ? operator, custom errors |

### Phase 3: Abstraction 🧠
Start writing "advanced" code. Generics, Traits, iterators, closures — Rust's killer features.

| # | What you'll build | What you'll pick up along the way |
|---|-------------------|-----------------------------------|
| 08 | Build a geometry calculator | Generics, Traits, dyn Trait |
| 09 | Write a grade management system | Vec, HashMap, HashSet |
| 10 | Create a SQL-like query engine | Iterators, closures, Fn traits |

### Phase 4: Advanced Skills ⚡
Modularization, concurrency, Cargo project management — from "writing code" to "building projects".

| # | What you'll build | What you'll pick up along the way |
|---|-------------------|-----------------------------------|
| 11 | Split student management into multi-file modules | mod/use/pub, multi-file modules |
| 12 | Write a concurrent file processor | Threads, Channels, Arc\<Mutex\<T\>\> |
| 13 | Set up a complete Cargo project | Dependencies, features, testing, docs |

### Phase 5: Capstone Project 🎯
Put it all together. This project uses every concept you've learned so far.

| # | What you'll build | What's involved |
|---|-------------------|-----------------|
| 14 | Command-line Notebook | Structs, enums, error handling, Traits, closures, iterators, modules, JSON persistence |

### Bonus 📚
Want more? Try async programming.

| # | What you'll build | What you'll pick up along the way |
|---|-------------------|-----------------------------------|
| - | Async HTTP requester | async/await, tokio, spawn, join!, select! |

## What does the file structure look like?

```
s_rust_new/
├── Cargo.toml                      ← The key to making it run
├── 00_环境与第一个程序.rs           ← Start here
├── 01_变量与数据类型.rs             ← Temperature converter lives here
├── 02_控制流与函数.rs               ← Number guessing game lives here
├── 03_所有权系统.rs                 ← The soul of Rust
├── 04_结构体与方法.rs               ← Student management system
├── 05_生命周期.rs                   ← Don't worry, really not that hard
├── 06_枚举与模式匹配.rs             ← Poker card dealer
├── 07_错误处理.rs                   ← Stop your programs from crashing
├── 08_泛型与Trait.rs                ← Geometry calculator
├── 09_集合类型.rs                   ← Grade management system
├── 10_迭代器与闭包.rs               ← SQL-like query engine
├── 11_模块系统.rs                   ← Code that finally makes sense
├── 12_并发编程.rs                   ← Multi-threaded downloader
├── 13_Cargo项目管理.rs              ← Real project structure
├── 14_综合实战_命令行记事本.rs       ← The grand finale
└── 课外资料_async_await.rs         ← Async programming intro
```

## A Few Tips

1. **Don't skip lessons** — Later lessons build on earlier ones; skip ahead and you'll be lost
2. **Don't just read** — Type the code yourself, run it, change it, run it again
3. **Don't fear errors** — The Rust compiler is the friendliest compiler you'll ever meet; it tells you what went wrong, why, and how to fix it
4. **Don't rush** — Lessons 09 and 10 are packed; splitting them across two days is perfectly fine
5. **Do the exercises** — They're not decoration; they're designed to reinforce what you learned

## What do I need?

- **Rust toolchain** — Install from [rustup.rs](https://rustup.rs/)
- **Editor** — VS Code + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension for the best experience
- **Computer** — Windows / macOS / Linux all work

## By the Numbers

| Metric | Count |
|--------|-------|
| Lessons | 15 + 1 bonus |
| Lines of code | ~13,000 |
| Hands-on projects | 15 |
| Exercises | 45 |
| Test cases | 15 |
| Iteration rounds | 5 rounds of polishing |

## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/) — The official Bible
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) — Small exercises to practice

---

> "If it compiles, the memory is safe." — Rust's promise, and a sentence you'll truly understand after finishing this course.
