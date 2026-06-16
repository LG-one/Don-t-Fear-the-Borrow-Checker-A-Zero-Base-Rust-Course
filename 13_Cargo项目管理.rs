// ============================================================================
// 第 13 课：Cargo 项目管理 —— Rust 的"超级管家"
// ============================================================================
//
// 想象一下：你搬进了一套新房子，需要管家帮你：
//   - 购买家具（下载依赖）
//   - 安排装修流程（编译项目）
//   - 质量验收（运行测试）
//   - 整理文档（生成文档）
//   - 打扫卫生（清理缓存）
//
// Cargo 就是 Rust 的"超级管家"，它帮你管理项目的一切。
// 前 12 课我们一直在单文件里写代码，但从今天开始，
// 你要学会用 Cargo 来组织真正的 Rust 项目！
//
// 本课内容：
//   1. cargo new —— 创建项目
//   2. Cargo.toml —— 项目配置文件
//   3. Cargo.lock —— 锁定依赖版本
//   4. 常用命令大全
//   5. 依赖管理
//   6. 特性开关 features
//   7. 项目结构
//   8. 工作空间 workspace
//   9. 测试
//  10. 文档注释
//  11. 完整实战演示
// ============================================================================

// ============================================================================
// 第一部分：cargo new —— 创建你的第一个项目
// ============================================================================
//
// 【生活类比】
// cargo new 就像开发商交给你一套毛坯房，已经有了基本结构：
//   - 大门钥匙（Cargo.toml 配置文件）
//   - 客厅（src/main.rs 主文件）
//   - 说明书（README.md）
//   - 装修记录本（.gitignore）
//
// 两种户型：
//   cargo new my_app        → 二进制项目（有 main 函数，能直接运行）
//   cargo new my_lib --lib  → 库项目（写给别人用的代码，不能直接运行）
//
// 【命令演示】（在终端中执行，不是在这个文件里）
//
//   # 创建一个二进制项目（默认）
//   $ cargo new hello_cargo
//       Creating binary (application) `hello_cargo` package
//
//   # 创建一个库项目
//   $ cargo new my_math_lib --lib
//       Creating library `my_math_lib` package
//
//   # 生成的目录结构：
//   hello_cargo/
//   ├── Cargo.toml          ← 项目配置（相当于身份证）
//   ├── src/
//   │   └── main.rs         ← 主代码文件（二进制项目）
//   ├── .gitignore          ← Git 忽略规则
//   └── README.md           ← 项目说明
//
//   my_math_lib/
//   ├── Cargo.toml
//   ├── src/
//   │   └── lib.rs          ← 主代码文件（库项目）
//   ├── .gitignore
//   └── README.md
//
// 注意：二进制项目入口是 src/main.rs，库项目入口是 src/lib.rs
// ============================================================================

fn main() {
    println!("=== 第 13 课：Cargo 项目管理 ===\n");

    // 模拟创建项目的过程
    demo_cargo_new();
    demo_cargo_toml();
    demo_cargo_lock();
    demo_common_commands();
    demo_dependency_management();
    demo_features();
    demo_project_structure();
    demo_workspace();
    demo_testing();
    demo_doc_comments();

    // 完整实战演示
    practical_demo();

    println!("\n=== 本课结束 ===");
}

// ============================================================================
// 第二部分：Cargo.toml —— 项目的"身份证"
// ============================================================================
//
// 【生活类比】
// Cargo.toml 就像你的身份证 + 购物清单：
//   - [package]    = 身份证（你是谁、多大、住哪）
//   - [dependencies] = 购物清单（需要买什么材料）
//   - [dev-dependencies] = 装修工具（只在装修时用，住进去就收起来了）
//
// # ----- 一个真实的 Cargo.toml 长这样 -----
//
// [package]
// name = "hello_cargo"           # 项目名称（全部小写，用 - 分隔）
// version = "0.1.0"              # 版本号（语义化版本：主.次.修订）
// edition = "2021"               # Rust 版本（2015 / 2018 / 2021 / 2024）
// authors = ["G-one <g-one@example.com>"]  # 作者信息
// description = "一个示例项目"    # 简短描述
// license = "MIT"                # 开源协议
// repository = "https://github.com/..."    # 代码仓库地址
//
// [dependencies]
// serde = "1.0"                  # 运行时依赖：序列化框架
// serde_json = "1.0"             # 运行时依赖：JSON 解析
// rand = "0.8"                   # 运行时依赖：随机数
//
// [dev-dependencies]
// criterion = "0.5"              # 测试依赖：性能基准测试
//
// [build-dependencies]
// cc = "1.0"                     # 编译依赖：C 代码编译
//
// -----
//
// 版本号含义（语义化版本 SemVer）：
//   "1.0.0"  = 主版本.次版本.修订版本
//   主版本：不兼容的 API 修改
//   次版本：向下兼容的功能新增
//   修订版本：向下兼容的问题修正
//
// 版本范围写法：
//   "1.0"     等于 "^1.0.0"，即 >=1.0.0, <2.0.0
//   "~1.0.2"  等于 >=1.0.2, <1.1.0（只允许修订版本变化）
//   ">=1.0, <1.5"  明确指定范围
//   "1.0.*"   通配符，匹配 1.0.x
//   "=1.0.7"  精确锁定版本
// ============================================================================

fn demo_cargo_toml() {
    println!("--- 第二部分：Cargo.toml 配置 ---");

    // 模拟解析 Cargo.toml 的内容
    // 在真实项目中，Cargo 会自动读取并解析这个文件
    let package_name = "hello_cargo";
    let version = "0.1.0";
    let edition = "2021";

    println!("项目名称: {}", package_name);
    println!("版本号:   {}", version);
    println!("Rust版本: {}", edition);
    println!();
}

// ============================================================================
// 第三部分：Cargo.lock —— 版本"锁定器"
// ============================================================================
//
// 【生活类比】
// 如果 Cargo.toml 是购物清单（写"苹果 1斤"），
// 那 Cargo.lock 就是购物小票（记下你买了"山东红富士苹果 1.02斤"）。
//
// Cargo.lock 的作用：
//   1. 记录所有依赖的精确版本号
//   2. 保证团队每个人用的都是完全相同的版本
//   3. 保证 CI/CD 和本地环境一致
//
// 什么时候该提交 Cargo.lock？
//   - 二进制项目（应用程序）：必须提交！保证所有人用同一版本
//   - 库项目：通常不提交，让使用者自己解析版本
//
// 什么时候会更新 Cargo.lock？
//   - cargo update          → 更新所有依赖到允许的最新版
//   - cargo update -p serde → 只更新指定依赖
//   - 手动改 Cargo.toml    → 重新解析时自动更新
// ============================================================================

fn demo_cargo_lock() {
    println!("--- 第三部分：Cargo.lock ---");

    // 模拟 Cargo.lock 中记录的精确版本
    // 在真实项目中，Cargo.lock 是一个 YAML 格式的文件
    let locked_versions = vec![
        ("serde", "1.0.193"),
        ("serde_json", "1.0.108"),
        ("itoa", "1.0.9"),
        ("ryu", "1.0.15"),
    ];

    println!("Cargo.lock 锁定的版本：");
    for (name, ver) in &locked_versions {
        println!("  {} = \"{}\"", name, ver);
    }

    println!("\n规则：");
    println!("  应用程序 → 提交 Cargo.lock");
    println!("  库项目   → 不提交 Cargo.lock");
    println!();
}

// ============================================================================
// 第四部分：常用命令大全
// ============================================================================
//
// 【生活类比】
// 把这些命令想象成管家的日常工作：
//
//   cargo new       = 搭建新房
//   cargo build     = 装修施工
//   cargo run       = 装修完直接住进去
//   cargo check     = 质量检查（不装修，只检查图纸）
//   cargo test      = 验收测试
//   cargo doc       = 写说明书
//   cargo fmt       = 整理房间
//   cargo clippy    = 请专家提建议
//   cargo clean     = 大扫除
//   cargo update    = 更新材料
//
// 详细说明：
//
// # ---------- cargo build ----------
// # 编译项目，生成可执行文件到 target/debug/
// $ cargo build
//     Compiling hello_cargo v0.1.0 (/path/to/hello_cargo)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.50s
//
// # release 模式编译（优化级别高，编译慢，运行快）
// $ cargo build --release
// # 生成文件在 target/release/
//
// # ---------- cargo run ----------
// # 编译并立即运行（开发时最常用！）
// $ cargo run
//     Compiling hello_cargo v0.1.0
//     Finished dev [unoptimized + debuginfo] target(s) in 0.30s
//      Running `target/debug/hello_cargo`
// Hello, world!
//
// # 传递命令行参数
// $ cargo run -- arg1 arg2
// # 程序中用 std::env::args() 读取
//
// # ---------- cargo check ----------
// # 只检查代码是否能编译，不生成可执行文件（比 build 快！）
// $ cargo check
//     Checking hello_cargo v0.1.0
//     Finished dev [unoptimized + debuginfo] target(s) in 0.15s
//
// # 用途：快速检查语法错误，节省编译时间
// # 建议：写代码时多用 cargo check，少用 cargo build
//
// # ---------- cargo test ----------
// # 运行所有测试
// $ cargo test
//    Compiling hello_cargo v0.1.0
//     Finished test [unoptimized + debuginfo] target(s) in 0.50s
//      Running unittests src/main.rs
// running 2 tests
// test tests::it_works ... ok
// test tests::test_add ... ok
//
// test result: ok. 2 passed; 0 failed; 0 ignored
//
// # 只运行名字包含某个关键词的测试
// $ cargo test test_add
//
// # ---------- cargo doc ----------
// # 生成 HTML 文档到 target/doc/
// $ cargo doc
// # 生成文档并自动在浏览器中打开
// $ cargo doc --open
//
// # ---------- cargo fmt ----------
// # 格式化代码（统一代码风格）
// $ cargo fmt
// # 只检查是否有格式问题，不修改
// $ cargo fmt -- --check
//
// # ---------- cargo clippy ----------
// # 静态分析工具，给出代码改进建议
// $ cargo clippy
// # 常见建议：
// #   - 变量遮蔽可以用更好
// #   - 不必要的 clone
// #   - 可以用更简洁的写法
//
// # ---------- cargo clean ----------
// # 清理 target/ 目录，释放磁盘空间
// $ cargo clean
#
// # ---------- cargo update ----------
// # 更新 Cargo.lock 中的依赖版本
// $ cargo update
// # 只更新某个依赖
// $ cargo update -p serde
// ============================================================================

fn demo_common_commands() {
    println!("--- 第四部分：常用命令 ---");

    // 这些命令都在终端中使用，这里用代码模拟它们的效果
    let commands = vec![
        ("cargo new my_app", "创建二进制项目"),
        ("cargo new my_lib --lib", "创建库项目"),
        ("cargo build", "编译项目（debug 模式）"),
        ("cargo build --release", "编译项目（release 模式）"),
        ("cargo run", "编译并运行"),
        ("cargo check", "只检查编译，不生成文件（快！）"),
        ("cargo test", "运行测试"),
        ("cargo doc --open", "生成文档并打开"),
        ("cargo fmt", "格式化代码"),
        ("cargo clippy", "静态分析，给出改进建议"),
        ("cargo clean", "清理编译缓存"),
        ("cargo update", "更新依赖版本"),
    ];

    for (cmd, desc) in &commands {
        println!("  {:<30} → {}", cmd, desc);
    }
    println!();
}

// ============================================================================
// 第五部分：依赖管理 —— 别人的代码拿来就用
// ============================================================================
//
// 【生活类比】
// 写程序就像做菜，你不需要自己种菜（从零写所有代码），
// 而是去超市（crates.io）买现成的食材（crate）。
//
// crates.io 是 Rust 的官方"超市"，里面有超过 15 万个 crate。
//
// 添加依赖的三种方式：
//
// # 方式 1：从 crates.io 下载（最常用）
// # 在 Cargo.toml 的 [dependencies] 下添加：
// [dependencies]
// serde = "1.0"
// rand = "0.8"
//
// # 或者用命令自动添加：
// $ cargo add serde
// $ cargo add rand@0.8       # 指定版本
// $ cargo add serde --features derive   # 启用特性
//
// # 方式 2：从 Git 仓库下载
// [dependencies]
// my_lib = { git = "https://github.com/user/my_lib.git" }
// my_lib = { git = "https://github.com/user/my_lib.git", branch = "main" }
// my_lib = { git = "https://github.com/user/my_lib.git", tag = "v1.0" }
// my_lib = { git = "https://github.com/user/my_lib.git", rev = "abc123" }
//
// # 方式 3：使用本地路径（开发多个相关项目时常用）
// [dependencies]
// my_utils = { path = "../my_utils" }     # 相对路径
// my_utils = { path = "/absolute/path" }  # 绝对路径
//
// -----
//
// 常用 crate 推荐：
//   serde / serde_json → 数据序列化（JSON、YAML 等）
//   rand               → 随机数生成
//   tokio              → 异步运行时
//   reqwest            → HTTP 客户端
//   clap               → 命令行参数解析
//   log / env_logger   → 日志系统
//   anyhow / thiserror → 错误处理
// ============================================================================

fn demo_dependency_management() {
    println!("--- 第五部分：依赖管理 ---");

    // 模拟几种不同的依赖来源
    println!("依赖来源：");
    println!("  1. crates.io  → serde = \"1.0\"              （最常用）");
    println!("  2. Git 仓库  → my_lib = {{ git = \"...\" }}    （用别人未发布的代码）");
    println!("  3. 本地路径  → my_utils = {{ path = \"..\" }}  （自己多个项目间引用）");

    // 模拟使用 serde_json 的功能
    // 在真实项目中，需要在 Cargo.toml 添加：serde_json = "1.0"
    // 这里我们手动模拟 JSON 解析的结果
    println!("\n模拟 JSON 解析（serde_json 的功能）：");
    let json_str = r#"{"name": "G-one", "age": 25, "skills": ["Rust", "Python"]}"#;
    println!("  输入: {}", json_str);

    // 手动解析 JSON 字符串（模拟 serde_json 的行为）
    let parsed = simulate_json_parse(json_str);
    println!("  解析结果: {:?}", parsed);
    println!();
}

/// 模拟 JSON 解析（演示 serde_json 的功能）
/// 在真实项目中，直接用 serde_json::from_str() 就行
fn simulate_json_parse(json: &str) -> std::collections::HashMap<String, String> {
    let mut result = std::collections::HashMap::new();

    // 简单的模拟解析：提取 "key": "value" 形式的内容
    // 真实项目中用 serde_json::from_str() 一行搞定
    let json = json.trim_matches(|c| c == '{' || c == '}');
    for pair in json.split(',') {
        let pair = pair.trim();
        if let Some(colon_pos) = pair.find(':') {
            let key = pair[..colon_pos].trim().trim_matches('"').to_string();
            let value = pair[colon_pos + 1..].trim();
            // 处理字符串值
            let value = if value.starts_with('"') && value.ends_with('"') {
                value[1..value.len() - 1].to_string()
            } else {
                value.to_string()
            };
            result.insert(key, value);
        }
    }
    result
}

// ============================================================================
// 第六部分：features 特性开关 —— 按需定制
// ============================================================================
//
// 【生活类比】
// 买手机时可以选择不同配置：
//   - 基础版：只能打电话发短信
//   - 标准版：基础版 + 上网 + 拍照
//   - 旗舰版：标准版 + 5G + 高级摄像头
//
// features 就是 Rust 的"配置选项"，让你按需启用功能。
//
// ----- 在 Cargo.toml 中定义 features -----
//
// [package]
// name = "my_lib"
//
// [features]
// # 默认启用的特性
// default = ["logging"]
//
// # 可选特性
// logging = []                    # 启用日志功能
// advanced = ["logging"]          # 高级功能依赖 logging
// json = ["dep:serde_json"]       # 启用 JSON 支持（依赖另一个 crate）
//
// [dependencies]
// serde_json = { version = "1.0", optional = true }  # 可选依赖
//
// ----- 使用 features -----
//
// # 在代码中用条件编译：
// #[cfg(feature = "logging")]
// fn log_message(msg: &str) {
//     println!("[LOG] {}", msg);
// }
//
// # 在 Cargo.toml 中启用别人的 features：
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
//
// # 或者用命令行：
// $ cargo build --features "logging,json"
// $ cargo build --no-default-features
// ============================================================================

fn demo_features() {
    println!("--- 第六部分：features 特性开关 ---");

    // 模拟一个带 feature 的库
    // 真实项目中，这些是条件编译的
    println!("模拟 features 效果：");

    // 模拟 logging feature 启用时的行为
    log_message("程序启动", true);

    let result = calculate(10, 5, "+");
    log_message(&format!("计算结果: 10 + 5 = {}", result), true);

    let result = calculate(10, 5, "-");
    log_message(&format!("计算结果: 10 - 5 = {}", result), true);

    // 模拟 logging feature 未启用时（静默模式）
    log_message("这条消息不会显示", false);

    println!();
}

/// 模拟带 feature 开关的日志函数
/// 在真实项目中会用 #[cfg(feature = "logging")] 条件编译
fn log_message(msg: &str, logging_enabled: bool) {
    if logging_enabled {
        println!("  [LOG] {}", msg);
    }
    // logging 未启用时，什么都不打印
}

/// 模拟一个支持加减法的计算器
fn calculate(a: i32, b: i32, op: &str) -> i32 {
    match op {
        "+" => a + b,
        "-" => a - b,
        _ => 0,
    }
}

// ============================================================================
// 第七部分：项目结构 —— 代码的"房间布局"
// ============================================================================
//
// 【生活类比】
// 一个好的房子要有合理的布局：
//   - 客厅（src/main.rs）→ 迎接客人（程序入口）
//   - 卧室（src/lib.rs）  → 私密空间（核心逻辑）
//   - 储藏室（src/bin/）  → 多个独立的小房间（多个可执行文件）
//   - 测试间（tests/）    → 质量检测（集成测试）
//   - 展示厅（examples/） → 样板间（使用示例）
//   - 赛道（benches/）    → 性能测试（基准测试）
//
// 完整项目结构：
//
// my_project/
// ├── Cargo.toml
// ├── Cargo.lock
// ├── src/
// │   ├── main.rs          # 二进制入口（和 lib.rs 可以共存！）
// │   ├── lib.rs           # 库入口（被 main.rs 和外部引用）
// │   ├── utils.rs         # 工具模块
// │   ├── config.rs        # 配置模块
// │   └── bin/             # 额外的可执行文件
// │       ├── server.rs    # cargo run --bin server
// │       └── client.rs    # cargo run --bin client
// ├── tests/               # 集成测试
// │   └── integration_test.rs
// ├── examples/            # 使用示例
// │   └── basic_usage.rs   # cargo run --example basic_usage
// ├── benches/             # 基准测试
// │   └── benchmark.rs
// └── build.rs             # 构建脚本（高级用法）
//
// 模块系统回顾（第 12 课的内容）：
//   - 在 main.rs 或 lib.rs 中用 mod 声明模块
//   - mod utils;  →  Rust 会找 src/utils.rs 或 src/utils/mod.rs
//   - 父模块用 pub use 把子模块的内容"提升"到自己的层级
// ============================================================================

fn demo_project_structure() {
    println!("--- 第七部分：项目结构 ---");

    println!("标准目录结构：");
    println!("  src/main.rs      → 二进制入口");
    println!("  src/lib.rs       → 库入口");
    println!("  src/bin/         → 额外的可执行文件");
    println!("  tests/           → 集成测试");
    println!("  examples/        → 使用示例");
    println!("  benches/         → 基准测试");
    println!("  build.rs         → 构建脚本（可选）");

    // 演示 bin 目录的用法
    println!("\n多个二进制文件的用法：");
    println!("  cargo run                → 运行 src/main.rs");
    println!("  cargo run --bin server   → 运行 src/bin/server.rs");
    println!("  cargo run --bin client   → 运行 src/bin/client.rs");
    println!();
}

// ============================================================================
// 第八部分：workspace 工作空间 —— 管理多个相关项目
// ============================================================================
//
// 【生活类比】
// 如果你是一个建筑公司，同时管理多个工地（项目），
// 工作空间就是你的"总部"，统一管理所有工地的材料和标准。
//
// 工作空间的好处：
//   1. 共享 Cargo.lock（所有项目用同一版本的依赖）
//   2. 共享 target/ 目录（节省编译时间和磁盘空间）
//   3. 统一管理依赖版本
//   4. 一次编译所有项目
//
// 项目结构：
//
// my_workspace/
// ├── Cargo.toml           ← 工作空间根配置
// ├── Cargo.lock           ← 所有项目共享
// ├── app/                 ← 二进制项目
// │   ├── Cargo.toml
// │   └── src/main.rs
// ├── core/                ← 核心库
// │   ├── Cargo.toml
// │   └── src/lib.rs
// └── utils/               ← 工具库
//     ├── Cargo.toml
//     └── src/lib.rs
//
// 根 Cargo.toml 内容：
// [workspace]
// members = [
//     "app",     # 二进制项目
//     "core",    # 核心库
//     "utils",   # 工具库
// ]
//
// app 的 Cargo.toml：
// [dependencies]
// core = { path = "../core" }     # 引用工作空间内的核心库
// utils = { path = "../utils" }   # 引用工作空间内的工具库
//
// 常用命令：
//   cargo build                → 编译所有成员
//   cargo build -p app         → 只编译 app
//   cargo test                 → 测试所有成员
//   cargo test -p core         → 只测试 core
// ============================================================================

fn demo_workspace() {
    println!("--- 第八部分：workspace 工作空间 ---");

    // 模拟工作空间的成员
    let workspace_members = vec![
        ("app", "二进制项目", "引用 core 和 utils"),
        ("core", "核心库", "被 app 和 utils 引用"),
        ("utils", "工具库", "被 app 引用"),
    ];

    println!("工作空间成员：");
    for (name, desc, deps) in &workspace_members {
        println!("  {:<10} {:<12} → {}", name, desc, deps);
    }

    println!("\n根 Cargo.toml 配置：");
    println!("  [workspace]");
    println!("  members = [\"app\", \"core\", \"utils\"]");
    println!();
}

// ============================================================================
// 第九部分：测试 —— 代码的"质量保证"
// ============================================================================
//
// 【生活类比】
// 产品出厂前要经过质检：
//   - 单元测试 = 零件检测（每个零件单独检查）
//   - 集成测试 = 整机测试（所有零件组装后检查）
//
// 测试函数的要求：
//   1. 用 #[test] 标注
//   2. 函数没有参数
//   3. 用 assert!、assert_eq!、assert_ne! 验证结果
//
// 测试运行方式：
//   cargo test              → 运行所有测试
//   cargo test test_name    → 运行名字包含 test_name 的测试
//   cargo test -- --show-output  → 显示 println! 输出
// ============================================================================

// --- 被测试的函数 ---

/// 计算两个数的和
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 计算两个数的差
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// 判断一个数是否为偶数
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// 计算斐波那契数列的第 n 项
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn demo_testing() {
    println!("--- 第九部分：测试 ---");

    // 在 main 函数里调用这些函数，验证它们正常工作
    println!("函数测试结果：");
    println!("  add(3, 5) = {}", add(3, 5));
    println!("  subtract(10, 4) = {}", subtract(10, 4));
    println!("  is_even(4) = {}", is_even(4));
    println!("  is_even(7) = {}", is_even(7));
    println!("  fibonacci(10) = {}", fibonacci(10));

    // 注意：真正的 #[test] 函数不能放在 main.rs 的普通代码里
    // 它们通常放在文件末尾的 #[cfg(test)] mod tests {} 块中
    // 或者放在 tests/ 目录下（集成测试）
    //
    // 运行测试的命令：
    //   cargo test            → 运行所有测试
    //   cargo test test_add   → 只运行名字含 test_add 的测试
    //   cargo test -- --show-output  → 显示测试中的 println! 输出

    println!("\n测试类型：");
    println!("  单元测试 → 放在 src/ 内，用 #[cfg(test)] mod tests {{}}");
    println!("  集成测试 → 放在 tests/ 目录下，测试公共 API");
    println!();
}

// ============================================================================
// 第十部分：文档注释 —— 给代码写"说明书"
// ============================================================================
//
// 【生活类比】
// 买家电时，包装盒里一定有说明书。好的说明书让用户一看就懂。
// 文档注释就是代码的"说明书"，用 cargo doc 可以生成漂亮的网页文档。
//
// 文档注释的写法：
//   /// 三斜线注释 → 用于函数、结构体、枚举等（最常用）
//   //! 感叹号注释 → 用于模块、crate 的顶层说明
//
// 文档中的特殊语法（支持 Markdown）：
//   # Examples          → 示例代码（会被自动测试！）
//   # Panics            → 什么情况会 panic
//   # Errors            → 什么情况会返回错误
//   # Safety            → unsafe 相关说明
// ============================================================================

/// 计算圆的面积
///
/// # 参数
/// * `radius` - 圆的半径，必须为非负数
///
/// # 返回值
/// 返回圆的面积，类型为 f64
///
/// # Examples
/// ```
/// let area = circle_area(5.0);
/// assert!((area - 78.539816).abs() < 0.001);
/// ```
///
/// # Panics
/// 如果 radius 为负数，会 panic
fn circle_area(radius: f64) -> f64 {
    if radius < 0.0 {
        panic!("半径不能为负数: {}", radius);
    }
    std::f64::consts::PI * radius * radius
}

/// 安全除法：避免除以零的 panic
///
/// 当除数为零时返回 `None`，否则返回 `Some(结果)`
///
/// # Examples
/// ```
/// assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
/// assert_eq!(safe_divide(10.0, 0.0), None);
/// ```
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn demo_doc_comments() {
    println!("--- 第十部分：文档注释 ---");

    println!("文档注释的两种形式：");
    println!("  /// 三斜线  → 函数、结构体、枚举的文档");
    println!("  //! 感叹号  → 模块、crate 的顶层文档");

    // 使用带文档注释的函数
    let area = circle_area(5.0);
    println!("\ncircle_area(5.0) = {:.6}", area);

    match safe_divide(10.0, 3.0) {
        Some(result) => println!("safe_divide(10.0, 3.0) = {:.4}", result),
        None => println!("safe_divide(10.0, 3.0) = None"),
    }

    match safe_divide(10.0, 0.0) {
        Some(result) => println!("safe_divide(10.0, 0.0) = {}", result),
        None => println!("safe_divide(10.0, 0.0) = None（除零保护生效）"),
    }

    // 生成文档的命令：
    //   cargo doc           → 生成 HTML 文档到 target/doc/
    //   cargo doc --open    → 生成并自动在浏览器打开
    //   cargo test          → 会自动运行文档中的示例代码（doc test）！
    println!("\n提示：cargo test 会自动测试文档中的 # Examples 代码块！");
    println!();
}

// ============================================================================
// 第十一部分：完整实战演示 —— 模拟一个 Cargo 项目
// ============================================================================
//
// 下面演示一个完整的 Cargo 项目，包含：
//   1. 库代码（lib.rs 中的模块）
//   2. 主程序（main.rs 中使用库）
//   3. 单元测试
//   4. 文档注释
//
// 真实项目中的文件结构：
//
// my_calculator/
// ├── Cargo.toml
// ├── src/
// │   ├── main.rs           ← 使用下面的库
// │   ├── lib.rs            ← 定义 pub mod math; pub mod string_utils;
// │   ├── math.rs           ← 数学运算模块
// │   └── string_utils.rs   ← 字符串工具模块
// └── tests/
//     └── integration.rs    ← 集成测试
//
// 由于这是单文件教学，我们用内联模块来模拟多文件结构。
// ============================================================================

// ----- 模拟 lib.rs 中的内容 -----
// 在真实项目中，这些会分别放在不同的文件中

/// 数学运算模块
///
/// 提供基本的数学运算功能
mod math {
    /// 计算两个数的和
    ///
    /// # Examples
    /// ```
    /// let result = math::add(2, 3);
    /// assert_eq!(result, 5);
    /// ```
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// 计算两个数的差
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    /// 计算两个数的积
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    /// 安全除法，返回 Option
    pub fn divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None
        } else {
            Some(a / b)
        }
    }

    // 单元测试：放在模块内部
    #[cfg(test)]
    mod tests {
        use super::*;  // 引入父模块的所有内容

        #[test]
        fn test_add() {
            assert_eq!(add(2, 3), 5);
            assert_eq!(add(-1, 1), 0);
            assert_eq!(add(0, 0), 0);
        }

        #[test]
        fn test_subtract() {
            assert_eq!(subtract(10, 3), 7);
            assert_eq!(subtract(3, 10), -7);
        }

        #[test]
        fn test_multiply() {
            assert_eq!(multiply(3, 4), 12);
            assert_eq!(multiply(0, 100), 0);
            assert_eq!(multiply(-2, 3), -6);
        }

        #[test]
        fn test_divide() {
            assert_eq!(divide(10.0, 2.0), Some(5.0));
            assert_eq!(divide(10.0, 0.0), None);
            assert_eq!(divide(0.0, 5.0), Some(0.0));
        }
    }
}

/// 字符串工具模块
///
/// 提供常用的字符串处理功能
mod string_utils {
    /// 反转字符串
    ///
    /// # Examples
    /// ```
    /// let reversed = string_utils::reverse("hello");
    /// assert_eq!(reversed, "olleh");
    /// ```
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    /// 检查是否为回文
    pub fn is_palindrome(s: &str) -> bool {
        let s = s.to_lowercase();
        s == reverse(&s)
    }

    /// 统计单词数量
    pub fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }

    /// 将字符串转为 snake_case
    pub fn to_snake_case(s: &str) -> String {
        let mut result = String::new();
        for (i, ch) in s.chars().enumerate() {
            if ch.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_reverse() {
            assert_eq!(reverse("hello"), "olleh");
            assert_eq!(reverse("rust"), "tsur");
            assert_eq!(reverse(""), "");
        }

        #[test]
        fn test_is_palindrome() {
            assert!(is_palindrome("racecar"));
            assert!(is_palindrome("madam"));
            assert!(!is_palindrome("hello"));
        }

        #[test]
        fn test_word_count() {
            assert_eq!(word_count("hello world"), 2);
            assert_eq!(word_count("  spaces  everywhere  "), 2);
            assert_eq!(word_count(""), 0);
        }

        #[test]
        fn test_to_snake_case() {
            assert_eq!(to_snake_case("HelloWorld"), "hello_world");
            assert_eq!(to_snake_case("MyVariable"), "my_variable");
            assert_eq!(to_snake_case("ABC"), "a_b_c");
        }
    }
}

// ----- 模拟 main.rs 中的内容 -----
// 在真实项目中，main.rs 会用 use 引入 lib.rs 中的模块

fn practical_demo() {
    println!("--- 第十一部分：完整实战演示 ---");

    // 使用数学模块
    println!("\n数学运算模块：");
    println!("  2 + 3 = {}", math::add(2, 3));
    println!("  10 - 4 = {}", math::subtract(10, 4));
    println!("  3 × 4 = {}", math::multiply(3, 4));
    match math::divide(10.0, 3.0) {
        Some(result) => println!("  10 ÷ 3 = {:.4}", result),
        None => println!("  10 ÷ 3 = 错误（除零）"),
    }

    // 使用字符串工具模块
    println!("\n字符串工具模块：");
    println!("  reverse(\"Rust\") = {}", string_utils::reverse("Rust"));
    println!("  is_palindrome(\"racecar\") = {}", string_utils::is_palindrome("racecar"));
    println!("  is_palindrome(\"hello\") = {}", string_utils::is_palindrome("hello"));
    println!("  word_count(\"hello world foo bar\") = {}", string_utils::word_count("hello world foo bar"));
    println!("  to_snake_case(\"HelloWorld\") = {}", string_utils::to_snake_case("HelloWorld"));

    // 模拟集成测试的场景
    println!("\n集成测试演示：");
    // 集成测试会放在 tests/ 目录下，测试公共 API
    // 这里我们模拟一个集成测试的执行
    let a = math::add(10, 20);
    let b = math::multiply(a, 2);
    let result_str = format!("计算结果: {}", b);
    let reversed = string_utils::reverse(&result_str);
    println!("  步骤1: add(10, 20) = {}", a);
    println!("  步骤2: multiply({}, 2) = {}", a, b);
    println!("  步骤3: format → \"{}\"", result_str);
    println!("  步骤4: reverse → \"{}\"", reversed);
}

// ============================================================================
// 附录：Cargo 命令速查表
// ============================================================================
//
// 命令                          | 说明
// -----------------------------|------------------------------------------
// cargo new <name>             | 创建二进制项目
// cargo new <name> --lib       | 创建库项目
// cargo init                   | 在当前目录初始化项目
// cargo build                  | 编译（debug 模式）
// cargo build --release        | 编译（release 模式）
// cargo run                    | 编译并运行
// cargo check                  | 快速检查编译
// cargo test                   | 运行测试
// cargo test <name>            | 运行匹配名字的测试
// cargo doc                    | 生成文档
// cargo doc --open             | 生成文档并打开
// cargo fmt                    | 格式化代码
// cargo clippy                 | 静态分析
// cargo clean                  | 清理编译缓存
// cargo update                 | 更新所有依赖
// cargo update -p <crate>      | 更新指定依赖
// cargo add <crate>            | 添加依赖
// cargo add <crate> --features | 添加依赖并启用特性
// cargo remove <crate>         | 移除依赖
// cargo publish                | 发布到 crates.io
// cargo bench                  | 运行基准测试
// cargo tree                   | 显示依赖树
// ============================================================================

/*
 * 核心收获：
 * - Cargo 是 Rust 的项目管理工具，负责编译、依赖、测试、文档等一切事务
 * - Cargo.toml 是项目配置文件，定义项目元信息和依赖；Cargo.lock 锁定精确版本
 * - features 特性开关让你按需启用功能，减少不必要的编译和依赖
 *
 * 常见陷阱：
 * - 忘记在 Cargo.toml 中添加依赖就直接 use，编译时会报 "unresolved import"
 * - 库项目不小心提交了 Cargo.lock，导致和使用者的依赖版本冲突
 *
 * 下节课预告：
 * - 综合实战项目：用前 13 课学到的所有知识，从零搭建一个完整的 Rust 项目
 */

// ============================================================================
// 练习题
// ============================================================================

// 练习 1（基础巩固，5-10 分钟）：
// 描述创建一个 Cargo 项目的完整步骤
//
// 请在下方写出以下步骤（用注释）：
//   1. 用什么命令创建项目？
//   2. Cargo.toml 里需要配置什么？
//   3. 如何添加一个依赖（比如 rand）？
//   4. 如何编译并运行项目？
//   5. 如何运行测试？
//
// 你的答案：
// /*

// */
//

// ===== 测试版练习 =====
// 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
// 取消下面的注释，运行 cargo test --example lesson13

/*
#[cfg(test)]
mod tests {
    // 测试练习 2 的核心功能：基本数学函数库
    // 这些函数模拟你在 Cargo 库项目中会写的代码

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-3, -7), -10);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 3), 7);
        assert_eq!(subtract(3, 10), -7);
        assert_eq!(subtract(0, 0), 0);
    }
}
*/

// 练习 2（应用练习，15-20 分钟）：
// 写一个库项目，包含基本数学函数和测试
//
// 要求：
//   1. 创建一个库项目
//   2. 实现 add 和 subtract 函数
//   3. 为每个函数写文档注释（///）
//   4. 写至少 4 个单元测试（#[test]）
//   5. 确保 cargo test 通过
//
// 提示：在终端中执行以下命令：
//   cargo new my_math --lib
//   cd my_math
//   然后编辑 src/lib.rs
//
// 你的代码写在 Cargo 项目中，这里留一个参考实现：
fn exercise_2_reference() {
    // 参考实现（放在 lib.rs 中）：
    //
    // /// 计算两个数的和
    // ///
    // /// # Examples
    // /// ```
    // /// let result = my_math::add(2, 3);
    // /// assert_eq!(result, 5);
    // /// ```
    // pub fn add(a: i32, b: i32) -> i32 {
    //     a + b
    // }
    //
    // /// 计算两个数的差
    // pub fn subtract(a: i32, b: i32) -> i32 {
    //     a - b
    // }
    //
    // #[cfg(test)]
    // mod tests {
    //     use super::*;
    //
    //     #[test]
    //     fn test_add_positive() {
    //         assert_eq!(add(2, 3), 5);
    //     }
    //
    //     #[test]
    //     fn test_add_negative() {
    //         assert_eq!(add(-1, -2), -3);
    //     }
    //
    //     #[test]
    //     fn test_subtract() {
    //         assert_eq!(subtract(10, 3), 7);
    //     }
    //
    //     #[test]
    //     fn test_subtract_negative_result() {
    //         assert_eq!(subtract(3, 10), -7);
    //     }
    // }

    println!("练习 2 参考实现已在注释中，请在真实的 Cargo 项目中实践！");
}

// 练习 3（进阶挑战，选做）：
// 给你的库加一个可选的 logging feature
//
// 要求：
//   1. 在 Cargo.toml 中定义 features
//   2. 添加一个 logging feature
//   3. 在代码中用 #[cfg(feature = "logging")] 条件编译
//   4. 启用 feature 时，函数执行时打印日志
//   5. 不启用时，函数静默执行
//
// 提示：Cargo.toml 配置示例：
//
// [features]
// default = []
// logging = []
//
// 代码示例：
// pub fn add(a: i32, b: i32) -> i32 {
//     let result = a + b;
//     #[cfg(feature = "logging")]
//     println!("[LOG] add({}, {}) = {}", a, b, result);
//     result
// }
//
// 运行方式：
//   cargo test                        → 不启用 logging
//   cargo test --features logging     → 启用 logging

fn exercise_3_hint() {
    println!("练习 3 提示：");
    println!("  1. 在 Cargo.toml 的 [features] 下添加：logging = []");
    println!("  2. 在函数中用 #[cfg(feature = \"logging\")] 条件编译");
    println!("  3. cargo test --features logging 启用特性");
}

// ============================================================================
// 附录：完整的 Cargo 项目示例（复制粘贴即可运行）
// ============================================================================
//
// 第1步：在终端中执行
//   cargo new my_math --lib
//   cd my_math
//
// 第2步：用以下内容替换 src/lib.rs
//
// ---- src/lib.rs ----
// /// 计算两个数的和
// pub fn add(a: i32, b: i32) -> i32 { a + b }
//
// /// 计算两个数的差
// pub fn subtract(a: i32, b: i32) -> i32 { a - b }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_add() { assert_eq!(add(2, 3), 5); }
//     #[test]
//     fn test_subtract() { assert_eq!(subtract(10, 3), 7); }
// }
//
// ---- src/main.rs ----（新建这个文件）
// use my_math::{add, subtract};
// fn main() {
//     println!("2 + 3 = {}", add(2, 3));
//     println!("10 - 3 = {}", subtract(10, 3));
// }
//
// 第3步：执行
//   cargo test          ← 运行测试
//   cargo run           ← 运行程序
//   cargo doc --open    ← 生成并打开文档

// ============================================================================
// 附录 B：完整的多文件 Cargo 项目示例
// ============================================================================
//
// 这个项目演示了真实的 Cargo 项目结构：
// - src/main.rs：程序入口
// - src/lib.rs：库入口
// - src/models.rs：数据模型
// - src/utils.rs：工具函数
// - tests/integration_test.rs：集成测试
//
// 请按以下步骤操作：
//
// 第1步：创建项目
//   cargo new student_manager
//   cd student_manager
//
// 第2步：创建文件结构
//   student_manager/
//   ├── Cargo.toml
//   ├── src/
//   │   ├── main.rs
//   │   ├── lib.rs
//   │   ├── models.rs
//   │   └── utils.rs
//   └── tests/
//       └── integration_test.rs
//
// 第3步：用以下内容替换各文件
//
// ---- src/models.rs ----
// /// 学生结构体
// #[derive(Debug, Clone)]
// pub struct Student {
//     pub name: String,
//     pub age: u32,
//     pub scores: Vec<f64>,
// }
//
// impl Student {
//     pub fn new(name: &str, age: u32) -> Self {
//         Student {
//             name: name.to_string(),
//             age,
//             scores: Vec::new(),
//         }
//     }
//
//     pub fn add_score(&mut self, score: f64) {
//         self.scores.push(score);
//     }
//
//     pub fn average(&self) -> f64 {
//         if self.scores.is_empty() {
//             return 0.0;
//         }
//         self.scores.iter().sum::<f64>() / self.scores.len() as f64
//     }
//
//     pub fn is_passed(&self) -> bool {
//         self.average() >= 60.0
//     }
// }
//
// ---- src/utils.rs ----
// use crate::models::Student;
//
// /// 按平均分排序（降序）
// pub fn sort_by_average(students: &mut Vec<Student>) {
//     students.sort_by(|a, b| {
//         b.average().partial_cmp(&a.average()).unwrap()
//     });
// }
//
// /// 过滤及格学生
// pub fn filter_passed(students: &[Student]) -> Vec<&Student> {
//     students.iter().filter(|s| s.is_passed()).collect()
// }
//
// /// 计算全班平均分
// pub fn class_average(students: &[Student]) -> f64 {
//     if students.is_empty() {
//         return 0.0;
//     }
//     let total: f64 = students.iter().map(|s| s.average()).sum();
//     total / students.len() as f64
// }
//
// ---- src/lib.rs ----
// pub mod models;
// pub mod utils;
//
// ---- src/main.rs ----
// use student_manager::models::Student;
// use student_manager::utils::{sort_by_average, filter_passed, class_average};
//
// fn main() {
//     let mut students = vec![
//         Student::new("小明", 18),
//         Student::new("小红", 19),
//         Student::new("小刚", 18),
//     ];
//
//     students[0].add_score(85.0);
//     students[0].add_score(92.0);
//     students[1].add_score(58.0);
//     students[1].add_score(63.0);
//     students[2].add_score(95.0);
//     students[2].add_score(88.0);
//
//     println!("=== 学生成绩管理 ===\n");
//     for s in &students {
//         println!("{}: 平均分 {:.1}, {}", s.name, s.average(),
//             if s.is_passed() { "及格" } else { "不及格" });
//     }
//
//     sort_by_average(&mut students);
//     println!("\n按平均分排名:");
//     for (i, s) in students.iter().enumerate() {
//         println!("  {}. {} ({:.1})", i + 1, s.name, s.average());
//     }
//
//     let passed = filter_passed(&students);
//     println!("\n及格人数: {}/{}", passed.len(), students.len());
//     println!("全班平均分: {:.1}", class_average(&students));
// }
//
// ---- tests/integration_test.rs ----
// use student_manager::models::Student;
// use student_manager::utils::{sort_by_average, filter_passed, class_average};
//
// #[test]
// fn test_student_average() {
//     let mut s = Student::new("测试", 18);
//     s.add_score(80.0);
//     s.add_score(90.0);
//     assert_eq!(s.average(), 85.0);
// }
//
// #[test]
// fn test_student_passed() {
//     let mut s = Student::new("测试", 18);
//     s.add_score(50.0);
//     s.add_score(70.0);
//     assert!(s.is_passed()); // 平均分 60，刚好及格
// }
//
// #[test]
// fn test_sort_by_average() {
//     let mut students = vec![
//         Student::new("A", 18),
//         Student::new("B", 18),
//     ];
//     students[0].add_score(60.0);
//     students[1].add_score(90.0);
//     sort_by_average(&mut students);
//     assert_eq!(students[0].name, "B");
// }
//
// #[test]
// fn test_class_average() {
//     let mut students = vec![
//         Student::new("A", 18),
//         Student::new("B", 18),
//     ];
//     students[0].add_score(80.0);
//     students[1].add_score(100.0);
//     assert_eq!(class_average(&students), 90.0);
// }
//
// 第4步：运行
//   cargo test            ← 运行所有测试（单元 + 集成）
//   cargo run             ← 运行程序
//   cargo doc --open      ← 生成文档
//
// 注意：单元测试写在各模块文件中（src/models.rs、src/utils.rs），
//       集成测试写在 tests/ 目录下。
//       cargo test 会自动运行所有测试。
