// -*- coding: utf-8 -*-
// =============================================
// G-one 的 Rust 课堂 第07课：错误处理
// =============================================
// 同学们好！欢迎来到第七课！
//
// 上节课我们学了枚举和模式匹配，这节课我们要学习"错误处理"。
//
// 【生活类比：开车与错误处理】
// 想象你正在开车去一个地方：
// - 正常情况：你顺利到达目的地（程序正常运行）
// - 意外情况：爆胎了、没油了、迷路了（程序遇到错误）
//
// Rust 的错误处理就像一个经验丰富的司机：
// - 他知道可能会出什么问题
// - 他会提前准备应对方案
// - 他不会假装问题不存在
//
// 其他语言（比如 Java、Python）用"异常"（exception）处理错误，
// 但 Rust 不用异常！Rust 用两种方式：
// 1. panic! —— 车祸现场，程序直接挂掉（不可恢复）
// 2. Result<T, E> —— 路况报告，告诉你"这条路通"或"这条路封了"（可恢复）
//
// 我会像教小朋友一样，一步一步带你学习。
// 准备好了吗？让我们开始吧！

// 引入需要的标准库
use std::fs;       // 文件系统操作
use std::io;       // 输入输出操作
use std::num::ParseIntError;  // 数字解析错误
use std::fmt;       // 格式化输出

fn main() {
    println!("============================================");
    println!("第07课：错误处理");
    println!("============================================\n");

    demo_panic();
    demo_result();
    demo_question_mark();
    demo_custom_error();
    demo_unwrap_expect();
    demo_combinators();
    project_config_parser();
    demo_exercises();

    // =============================================
    // 补充：生态里的错误处理 crate
    // =============================================

    /*
     * 【thiserror 和 anyhow】
     * 在实际项目中，大家经常用这两个 crate 来简化错误处理：
     *
     * thiserror：
     * - 用宏自动实现 Display 和 From
     * - 适合写库（library）
     * - 例如：
     *   #[derive(thiserror::Error)]
     *   enum AppError {
     *       #[error("文件不存在: {0}")]
     *       FileNotFound(String),
     *       #[error("解析错误: {0}")]
     *       ParseError(#[from] std::num::ParseIntError),
     *   }
     *
     * anyhow：
     * - 提供 anyhow::Result<T>，自动处理各种错误类型
     * - 适合写应用程序（application）
     * - 例如：
     *   fn read_config() -> anyhow::Result<Config> {
     *       let content = fs::read_to_string("config.txt")?;
     *       // ...
     *   }
     *
     * 今天我们手动实现是为了理解原理，
     * 以后实际开发时可以用这些库来简化代码。
     */

    println!("\n--- 补充：生态里的错误处理 crate ---\n");
    println!("thiserror：用宏自动实现 Display 和 From，适合写库");
    println!("anyhow：提供统一的 Result 类型，适合写应用");
    println!("今天我们手动实现是为了理解原理！");

    // =============================================
    // 【教授的话】
    // =============================================

    println!("\n==========================================");
    println!("课程总结");
    println!("==========================================");

    /*
     * 恭喜你完成了第七课！
     *
     * 你已经学会了：
     * ✓ 两种错误（panic! vs Result）
     * ✓ panic! 宏（不可恢复错误）
     * ✓ Result<T, E>（可恢复错误）
     * ✓ match 处理 Result
     * ✓ ? 运算符（错误传播的语法糖）
     * ✓ 自定义错误类型（enum AppError）
     * ✓ Display trait（让错误信息友好）
     * ✓ From trait（自动转换错误类型）
     * ✓ unwrap/expect（快捷方式，谨慎使用！）
     * ✓ 组合子（map/unwrap_or/unwrap_or_else）
     *
     * 这些都是让程序更健壮的重要工具！
     *
     * 记住 Rust 的错误处理哲学：
     * 1. 显式优于隐式 —— 错误必须被处理，不能忽略
     * 2. 编译时检查 —— 忘记处理错误会编译失败
     * 3. 类型安全 —— 错误类型明确，不会混淆
     * 4. 可组合 —— ? 运算符让错误传播优雅简洁
     *
     * 下一课我们会学习泛型和 trait，Rust 的抽象机制。
     * 继续加油，你已经很棒了！
     */
}

// =============================================
// demo 函数
// =============================================

/// demo_panic() —— panic! 宏演示
///
/// 【生活类比】
/// panic! 就像车祸：程序直接挂掉，无法继续运行
/// 用于"不可能发生"的情况，或者 bug
fn demo_panic() {
    println!("--- 第一节：panic! 宏 ---\n");

    // panic! 会让程序立即崩溃
    // 取消下面这行的注释，运行时会看到程序崩溃：
    // panic!("哎呀，出车祸了！程序崩溃了！");

    // 常见的隐式 panic：数组越界
    // 就像你要拿第 10 个抽屉，但只有 3 个抽屉
    let fruits = vec!["苹果", "香蕉", "橙子"];
    println!("水果篮里有 {} 个水果", fruits.len());
    println!("第一个水果：{}", fruits[0]);

    // 取消下面这行的注释，会触发 panic（数组越界）：
    // println!("第10个水果：{}", fruits[9]);  // 索引 9 超出范围，panic!

    // 【小知识】
    // 设置环境变量 RUST_BACKTRACE=1 可以看到详细的调用栈
    // 帮助你找到 panic 发生的位置
}

/// demo_result() —— Result<T, E> 可恢复错误演示
///
/// 【生活类比】
/// Result 就像路况报告：
/// - Ok(value) = 路况正常，可以通行，返回目的地
/// - Err(error) = 路况异常，返回错误原因
fn demo_result() {
    // =============================================
    // 第二节：Result<T, E> —— 可恢复的错误
    // =============================================

    /*
     * 【代码结构】
     * enum Result<T, E> {
     *     Ok(T),   // 成功，包含返回值
     *     Err(E),  // 失败，包含错误信息
     * }
     *
     * T = 成功时返回的类型
     * E = 失败时返回的错误类型
     */

    println!("\n--- 第二节：Result<T, E> ---\n");

    // 示例：读取文件（可能失败的操作）
    // read_file_greeting 调用了辅助函数 read_file_greeting
    let result = read_file_greeting("hello.txt");
    match result {
        Ok(content) => println!("文件内容：{}", content),
        Err(error) => println!("读取失败：{}", error),
    }

    // =============================================
    // 第三节：match 处理 Result —— 像 GPS 一样决策
    // =============================================

    println!("\n--- 第三节：用 match 处理错误 ---\n");

    // 尝试解析数字（可能失败）
    // parse_number 调用了辅助函数 parse_number
    let numbers = vec!["42", "abc", "100", "xyz", "0"];
    for num_str in numbers {
        match parse_number(num_str) {
            Ok(n) => println!("'{}' 解析成功：{}", num_str, n),
            Err(e) => println!("'{}' 解析失败：{}", num_str, e),
        }
    }
}

/// demo_question_mark() —— ? 运算符演示
///
/// 【生活类比】
/// ? 运算符就像 GPS 的自动绕路功能：
/// - 如果这条路通（Ok），继续走
/// - 如果这条路封（Err），自动返回错误，不用你手动处理
///
/// 【语法糖】
/// let value = some_function()?;  // 等价于下面的代码
/// let value = match some_function() {
///     Ok(v) => v,
///     Err(e) => return Err(e),  // 自动返回错误
/// };
///
/// 注意：? 只能用在返回 Result 的函数中！
fn demo_question_mark() {
    println!("\n--- 第四节：? 运算符 ---\n");

    // ? 运算符让错误传播变得简洁
    // read_and_parse 调用了辅助函数 read_and_parse（内部使用 ? 运算符）
    match read_and_parse() {
        Ok(n) => println!("读取并解析成功：{}", n),
        Err(e) => println!("读取或解析失败：{}", e),
    }
}

/// demo_custom_error() —— 自定义错误类型 + Display/From trait 演示
///
/// 【生活类比】
/// 自定义错误类型就像设计自己的"错误报告表"：
/// - 文件不存在 → "文件错误"
/// - 格式不对 → "格式错误"
/// - 端口号无效 → "参数错误"
///
/// Display trait = 让错误信息对人类友好
/// From trait = 让错误类型可以自动转换（就像翻译官）
fn demo_custom_error() {
    // =============================================
    // 第五节：自定义错误类型
    // =============================================

    println!("\n--- 第五节：自定义错误类型 ---\n");

    // 使用自定义的 AppError
    // parse_and_double 调用了辅助函数 parse_and_double（内部用 ? 自动转换错误）
    let test_cases = vec!["42", "abc", "100", "xyz"];
    for input in test_cases {
        match parse_and_double(input) {
            Ok(result) => println!("'{}' 的两倍是 {}", input, result),
            Err(e) => println!("解析 '{}' 失败：{}", input, e),
        }
    }

    // =============================================
    // 第六节：Display 和 From trait —— 让错误更友好
    // =============================================

    println!("\n--- 第六节：Display 和 From trait ---\n");

    // Display trait 让我们可以用 {} 打印错误
    let error = AppError::Parse("abc".to_string());
    println!("Display 错误：{}", error);  // 会调用 Display 的实现

    // From trait 让 ParseIntError 自动转换成 AppError
    // 这样在 parse_and_double 函数中，? 运算符就能自动转换错误类型
    println!("From trait 让错误类型自动转换成为可能！");
}

/// demo_unwrap_expect() —— unwrap/expect 演示
///
/// 【生活类比】
/// unwrap/expect 就像闭着眼睛开车：
/// - 如果没出事，很快就到了
/// - 如果出事了，直接车祸（panic!）
///
/// 【什么时候用？】
/// - 你 100% 确定不会失败时（比如硬编码的数字）
/// - 原型开发时，快速验证想法
/// - 测试代码中
///
/// 【什么时候不能用？】
/// - 处理用户输入时、读取文件时、任何可能失败的操作
fn demo_unwrap_expect() {
    println!("\n--- 第七节：unwrap 和 expect ---\n");

    // unwrap：成功返回值，失败 panic
    let s = "42";
    let n: i32 = s.parse().unwrap();  // 这里确定不会失败
    println!("unwrap 结果：{}", n);

    // expect：类似 unwrap，但可以自定义错误信息
    let n: i32 = s.parse().expect("这不应该发生：解析数字失败");
    println!("expect 结果：{}", n);

    // 如果用 unwrap 处理可能失败的操作，会 panic：
    // let n: i32 = "abc".parse().unwrap();  // 这行会 panic!

    // 安全的做法是用 match 或 ? 运算符
    match "abc".parse::<i32>() {
        Ok(n) => println!("解析成功：{}", n),
        Err(e) => println!("解析失败（安全处理）：{}", e),
    }
}

/// demo_combinators() —— 组合子演示（map/unwrap_or/unwrap_or_else）
///
/// 【生活类比】
/// 组合子就像流水线上的加工步骤：
/// - map：对成功的结果进行加工
/// - unwrap_or：失败时提供默认值
/// - unwrap_or_else：失败时计算默认值
fn demo_combinators() {
    println!("\n--- 第八节：组合子 ---\n");

    // map：转换 Ok 中的值
    // 就像：如果路通，就把目的地改成"加油站"
    let s = "42";
    let n: Result<i32, _> = s.parse();
    let doubled = n.map(|x| x * 2);  // 如果成功，结果乘以 2
    println!("map 结果：{:?}", doubled);  // Ok(84)

    let s = "abc";
    let n: Result<i32, _> = s.parse();
    let doubled = n.map(|x| x * 2);  // 如果失败，保持 Err
    println!("map 失败：{:?}", doubled);  // Err(...)

    // unwrap_or：提供默认值
    // 就像：如果路封了，就去附近的咖啡店
    let n: i32 = "abc".parse().unwrap_or(0);  // 解析失败，默认值是 0
    println!("unwrap_or 结果：{}", n);  // 0

    let n: i32 = "42".parse().unwrap_or(0);  // 解析成功，使用解析结果
    println!("unwrap_or 结果：{}", n);  // 42

    // unwrap_or_else：用闭包计算默认值
    // 就像：如果路封了，让 GPS 重新规划路线
    let n: i32 = "abc".parse().unwrap_or_else(|e| {
        println!("解析失败：{}，使用默认值", e);
        -1  // 返回默认值
    });
    println!("unwrap_or_else 结果：{}", n);  // -1
}

/// project_config_parser() —— 实战项目：配置文件解析器
///
/// 【生活类比】
/// 配置文件解析器就像读取"旅行计划"：
/// - 文件里写着：目的地、出发时间、是否带伞
/// - 我们需要读取、解析、验证这些信息
/// - 如果信息有误，要告诉用户哪里错了
fn project_config_parser() {
    println!("\n--- 第九节：实战：配置文件解析器 ---\n");

    // 创建一个临时配置文件用于测试
    let config_content = "localhost:8080:true";
    match fs::write("test_config.txt", config_content) {
        Ok(_) => println!("创建测试配置文件成功"),
        Err(e) => println!("创建配置文件失败：{}", e),
    }

    // 测试1：正常配置
    // parse_config 调用了辅助函数 parse_config（内部使用 AppError + ? 运算符）
    println!("\n测试1：正常配置文件");
    match parse_config("test_config.txt") {
        Ok(config) => println!("配置解析成功：{:?}", config),
        Err(e) => println!("配置解析失败：{}", e),
    }

    // 测试2：格式错误的配置
    println!("\n测试2：格式错误的配置文件");
    fs::write("bad_config.txt", "localhost:8080").unwrap();  // 缺少 debug 字段
    match parse_config("bad_config.txt") {
        Ok(config) => println!("配置解析成功：{:?}", config),
        Err(e) => println!("配置解析失败：{}", e),
    }

    // 测试3：端口号无效
    println!("\n测试3：端口号无效的配置文件");
    fs::write("bad_port.txt", "localhost:abc:true").unwrap();  // 端口号不是数字
    match parse_config("bad_port.txt") {
        Ok(config) => println!("配置解析成功：{:?}", config),
        Err(e) => println!("配置解析失败：{}", e),
    }

    // 测试4：debug 值无效
    println!("\n测试4：debug 值无效的配置文件");
    fs::write("bad_debug.txt", "localhost:8080:yes").unwrap();  // debug 不是 true/false
    match parse_config("bad_debug.txt") {
        Ok(config) => println!("配置解析成功：{:?}", config),
        Err(e) => println!("配置解析失败：{}", e),
    }

    // 测试5：文件不存在
    println!("\n测试5：文件不存在");
    match parse_config("nonexistent.txt") {
        Ok(config) => println!("配置解析成功：{:?}", config),
        Err(e) => println!("配置解析失败：{}", e),
    }

    // 清理测试文件
    let _ = fs::remove_file("test_config.txt");
    let _ = fs::remove_file("bad_config.txt");
    let _ = fs::remove_file("bad_port.txt");
    let _ = fs::remove_file("bad_debug.txt");
}

/// demo_exercises() —— 练习题演示
///
/// 调用 safe_divide、validate_email、parse_and_double_chain 三个辅助函数
fn demo_exercises() {
    println!("\n==========================================");
    println!("练习题参考答案");
    println!("==========================================\n");

    // 练习1：安全除法
    println!("--- 练习1：安全除法 ---");
    println!("10 / 3 = {:?}", safe_divide(10.0, 3.0));
    println!("10 / 0 = {:?}", safe_divide(10.0, 0.0));
    println!("100 / 5 = {:?}", safe_divide(100.0, 5.0));

    // 练习2：邮箱验证器
    println!("\n--- 练习2：邮箱验证器 ---");
    let emails = vec![
        "test@example.com",      // 有效
        "user@domain.org",       // 有效
        "invalid",               // 无效：没有 @
        "missing@dotcom",        // 无效：没有 .
        "@domain.com",           // 无效：没有用户名
        "user@.com",             // 无效：域名以 . 开头
    ];
    for email in emails {
        match validate_email(email) {
            Ok(()) => println!("'{}' 有效", email),
            Err(e) => println!("'{}' 无效：{}", email, e),
        }
    }

    // 练习3：链式解析器
    println!("\n--- 练习3：链式解析器 ---");
    let inputs = vec!["42", "abc", "100", "xyz", "25"];
    for input in inputs {
        match parse_and_double_chain(input) {
            Ok(result) => println!("'{}' 的两倍是 {}", input, result),
            Err(e) => println!("解析 '{}' 失败：{}", input, e),
        }
    }
}

// =============================================
// 辅助函数
// =============================================

// 读取文件（演示 Result）
fn read_file_greeting(path: &str) -> Result<String, io::Error> {
    // fs::read_to_string 返回 Result<String, io::Error>
    // 如果文件存在，返回 Ok(文件内容)
    // 如果文件不存在，返回 Err(错误信息)
    fs::read_to_string(path)
}

// 解析数字（演示错误处理）
fn parse_number(s: &str) -> Result<i32, String> {
    // s.parse() 返回 Result<i32, ParseIntError>
    // 我们用 map_err 把 ParseIntError 转换成 String
    s.parse::<i32>().map_err(|e| format!("'{}' 不是有效数字：{}", s, e))
}

// 读取并解析（演示 ? 运算符）
fn read_and_parse() -> Result<i32, io::Error> {
    // ? 运算符：如果 Ok，取出值；如果 Err，立即返回错误
    let content = fs::read_to_string("number.txt")?;  // 文件不存在会返回 Err
    let number: i32 = content.trim().parse().unwrap_or(0);  // 解析失败默认为 0
    Ok(number)
}

// =============================================
// 自定义错误类型
// =============================================

// 定义自己的错误枚举
#[derive(Debug)]
enum AppError {
    // 每个变体代表一种错误类型
    Io(io::Error),           // 文件/网络错误
    Parse(ParseIntError),    // 数字解析错误
    Validation(String),      // 验证错误（自定义信息）
}

// 实现 Display trait，让错误可以用 {} 打印
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO 错误：{}", e),
            AppError::Parse(e) => write!(f, "解析错误：{}", e),
            AppError::Validation(msg) => write!(f, "验证错误：{}", msg),
        }
    }
}

// 实现 From trait，让 io::Error 自动转换成 AppError
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

// 实现 From trait，让 ParseIntError 自动转换成 AppError
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

// 使用自定义错误类型和 ? 运算符
fn parse_and_double(input: &str) -> Result<i32, AppError> {
    // ? 运算符会自动调用 From trait 进行错误类型转换
    // ParseIntError → AppError::Parse
    let n: i32 = input.parse()?;
    Ok(n * 2)
}

// ===== 测试版练习 =====
// 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
// 取消下面的注释，运行 cargo test --example 07_错误处理

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise1_safe_divide() {
        // 测试练习 1 的核心功能：安全除法
        assert_eq!(safe_divide(10.0, 3.0), Ok(10.0 / 3.0));
        assert_eq!(safe_divide(10.0, 0.0), Err("除数不能为零".to_string()));
        assert_eq!(safe_divide(100.0, 5.0), Ok(20.0));
        assert_eq!(safe_divide(-9.0, 3.0), Ok(-3.0));
    }
}
*/

// =============================================
// 练习题参考答案
// =============================================

// 练习1：安全除法
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

// 练习2：邮箱验证器
fn validate_email(email: &str) -> Result<(), String> {
    // 检查是否包含 @
    if !email.contains('@') {
        return Err("邮箱必须包含 @ 符号".to_string());
    }

    // 检查是否包含 .
    if !email.contains('.') {
        return Err("邮箱必须包含 . 符号".to_string());
    }

    // 分割 @ 前后部分
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err("邮箱格式不正确：应该只有一个 @".to_string());
    }

    // 检查用户名和域名是否为空
    if parts[0].is_empty() {
        return Err("邮箱格式不正确：缺少用户名".to_string());
    }
    if parts[1].is_empty() {
        return Err("邮箱格式不正确：缺少域名".to_string());
    }

    // 检查域名是否包含 .
    if !parts[1].contains('.') {
        return Err("邮箱格式不正确：域名应该包含 .".to_string());
    }

    Ok(())  // 验证通过
}

// 练习3：链式解析器（用 ? 自动转换错误类型）
fn parse_and_double_chain(input: &str) -> Result<i32, AppError> {
    // ? 运算符自动把 ParseIntError 转换成 AppError
    let n: i32 = input.parse()?;

    // 检查数值范围
    if n > 1000 {
        return Err(AppError::Validation(format!("数值 {} 超过上限 1000", n)));
    }

    Ok(n * 2)
}

// =============================================
// 实战项目：配置文件解析器
// =============================================

// 配置结构体
#[derive(Debug)]
struct Config {
    host: String,   // 主机地址
    port: u16,      // 端口号
    debug: bool,    // 是否开启调试模式
}

// 解析配置文件（演示完整的错误处理流程）
fn parse_config(path: &str) -> Result<Config, AppError> {
    // 1. 读取文件（可能失败：文件不存在）
    //    ? 运算符自动把 io::Error 转换成 AppError::Io
    let content = fs::read_to_string(path)?;

    // 2. 解析格式（应该有 3 个部分，用 : 分隔）
    let parts: Vec<&str> = content.trim().split(':').collect();
    if parts.len() != 3 {
        return Err(AppError::Validation(
            format!("配置格式错误，期望 host:port:debug，实际得到：'{}'", content)
        ));
    }

    // 3. 解析主机地址
    let host = parts[0].to_string();
    if host.is_empty() {
        return Err(AppError::Validation("主机地址不能为空".to_string()));
    }

    // 4. 解析端口号（可能失败：不是数字）
    //    ? 运算符自动把 ParseIntError 转换成 AppError::Parse
    let port: u16 = parts[1].parse()?;

    // 5. 解析 debug 标志（必须是 true 或 false）
    let debug = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err(AppError::Validation(
            format!("debug 字段必须是 'true' 或 'false'，实际得到：'{}'", parts[2])
        )),
    };

    // 6. 返回成功的结果
    Ok(Config { host, port, debug })
}

/*
 * 核心收获：
 * - Rust 用 panic!（不可恢复）和 Result<T, E>（可恢复）两种方式处理错误
 * - ? 运算符是错误传播的语法糖，让代码更简洁
 * - 自定义错误类型 + Display + From 可以实现优雅的错误处理
 *
 * 常见陷阱：
 * - 滥用 unwrap/expect：在可能失败的操作上使用会导致程序 panic
 * - 忘记处理错误：Rust 编译器会强制你处理 Result，这是好事，不要绕过它
 *
 * 下节课预告：
 * - 下节课学泛型和 Trait，Rust 的抽象机制，让你的代码更通用、更强大！
 */
