// ============================================================================
// 第 14 课：综合实战 —— 命令行记事本
// ============================================================================
//
// ⚠️ 注意：本课包含交互式输入（stdin），请在终端中运行：
//   cargo run --example lesson14
// 不要在 IDE 的"Run"按钮中运行，否则可能无法输入。
//
// 【写给零基础的你】
//
// 恭喜你走到了第 14 课！从第 0 课的"Hello World"到现在，
// 你已经学了变量、函数、结构体、枚举、所有权、生命周期、
// 错误处理、Trait、闭包、迭代器、模块……一大堆知识点。
//
// 这节课，我们要把它们全部串起来，做一个真正的小项目：
//   一个「命令行记事本」（Mini Notebook）
//
// 想象一下：你打开电脑，敲几行命令，就能新建笔记、查看笔记、
// 搜索笔记、删除笔记，还能把所有笔记保存成文件，下次打开继续用。
//
// 这就像你小时候用的那个小本子：
//   - 想记东西？翻到新一页，写下来（添加笔记）
//   - 想看看之前写了啥？一页一页翻（列出笔记）
//   - 找某个关键词？前后翻着找（搜索笔记）
//   - 写错了？撕掉那一页（删除笔记）
//   - 本子用完了？放书架上（保存到文件）
//   - 第二天？从书架上拿出来继续用（从文件加载）
//
// 本节课目标：
//   1. 用结构体定义 Note（笔记）和 Notebook（记事本）
//   2. 用枚举定义 Command（用户输入的命令）
//   3. 用所有权和借用管理笔记数据
//   4. 用自定义错误类型处理各种异常情况
//   5. 用闭包和迭代器优雅地搜索、过滤笔记
//   6. 用模块思想把代码组织得井井有条
//   7. 用文件 I/O 实现笔记的持久化存储
//   8. 用 loop + match 实现交互式命令循环（REPL）
//
// 准备好了吗？这是你第一个完整的 Rust 小项目！
//

// ============================================================================
// 第一部分：定义数据结构 —— Note、Notebook、Command
// ============================================================================
//
// 【类比时间】
//
// 一个记事本里有什么？
//   - 每一页笔记（Note）：有标题、内容、写的时间
//   - 整个本子（Notebook）：包含很多页笔记
//   - 你想做的事（Command）：添加、查看、搜索、删除、保存、加载
//
// 这就像搭积木，先做好每一块积木（数据结构），再用它们拼出完整的房子。
//

// ---------- 第一小节：Note 结构体 ----------

use std::fmt;           // 用于实现 Display trait
use std::fs;            // 用于文件读写
use std::io;            // 用于读取用户输入
use std::collections::HashMap; // 用于按标签分组（练习题用）

/// Note 结构体：表示一条笔记
///
/// 【字段说明】
///   - title：笔记的标题（比如"买菜清单"）
///   - content：笔记的内容（比如"鸡蛋、牛奶、面包"）
///   - created_at：创建时间（比如"2026-06-13 10:30"）
///   - tags：标签列表（比如["购物", "日常"]）
///
/// 【所有权说明】
///   - 这里用 String 而不是 &str，因为 Note 要"拥有"自己的数据
///   - 如果用 &str，就需要考虑生命周期，会变得很复杂
///   - String 是堆上分配的，Note 被销毁时，String 也会自动释放
///
#[derive(Debug, Clone)] // Debug 让我们可以用 {:?} 打印，Clone 让我们可以复制
struct Note {
    title: String,       // 标题：拥有所有权的字符串
    content: String,     // 内容：拥有所有权的字符串
    created_at: String,  // 创建时间：拥有所有权的字符串
    tags: Vec<String>,   // 标签列表：拥有所有权的字符串数组
}

/// 为 Note 实现 Display trait
///
/// 【为什么要实现 Display？】
///   - 这样我们就能用 println!("{}", note) 直接打印笔记
///   - 而不是只能用 println!("{:?}", note) 打印调试信息
///   - Display 是"给人看的"，Debug 是"给程序员看的"
///
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // \x1b[1;33m 是黄色加粗，\x1b[0m 是重置颜色
        // 这样标题会更醒目
        write!(
            f,
            "\x1b[1;33m[{}]\x1b[0m\n  内容：{}\n  时间：{}\n  标签：{}",
            self.title,
            self.content,
            self.created_at,
            if self.tags.is_empty() {
                "无".to_string()
            } else {
                self.tags.join(", ")
            }
        )
    }
}

/// 为 Note 实现一些实用方法
///
/// 【impl 块的作用】
///   - 在 impl 块里定义的函数叫做"方法"
///   - 方法的第一个参数是 &self（表示"我自己的数据"）
///   - 就像人会走路、说话，Note 也能做一些事情
///
impl Note {
    /// 创建一条新笔记
    ///
    /// 【参数说明】
    ///   - title: 笔记标题（传入的是 &str，函数内部会转成 String）
    ///   - content: 笔记内容
    ///   - tags: 标签列表
    ///
    /// 【为什么返回 Note 而不是 &Note？】
    ///   - 因为 Note 是新建的，函数结束后它还要继续存在
    ///   - 如果返回引用，函数结束后数据就没了（悬垂引用）
    ///   - 所以必须返回拥有所有权的 Note
    ///
    fn new(title: &str, content: &str, tags: Vec<String>) -> Self {
        Note {
            // .to_string() 把 &str 转成 String（在堆上分配内存）
            title: title.to_string(),
            content: content.to_string(),
            // 获取当前时间的简化版本（实际项目中会用 chrono 库）
            created_at: get_current_time(),
            tags,
        }
    }

    /// 检查笔记是否包含某个关键词
    ///
    /// 【闭包的使用】
    ///   - keyword 是从外部捕获的变量（不可变借用）
    ///   - 我们用闭包来检查标题或内容是否包含关键词
    ///   - to_lowercase() 让搜索不区分大小写
    ///
    fn contains(&self, keyword: &str) -> bool {
        let keyword_lower = keyword.to_lowercase();
        // 检查标题或内容中是否包含关键词（不区分大小写）
        self.title.to_lowercase().contains(&keyword_lower)
            || self.content.to_lowercase().contains(&keyword_lower)
            // 迭代器 + any：只要有一个标签包含关键词就返回 true
            || self.tags.iter().any(|tag| tag.to_lowercase().contains(&keyword_lower))
    }

    /// 把笔记转成 JSON 字符串（手动序列化，不依赖 serde）
    ///
    /// 【为什么手动序列化？】
    ///   - 为了不引入外部库，保持课程简单
    ///   - 实际项目中推荐使用 serde + serde_json
    ///   - 这里我们手动拼接 JSON 字符串
    ///
    fn to_json(&self) -> String {
        // 把标签列表转成 JSON 数组格式
        // 例如：["购物", "日常"] -> "\"购物\",\"日常\""
        let tags_json: String = self.tags
            .iter()
            .map(|tag| format!("\"{}\"", escape_json(tag)))
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "{{\"title\":\"{}\",\"content\":\"{}\",\"created_at\":\"{}\",\"tags\":[{}]}}",
            escape_json(&self.title),
            escape_json(&self.content),
            escape_json(&self.created_at),
            tags_json
        )
    }

    /// 从 JSON 字符串解析出一条笔记（手动反序列化）
    ///
    /// 【解析的思路】
    ///   - 找到每个字段的键（如 "title":）
    ///   - 提取对应的值
    ///   - 这是一个简化版本，实际项目中应该用 serde
    ///
    fn from_json(json: &str) -> Result<Self, AppError> {
        let title = extract_json_string(json, "title")?;
        let content = extract_json_string(json, "content")?;
        let created_at = extract_json_string(json, "created_at")?;
        let tags = extract_json_array(json, "tags")?;

        Ok(Note {
            title,
            content,
            created_at,
            tags,
        })
    }
}

// ---------- 第二小节：Notebook 结构体 ----------

/// Notebook 结构体：表示整个记事本
///
/// 【字段说明】
///   - notes：一个 Vec<Note>，存储所有的笔记
///
/// 【Vec 的比喻】
///   - Vec 就像一个"可变长的书架"
///   - 你可以往上面放书（push），也可以拿走书（remove）
///   - 书架可以变长变短，非常灵活
///
struct Notebook {
    notes: Vec<Note>, // 存储所有笔记的动态数组
}

/// 为 Notebook 实现方法
///
impl Notebook {
    /// 创建一个空的记事本
    fn new() -> Self {
        Notebook {
            notes: Vec::new(), // 空数组，还没有任何笔记
        }
    }

    /// 添加一条笔记
    ///
    /// 【所有权转移】
    ///   - note 的所有权从调用者转移到了 self.notes 里
    ///   - 调用 add_note 之后，原来的 note 变量就不能再用了
    ///   - 这就是 Rust 的"所有权转移"——数据只有一个主人
    ///
    fn add_note(&mut self, note: Note) {
        // push 会把 note 的所有权转移到 Vec 里
        self.notes.push(note);
        println!("\x1b[32m✓ 笔记添加成功！\x1b[0m");
    }

    /// 列出所有笔记
    ///
    /// 【借用的使用】
    ///   - &self 表示"不可变借用"——我只是看看，不会修改
    ///   - 就像你去图书馆看书，只能看，不能在上面涂画
    ///
    fn list_notes(&self) {
        if self.notes.is_empty() {
            println!("\x1b[33m📝 记事本是空的，还没有任何笔记哦！\x1b[0m");
            return;
        }

        println!("\x1b[1;36m=== 所有笔记（共 {} 条）===\x1b[0m", self.notes.len());

        // 迭代器 + enumerate：遍历每条笔记，同时获取索引
        // enumerate() 返回 (index, &note) 的元组
        for (i, note) in self.notes.iter().enumerate() {
            println!("\n\x1b[1;34m--- 第 {} 条 ---\x1b[0m", i + 1);
            println!("{}", note);
        }
    }

    /// 搜索笔记（按关键词）
    ///
    /// 【迭代器链式调用】
    ///   - iter()：创建迭代器
    ///   - filter()：过滤，只保留满足条件的
    ///   - collect()：把结果收集成一个新的 Vec
    ///
    /// 【闭包的捕获】
    ///   - keyword 是从外部捕获的不可变引用
    ///   - 闭包里的 |n| 是每一项的参数
    ///   - n.contains(keyword) 调用 Note 的 contains 方法
    ///
    fn search_notes(&self, keyword: &str) -> Vec<&Note> {
        self.notes
            .iter()                              // 创建迭代器，每个元素是 &Note
            .filter(|n| n.contains(keyword))     // 闭包：只保留包含关键词的笔记
            .collect()                            // 收集成 Vec<&Note>
    }

    /// 删除指定索引的笔记
    ///
    /// 【可变借用】
    ///   - &mut self 表示"可变借用"——我要修改数据
    ///   - 就像你在自己的本子上写字，可以修改内容
    ///
    /// 【返回值】
    ///   - 返回 Option<Note>：可能删除成功（Some），也可能索引越界（None）
    ///   - Option 是 Rust 处理"可能有值也可能没值"的方式
    ///
    fn delete_note(&mut self, index: usize) -> Option<Note> {
        // 先检查索引是否有效
        if index < self.notes.len() {
            // remove 会把指定位置的元素拿出来，后面的元素往前移
            let removed = self.notes.remove(index);
            println!("\x1b[32m✓ 笔记 \"{}\" 已删除！\x1b[0m", removed.title);
            Some(removed)
        } else {
            println!("\x1b[31m✗ 索引越界！有效范围是 1 到 {}\x1b[0m", self.notes.len());
            None
        }
    }

    /// 保存所有笔记到文件（JSON 格式）
    ///
    /// 【文件写入流程】
    ///   1. 把每条笔记转成 JSON 字符串
    ///   2. 用逗号连接，外面包上中括号，形成 JSON 数组
    ///   3. 写入文件
    ///
    /// 【错误处理】
    ///   - 文件操作可能失败（权限不足、磁盘满等）
    ///   - 返回 Result<(), AppError>：成功返回 Ok(())，失败返回 Err(...)
    ///
    fn save_to_file(&self, filename: &str) -> Result<(), AppError> {
        // 迭代器：把每条笔记转成 JSON 字符串
        let json_items: Vec<String> = self.notes
            .iter()
            .map(|note| note.to_json())    // 闭包：对每条笔记调用 to_json
            .collect();

        // 拼接成完整的 JSON 数组字符串
        let json_string = format!("[\n  {}\n]", json_items.join(",\n  "));

        // fs::write 是最简单的文件写入方式
        // 如果文件不存在会创建，如果存在会覆盖
        fs::write(filename, json_string)
            .map_err(|e| AppError::IoError(e))?;  // 把 io::Error 转成 AppError

        println!("\x1b[32m✓ 笔记已保存到文件 \"{}\"！\x1b[0m", filename);
        Ok(())
    }

    /// 从文件加载笔记
    ///
    /// 【文件读取流程】
    ///   1. 读取整个文件内容为字符串
    ///   2. 解析 JSON 字符串，提取每条笔记
    ///   3. 添加到当前记事本中
    ///
    fn load_from_file(&mut self, filename: &str) -> Result<(), AppError> {
        // fs::read_to_string 读取文件全部内容
        let content = fs::read_to_string(filename)
            .map_err(|e| AppError::IoError(e))?;

        // 简单的 JSON 数组解析：找到每一对 { ... }
        let mut notes = Vec::new();
        let mut depth = 0;          // 花括号深度
        let mut start = None;       // 当前 JSON 对象的起始位置
        let chars: Vec<char> = content.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            match ch {
                '{' => {
                    if depth == 0 {
                        start = Some(i); // 记录起始位置
                    }
                    depth += 1;
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        if let Some(s) = start {
                            // 提取 JSON 对象字符串并解析
                            let json_obj: String = chars[s..=i].iter().collect();
                            match Note::from_json(&json_obj) {
                                Ok(note) => notes.push(note),
                                Err(e) => {
                                    println!("\x1b[33m⚠ 跳过一条无法解析的笔记: {}\x1b[0m", e);
                                }
                            }
                            start = None;
                        }
                    }
                }
                _ => {} // 其他字符忽略
            }
        }

        let count = notes.len();
        // extend 会把 notes 中的所有元素转移到 self.notes 中
        self.notes.extend(notes);
        println!("\x1b[32m✓ 从文件 \"{}\" 加载了 {} 条笔记！\x1b[0m", filename, count);
        Ok(())
    }
}

// ---------- 第三小节：Command 枚举 ----------

/// Command 枚举：表示用户可以执行的命令
///
/// 【枚举的比喻】
///   - 枚举就像一个"多选题"，用户只能选其中一个
///   - 每个选项可以带不同的"附加信息"（关联数据）
///
/// 【变体说明】
///   - Add { title, content }：添加笔记，需要标题和内容
///   - List：列出所有笔记，不需要额外信息
///   - Search(keyword)：搜索笔记，需要关键词
///   - Delete(index)：删除笔记，需要索引
///   - Save(filename)：保存到文件，需要文件名
///   - Load(filename)：从文件加载，需要文件名
///   - Quit：退出程序，不需要额外信息
///
#[derive(Debug)]
enum Command {
    Add {
        title: String,
        content: String,
        tags: Vec<String>,
    },
    List,
    Search(String),       // 搜索关键词
    Delete(usize),        // 要删除的笔记索引（从 1 开始）
    Save(String),         // 保存的文件名
    Load(String),         // 加载的文件名
    Edit {
        index: usize,     // 要编辑的笔记索引
        new_title: Option<String>,   // 新标题（可选）
        new_content: Option<String>, // 新内容（可选）
    },
    Help,                 // 显示帮助信息
    Quit,                 // 退出程序
    // --- 进阶功能 新增命令 ---
    Todo(usize, Priority), // 设置优先级：todo 序号 高/中/低
    Stats,                 // 显示统计信息
    Done(usize),           // 标记完成：done 序号
    Export(String),        // 导出 Markdown：export [文件名]
}

/// 解析用户输入的字符串为 Command
///
/// 【FromStr Trait】
///   - FromStr 是 Rust 标准库的 trait
///   - 它定义了如何从字符串解析出某种类型
///   - 就像把"123"这个字符串转成数字 123
///
/// 【错误处理】
///   - parse 可能失败（用户输入了无效命令）
///   - 所以返回 Result<Command, AppError>
///
fn parse_command(input: &str) -> Result<Command, AppError> {
    // trim() 去掉首尾空白字符
    let input = input.trim();

    // split_whitespace() 按空白字符分割，返回迭代器
    // collect() 收集成 Vec<&str>
    let parts: Vec<&str> = input.splitn(3, ' ').collect();

    // 取第一个词作为命令名，如果没有则为空字符串
    let cmd = parts.first().unwrap_or(&"");

    // match 匹配命令
    match *cmd {
        "add" => {
            // add 命令格式：add 标题 | 内容 | 标签1,标签2
            // 例如：add 买菜清单 | 鸡蛋、牛奶 | 购物,日常
            if parts.len() < 2 {
                return Err(AppError::InvalidCommand("用法: add 标题 | 内容 | 标签1,标签2".to_string()));
            }

            // 去掉 "add " 前缀，取剩下的部分
            let rest = &input[3..]; // 跳过 "add"
            let segments: Vec<&str> = rest.split('|').map(|s| s.trim()).collect();

            let title = segments.first()
                .ok_or(AppError::InvalidCommand("缺少标题".to_string()))?
                .to_string();

            let content = segments.get(1)
                .unwrap_or(&"")
                .to_string();

            let tags: Vec<String> = segments
                .get(2)
                .map(|s| s.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect())
                .unwrap_or_default();

            Ok(Command::Add { title, content, tags })
        }

        "list" | "ls" => Ok(Command::List),

        "search" | "find" => {
            if parts.len() < 2 {
                return Err(AppError::InvalidCommand("用法: search 关键词".to_string()));
            }
            Ok(Command::Search(parts[1].to_string()))
        }

        "delete" | "del" | "rm" => {
            if parts.len() < 2 {
                return Err(AppError::InvalidCommand("用法: delete 序号".to_string()));
            }
            // 解析序号：字符串 -> 数字
            let index: usize = parts[1]
                .parse()
                .map_err(|_| AppError::InvalidCommand(format!("\"{}\" 不是有效的序号", parts[1])))?;
            Ok(Command::Delete(index))
        }

        "save" => {
            let filename = if parts.len() >= 2 {
                parts[1].to_string()
            } else {
                "notes.json".to_string() // 默认文件名
            };
            Ok(Command::Save(filename))
        }

        "load" => {
            let filename = if parts.len() >= 2 {
                parts[1].to_string()
            } else {
                "notes.json".to_string() // 默认文件名
            };
            Ok(Command::Load(filename))
        }

        "edit" => {
            if parts.len() < 2 {
                return Err(AppError::InvalidCommand("用法: edit 序号 | 新标题 | 新内容".to_string()));
            }
            let rest = &input[4..]; // 跳过 "edit"
            let segments: Vec<&str> = rest.split('|').map(|s| s.trim()).collect();

            let index: usize = segments.first()
                .ok_or(AppError::InvalidCommand("缺少序号".to_string()))?
                .parse()
                .map_err(|_| AppError::InvalidCommand("序号必须是数字".to_string()))?;

            let new_title = segments.get(1).filter(|s| !s.is_empty()).map(|s| s.to_string());
            let new_content = segments.get(2).filter(|s| !s.is_empty()).map(|s| s.to_string());

            Ok(Command::Edit { index, new_title, new_content })
        }

        "help" | "h" | "?" => Ok(Command::Help),

        "quit" | "exit" | "q" => Ok(Command::Quit),

        // --- 进阶功能 新增命令 ---

        "todo" => {
            // todo 命令格式：todo 序号 高/中/低
            if parts.len() < 3 {
                return Err(AppError::InvalidCommand("用法: todo 序号 高/中/低".to_string()));
            }
            let index: usize = parts[1]
                .parse()
                .map_err(|_| AppError::InvalidCommand("序号必须是数字".to_string()))?;
            let priority = Priority::from_str(parts[2])
                .ok_or(AppError::InvalidCommand(
                    format!("\"{}\" 不是有效的优先级，请用 高/中/低 或 high/medium/low", parts[2])
                ))?;
            Ok(Command::Todo(index, priority))
        }

        "done" => {
            // done 命令格式：done 序号
            if parts.len() < 2 {
                return Err(AppError::InvalidCommand("用法: done 序号".to_string()));
            }
            let index: usize = parts[1]
                .parse()
                .map_err(|_| AppError::InvalidCommand("序号必须是数字".to_string()))?;
            Ok(Command::Done(index))
        }

        "stats" => Ok(Command::Stats),

        "export" => {
            let filename = if parts.len() >= 2 {
                parts[1].to_string()
            } else {
                "notes.md".to_string() // 默认文件名
            };
            Ok(Command::Export(filename))
        }

        "" => Err(AppError::InvalidCommand("请输入命令，输入 help 查看帮助".to_string())),

        other => Err(AppError::InvalidCommand(format!(
            "未知命令 \"{}\"，输入 help 查看帮助",
            other
        ))),
    }
}

// ============================================================================
// 第二部分：自定义错误类型 —— AppError
// ============================================================================
//
// 【类比时间】
//
// 想象你去餐厅点菜，可能会遇到各种问题：
//   - 菜单上没有这道菜（InvalidCommand）
//   - 厨房着火了（IoError）
//   - 食材解析失败（ParseError）
//
// 我们需要一个统一的"错误类型"来处理所有这些问题。
// 这就是枚举的用武之地！
//

/// AppError 枚举：程序中所有可能出现的错误
///
/// 【错误处理的好处】
///   - 强制你考虑每种错误情况
///   - 代码更健壮，不会因为一个错误就崩溃
///   - 用户看到的是友好的提示，而不是天书般的报错
///
#[derive(Debug)]
enum AppError {
    InvalidCommand(String),  // 无效的命令
    IoError(io::Error),      // 文件 I/O 错误
    ParseError(String),      // 解析错误
}

/// 为 AppError 实现 Display trait
///
/// 【为什么需要 Display？】
///   - 这样我们就能用 println!("{}", error) 打印错误信息
///   - 用户看到的是人类能读懂的提示
///
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidCommand(msg) => write!(f, "命令错误: {}", msg),
            AppError::IoError(e) => write!(f, "文件错误: {}", e),
            AppError::ParseError(msg) => write!(f, "解析错误: {}", msg),
        }
    }
}

/// 为 AppError 实现 From<io::Error> trait
///
/// 【From Trait 的作用】
///   - 让我们可以用 ? 运算符自动把 io::Error 转成 AppError
///   - 这样就不用每次手动转换了
///   - 就像有了"自动翻译机"，英文进来，自动变成中文
///
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

// ============================================================================
// 第三部分：辅助函数 —— JSON 处理和时间获取
// ============================================================================
//
// 这些函数帮助我们处理 JSON 和时间，
// 是"工具人"角色，虽然不显眼但很重要。
//

/// 获取当前时间的简化版本
///
/// 【说明】
///   - 实际项目中应该用 chrono 库来获取精确时间
///   - 这里为了不引入外部依赖，用一个简化的方式
///   - 返回一个格式化的日期时间字符串
///
fn get_current_time() -> String {
    // 使用一个简化的固定时间格式
    // 在真实项目中，你会用 chrono::Local::now().format(...)
    "2026-06-13".to_string()
}

/// 转义 JSON 字符串中的特殊字符
///
/// 【JSON 转义规则】
///   - 双引号 " 需要转义为 \"
///   - 反斜杠 \ 需要转义为 \\
///   - 换行符 \n 需要转义为 \\n
///
fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")   // 先替换反斜杠（顺序很重要！）
     .replace('"', "\\\"")    // 再替换双引号
     .replace('\n', "\\n")    // 替换换行符
     .replace('\r', "\\r")    // 替换回车符
     .replace('\t', "\\t")    // 替换制表符
}

/// 从 JSON 字符串中提取指定键的字符串值
///
/// 【手动解析 JSON 的思路】
///   1. 找到 "key":" 的位置
///   2. 从这个位置往后找，提取引号内的内容
///   3. 处理转义字符
///
fn extract_json_string(json: &str, key: &str) -> Result<String, AppError> {
    let pattern = format!("\"{}\":\"", key);
    let start = json.find(&pattern)
        .ok_or(AppError::ParseError(format!("找不到字段 \"{}\"", key)))?
        + pattern.len();

    // 从 start 位置开始找结束的引号（要考虑转义）
    let mut end = start;
    let chars: Vec<char> = json[start..].chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            i += 2; // 跳过转义字符（如 \" 和 \\）
        } else if chars[i] == '"' {
            break;  // 找到结束引号
        } else {
            i += 1;
        }
    }
    end = start + i;

    let raw = &json[start..end];
    // 反转义：把 \\" 变回 "，把 \\n 变回换行等
    Ok(raw.replace("\\\"", "\"")
          .replace("\\n", "\n")
          .replace("\\r", "\r")
          .replace("\\t", "\t")
          .replace("\\\\", "\\"))
}

/// 从 JSON 字符串中提取指定键的数组值
///
fn extract_json_array(json: &str, key: &str) -> Result<Vec<String>, AppError> {
    let pattern = format!("\"{}\":[", key);
    let start = json.find(&pattern)
        .ok_or(AppError::ParseError(format!("找不到数组字段 \"{}\"", key)))?
        + pattern.len();

    // 找到 ] 结束位置
    let end = json[start..].find(']')
        .ok_or(AppError::ParseError("数组格式错误".to_string()))?
        + start;

    let array_content = &json[start..end].trim();

    if array_content.is_empty() {
        return Ok(Vec::new());
    }

    // 解析数组中的每个字符串元素
    let mut items = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut escape_next = false;

    for ch in array_content.chars() {
        if escape_next {
            current.push(ch);
            escape_next = false;
            continue;
        }
        match ch {
            '\\' if in_string => {
                escape_next = true;
                current.push(ch);
            }
            '"' => in_string = !in_string,
            ',' if !in_string => {
                let item = current.trim().trim_matches('"').to_string();
                if !item.is_empty() {
                    items.push(item);
                }
                current.clear();
            }
            _ if in_string => current.push(ch),
            _ => {} // 忽略空白等
        }
    }
    // 最后一个元素
    let item = current.trim().trim_matches('"').to_string();
    if !item.is_empty() {
        items.push(item);
    }

    Ok(items)
}

/// 显示帮助信息
///
/// 【格式化输出】
///   - 用 \x1b[1;36m 设置青色加粗
///   - 用 \x1b[0m 重置颜色
///   - 让帮助信息更易读
///
fn show_help() {
    println!("\n\x1b[1;36m╔══════════════════════════════════════════════╗");
    println!("║         G-one 的命令行记事本 - 帮助         ║");
    println!("╚══════════════════════════════════════════════╝\x1b[0m");
    println!();
    println!("  \x1b[1;33madd\x1b[0m 标题 | 内容 | 标签1,标签2   添加新笔记");
    println!("  \x1b[1;33mlist\x1b[0m  / \x1b[1;33mls\x1b[0m                       列出所有笔记");
    println!("  \x1b[1;33msearch\x1b[0m 关键词                   搜索笔记");
    println!("  \x1b[1;33mdelete\x1b[0m 序号                      删除笔记");
    println!("  \x1b[1;33medit\x1b[0m 序号 | 新标题 | 新内容     编辑笔记");
    println!("  \x1b[1;33msave\x1b[0m [文件名]                    保存到文件（默认 notes.json）");
    println!("  \x1b[1;33mload\x1b[0m [文件名]                    从文件加载（默认 notes.json）");
    println!("  \x1b[1;33mhelp\x1b[0m / \x1b[1;33m?\x1b[0m                         显示此帮助");
    println!("  \x1b[1;33mquit\x1b[0m / \x1b[1;33mq\x1b[0m                         退出程序");
    println!();
    println!("  \x1b[1;35m--- 进阶功能 进阶命令 ---\x1b[0m");
    println!("  \x1b[1;33mtodo\x1b[0m 序号 高/中/低               设置优先级");
    println!("  \x1b[1;33mdone\x1b[0m 序号                        标记为已完成");
    println!("  \x1b[1;33mstats\x1b[0m                              显示统计信息");
    println!("  \x1b[1;33mexport\x1b[0m [文件名]                    导出为 Markdown（默认 notes.md）");
    println!();
    println!("  \x1b[2m提示：add 命令中用竖线 | 分隔标题、内容和标签\x1b[0m");
    println!("  \x1b[2m示例：add 买菜清单 | 鸡蛋 牛奶 面包 | 购物,日常\x1b[0m");
    println!();
}

// ============================================================================
// 第四部分：模块内联演示 —— 用 mod 组织代码
// ============================================================================
//
// 【模块的比喻】
//
// 想象你有一个大书柜：
//   - 第一层放小说（models 模块：数据结构）
//   - 第二层放工具书（errors 模块：错误处理）
//   - 第三层放教材（storage 模块：文件存储）
//
// 在实际项目中，这些模块会放在不同的文件里。
// 这里为了方便学习，我们把它们"内联"在同一个文件中。
//

/// models 模块：数据模型相关的工具函数
mod models {
    /// 格式化笔记的摘要信息（用于搜索结果显示）
    pub fn format_summary(title: &str, content: &str, max_len: usize) -> String {
        // 如果内容太长，截断并加上省略号
        let display_content = if content.len() > max_len {
            format!("{}...", &content[..max_len])
        } else {
            content.to_string()
        };
        format!("{}: {}", title, display_content)
    }
}

/// storage 模块：存储相关的工具函数
mod storage {
    use std::fs;
    use std::path::Path;

    /// 检查文件是否存在
    pub fn file_exists(filename: &str) -> bool {
        Path::new(filename).exists()
    }

    /// 获取文件大小（字节）
    pub fn file_size(filename: &str) -> Option<u64> {
        fs::metadata(filename).ok().map(|m| m.len())
    }
}

// ============================================================================
// 第五部分：进阶功能 进阶 —— TODO CLI 核心功能
// ============================================================================
//
// 【写给想继续挑战的你】
//
// Phase 1 我们做了基本的记事本。现在来加上 TODO（待办事项）的核心功能：
//
//   1. Priority（优先级）：给每条笔记标记 Low / Medium / High
//   2. 统计功能：看看总共有多少条、完成了多少、按优先级分布
//   3. 标记完成：把一条笔记标记为"已完成"
//   4. Markdown 导出：把所有笔记导出成 .md 文件
//
// 这些功能综合运用了：
//   - 枚举（Priority）+ Display trait
//   - 结构体（TodoItem）+ 方法
//   - 迭代器链（filter / count / map）
//   - 文件 I/O（写入 Markdown 文件）
//
// 准备好了吗？让我们给记事本"升级"！
//

// ---------- 进阶功能.1：Priority 优先级枚举 ----------

/// Priority 枚举：任务的优先级
///
/// 【枚举 vs 布尔值】
///   - 为什么不用 bool（重要/不重要）？
///   - 因为优先级有三个等级，bool 只有两种状态
///   - 枚举可以表达"多种互斥状态"，比 bool 更精确
///
#[derive(Debug, Clone, PartialEq)]
enum Priority {
    Low,    // 低优先级
    Medium, // 中优先级
    High,   // 高优先级
}

/// 为 Priority 实现 Display
///
/// 【带颜色输出】
///   - Low 绿色（不紧急）
///   - Medium 黄色（需要关注）
///   - High 红色（立刻处理！）
///
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "\x1b[32m[低]\x1b[0m"),
            Priority::Medium => write!(f, "\x1b[33m[中]\x1b[0m"),
            Priority::High => write!(f, "\x1b[1;31m[高]\x1b[0m"),
        }
    }
}

impl Priority {
    /// 从字符串解析优先级
    ///
    /// 【输入容错】
    ///   - 支持中文：低、中、高
    ///   - 支持英文：low、medium、high
    ///   - 大小写不敏感
    ///
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" | "低" => Some(Priority::Low),
            "medium" | "med" | "中" => Some(Priority::Medium),
            "high" | "高" => Some(Priority::High),
            _ => None,
        }
    }
}

// ---------- 进阶功能.2：TodoItem 带优先级的待办项 ----------

/// TodoItem 结构体：带优先级和完成状态的待办项
///
/// 【组合优于继承】
///   - Rust 没有类的继承，但可以"组合"
///   - TodoItem 里面包含一个 Note（拥有关联数据）
///   - 再加上 priority 和 done 字段
///
#[derive(Debug, Clone)]
struct TodoItem {
    note: Note,             // 关联的笔记（组合：把 Note 作为字段）
    priority: Priority,     // 优先级
    done: bool,             // 是否完成
}

impl TodoItem {
    /// 创建一个待办项
    fn new(note: Note, priority: Priority) -> Self {
        TodoItem {
            note,
            priority,
            done: false, // 新建时默认未完成
        }
    }

    /// 标记为已完成
    ///
    /// 【可变借用】
    ///   - &mut self：需要修改自身的状态
    ///   - 把 done 从 false 改成 true
    ///
    fn mark_done(&mut self) {
        self.done = true;
        println!(
            "\x1b[32m✓ 已完成：{} {}\x1b[0m",
            self.priority, self.note.title
        );
    }

    /// 格式化显示
    fn display(&self) -> String {
        let status = if self.done {
            "\x1b[32m[已完成]\x1b[0m"
        } else {
            "\x1b[31m[未完成]\x1b[0m"
        };
        format!(
            "{} {} {}\n  内容：{}\n  时间：{}",
            self.priority, status, self.note.title,
            self.note.content, self.note.created_at
        )
    }

    /// 获取优先级的数值（用于排序，高优先级数字大）
    fn priority_value(&self) -> u8 {
        match self.priority {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
        }
    }
}

// ---------- 进阶功能.3：Notebook 新增方法（统计 & 导出）----------

impl Notebook {
    /// 按优先级筛选笔记
    ///
    /// 【迭代器链式调用】
    ///   - 我们复用 Note 的 tags 来存储优先级标记
    ///   - 用 filter + any 在标签中查找优先级关键词
    ///
    fn search_by_priority(&self, priority: &Priority) -> Vec<&Note> {
        let keyword = match priority {
            Priority::Low => "低",
            Priority::Medium => "中",
            Priority::High => "高",
        };
        self.notes
            .iter()
            .filter(|note| {
                note.tags.iter().any(|tag| tag == keyword)
            })
            .collect()
    }

    /// 统计功能：显示记事本的统计信息
    ///
    /// 【迭代器的统计方法】
    ///   - len()：总数
    ///   - filter().count()：满足条件的数量
    ///   - 这些方法让你不用写 for 循环就能统计数据
    ///
    fn show_statistics(&self) {
        let total = self.notes.len();

        // 用 tags 中是否包含"已完成"来判断完成状态
        let done_count = self.notes
            .iter()
            .filter(|n| n.tags.iter().any(|t| t == "已完成"))
            .count();

        let undone_count = total - done_count;

        // 按优先级统计：用闭包捕获 priority 关键词
        let high_count = self.notes
            .iter()
            .filter(|n| n.tags.iter().any(|t| t == "高"))
            .count();
        let medium_count = self.notes
            .iter()
            .filter(|n| n.tags.iter().any(|t| t == "中"))
            .count();
        let low_count = self.notes
            .iter()
            .filter(|n| n.tags.iter().any(|t| t == "低"))
            .count();

        // 没有标记优先级的笔记
        let no_priority = total - high_count - medium_count - low_count;

        println!("\n\x1b[1;36m╔══════════════════════════════════╗");
        println!("║         📊 统计信息              ║");
        println!("╚══════════════════════════════════╝\x1b[0m");
        println!();
        println!("  📝 总数：\x1b[1;33m{}\x1b[0m 条", total);
        println!("  ✅ 已完成：\x1b[32m{}\x1b[0m 条", done_count);
        println!("  ❌ 未完成：\x1b[31m{}\x1b[0m 条", undone_count);
        println!();

        // 用百分比展示完成率
        if total > 0 {
            let rate = (done_count as f64 / total as f64) * 100.0;
            println!("  📈 完成率：\x1b[1;33m{:.1}%\x1b[0m", rate);
        }
        println!();

        // 优先级分布
        println!("  \x1b[1m优先级分布：\x1b[0m");
        println!("    🔴 高优先级：\x1b[1;31m{}\x1b[0m 条", high_count);
        println!("    🟡 中优先级：\x1b[33m{}\x1b[0m 条", medium_count);
        println!("    🟢 低优先级：\x1b[32m{}\x1b[0m 条", low_count);
        if no_priority > 0 {
            println!("    ⚪ 未标记：\x1b[2m{}\x1b[0m 条", no_priority);
        }
        println!();
    }

    /// 导出为 Markdown 格式
    ///
    /// 【Markdown 简介】
    ///   - Markdown 是一种轻量级的"标记语言"
    ///   - 用简单的符号表示格式：# 标题、- 列表、**加粗** 等
    ///   - GitHub、笔记软件等都支持 Markdown
    ///
    fn export_markdown(&self, filename: &str) -> Result<(), AppError> {
        if self.notes.is_empty() {
            return Err(AppError::InvalidCommand("记事本是空的，没有内容可以导出".to_string()));
        }

        // 用 String::new() 创建空字符串，然后用 push_str 追加内容
        let mut md = String::new();

        // 标题
        md.push_str("# 我的笔记\n\n");
        md.push_str(&format!("> 共 {} 条笔记\n\n", self.notes.len()));
        md.push_str("---\n\n");

        // 遍历每条笔记，生成 Markdown 内容
        for (i, note) in self.notes.iter().enumerate() {
            // 二级标题：笔记标题
            md.push_str(&format!("## {}. {}\n\n", i + 1, note.title));

            // 如果有优先级标签，显示出来
            if let Some(priority_tag) = note.tags.iter().find(|t| {
                matches!(t.as_str(), "高" | "中" | "低")
            }) {
                let priority_label = match priority_tag.as_str() {
                    "高" => "🔴 高优先级",
                    "中" => "🟡 中优先级",
                    "低" => "🟢 低优先级",
                    _ => priority_tag,
                };
                md.push_str(&format!("**优先级：{}**\n\n", priority_label));
            }

            // 完成状态
            if note.tags.iter().any(|t| t == "已完成") {
                md.push_str("**状态：** ✅ 已完成\n\n");
            } else {
                md.push_str("**状态：** ❌ 未完成\n\n");
            }

            // 笔记内容（引用格式）
            md.push_str(&format!("> {}\n\n", note.content));

            // 标签（去除内部标记标签，只显示用户自定义标签）
            let user_tags: Vec<&str> = note.tags
                .iter()
                .filter(|t| !matches!(t.as_str(), "高" | "中" | "低" | "已完成"))
                .map(|s| s.as_str())
                .collect();
            if !user_tags.is_empty() {
                md.push_str(&format!("**标签：** {}\n\n", user_tags.join(", ")));
            }

            // 创建时间
            md.push_str(&format!("*创建于：{}*\n\n", note.created_at));
            md.push_str("---\n\n");
        }

        // 尾部
        md.push_str("*由命令行记事本导出*\n");

        // 写入文件
        fs::write(filename, md)
            .map_err(|e| AppError::IoError(e))?;

        println!("\x1b[32m✓ 已导出为 Markdown 文件 \"{}\"！\x1b[0m", filename);
        println!("\x1b[2m  你可以在任何 Markdown 阅读器中打开它\x1b[0m");
        Ok(())
    }
}

// ============================================================================
// 第六部分：主程序 —— REPL 循环
// ============================================================================
//
// 【REPL 是什么？】
//
// REPL = Read（读取）→ Evaluate（执行）→ Print（打印）→ Loop（循环）
//
// 就像你和朋友聊天：
//   1. 朋友说了一句话（Read）
//   2. 你理解了这句话（Evaluate）
//   3. 你回答了（Print）
//   4. 朋友又说了一句（Loop，继续循环）
//
// 我们的记事本就是这样一个"聊天机器人"！
//

fn main() {
    // 打印欢迎信息
    println!("\n\x1b[1;36m╔══════════════════════════════════════════════╗");
    println!("║     欢迎使用 G-one 的命令行记事本 v1.0     ║");
    println!("║     输入 help 查看所有命令                  ║");
    println!("╚══════════════════════════════════════════════╝\x1b[0m\n");

    // 创建一个空的记事本
    // mut 表示它是可变的，因为我们会往里面添加/删除笔记
    let mut notebook = Notebook::new();

    // REPL 主循环
    // loop {} 表示无限循环，直到用户输入 quit
    loop {
        // 打印提示符
        print!("\x1b[36m📓 > \x1b[0m");
        // 刷新输出缓冲区，确保提示符立即显示
        // （因为 print! 不会自动换行，需要手动刷新）
        use std::io::Write;
        io::stdout().flush().unwrap();

        // 读取用户输入
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // 读到 0 字节，说明输入结束了（比如按了 Ctrl+D）
                println!("\n再见！");
                break;
            }
            Ok(_) => {
                // 成功读取到输入
            }
            Err(e) => {
                // 读取失败
                eprintln!("读取输入失败: {}", e);
                continue;
            }
        }

        // 解析命令
        let command = match parse_command(&input) {
            Ok(cmd) => cmd,             // 解析成功
            Err(e) => {
                // 解析失败，打印错误信息，继续循环
                println!("\x1b[31m✗ {}\x1b[0m", e);
                continue;
            }
        };

        // 执行命令
        // match 会根据命令的类型，执行对应的代码
        match command {
            Command::Add { title, content, tags } => {
                // 创建新笔记并添加到记事本
                let note = Note::new(&title, &content, tags);
                notebook.add_note(note);
            }

            Command::List => {
                // 列出所有笔记
                notebook.list_notes();
            }

            Command::Search(keyword) => {
                // 搜索笔记
                println!("\n\x1b[1;36m=== 搜索 \"{}\" 的结果 ===\x1b[0m", keyword);

                // 使用迭代器链式调用进行搜索
                let results = notebook.search_notes(&keyword);

                if results.is_empty() {
                    println!("\x1b[33m没有找到包含 \"{}\" 的笔记\x1b[0m", keyword);
                } else {
                    println!("找到 {} 条匹配的笔记：\n", results.len());
                    // iter() + enumerate()：遍历搜索结果
                    for (i, note) in results.iter().enumerate() {
                        println!("\x1b[1;34m--- 第 {} 条 ---\x1b[0m", i + 1);
                        println!("{}\n", note);
                    }
                }
            }

            Command::Delete(index) => {
                // 删除笔记（用户输入从 1 开始，内部从 0 开始）
                if index == 0 {
                    println!("\x1b[31m✗ 序号从 1 开始，不能是 0\x1b[0m");
                } else {
                    notebook.delete_note(index - 1);
                }
            }

            Command::Save(filename) => {
                // 保存笔记到文件
                if let Err(e) = notebook.save_to_file(&filename) {
                    println!("\x1b[31m✗ 保存失败: {}\x1b[0m", e);
                }
            }

            Command::Load(filename) => {
                // 从文件加载笔记
                // 先检查文件是否存在（使用 storage 模块的函数）
                if !storage::file_exists(&filename) {
                    println!("\x1b[31m✗ 文件 \"{}\" 不存在！\x1b[0m", filename);
                } else {
                    // 显示文件大小（使用 storage 模块的函数）
                    if let Some(size) = storage::file_size(&filename) {
                        println!("\x1b[2m文件大小: {} 字节\x1b[0m", size);
                    }
                    if let Err(e) = notebook.load_from_file(&filename) {
                        println!("\x1b[31m✗ 加载失败: {}\x1b[0m", e);
                    }
                }
            }

            Command::Edit { index, new_title, new_content } => {
                // 编辑笔记
                if index == 0 {
                    println!("\x1b[31m✗ 序号从 1 开始，不能是 0\x1b[0m");
                } else if index > notebook.notes.len() {
                    println!("\x1b[31m✗ 索引越界！有效范围是 1 到 {}\x1b[0m", notebook.notes.len());
                } else {
                    let note = &mut notebook.notes[index - 1];
                    if let Some(title) = new_title {
                        note.title = title;
                        println!("\x1b[32m✓ 标题已更新\x1b[0m");
                    }
                    if let Some(content) = new_content {
                        note.content = content;
                        println!("\x1b[32m✓ 内容已更新\x1b[0m");
                    }
                }
            }

            // --- 进阶功能 新增命令的执行 ---

            Command::Todo(index, priority) => {
                // 给笔记设置优先级
                if index == 0 {
                    println!("\x1b[31m✗ 序号从 1 开始，不能是 0\x1b[0m");
                } else if index > notebook.notes.len() {
                    println!("\x1b[31m✗ 索引越界！有效范围是 1 到 {}\x1b[0m", notebook.notes.len());
                } else {
                    let note = &mut notebook.notes[index - 1];
                    // 用优先级关键词作为标签
                    let priority_tag = match &priority {
                        Priority::Low => "低",
                        Priority::Medium => "中",
                        Priority::High => "高",
                    };
                    // 先移除旧的优先级标签
                    note.tags.retain(|t| !matches!(t.as_str(), "高" | "中" | "低"));
                    // 添加新的优先级标签
                    note.tags.push(priority_tag.to_string());
                    println!("\x1b[32m✓ 已将 \"{}\" 设为 {} 优先级\x1b[0m", note.title, priority);
                }
            }

            Command::Done(index) => {
                // 标记笔记为已完成
                if index == 0 {
                    println!("\x1b[31m✗ 序号从 1 开始，不能是 0\x1b[0m");
                } else if index > notebook.notes.len() {
                    println!("\x1b[31m✗ 索引越界！有效范围是 1 到 {}\x1b[0m", notebook.notes.len());
                } else {
                    let note = &mut notebook.notes[index - 1];
                    if !note.tags.iter().any(|t| t == "已完成") {
                        note.tags.push("已完成".to_string());
                    }
                    println!(
                        "\x1b[32m✓ 已完成：\"{}\"\x1b[0m",
                        note.title
                    );
                }
            }

            Command::Stats => {
                // 显示统计信息
                notebook.show_statistics();
            }

            Command::Export(filename) => {
                // 导出为 Markdown
                if let Err(e) = notebook.export_markdown(&filename) {
                    println!("\x1b[31m✗ 导出失败: {}\x1b[0m", e);
                }
            }

            Command::Help => {
                // 显示帮助信息
                show_help();
            }

            Command::Quit => {
                // 退出程序
                println!("\n\x1b[1;33m👋 再见！记得保存你的笔记哦！\x1b[0m\n");
                break; // 跳出 loop 循环
            }
        }
    }
}

// ============================================================================
// 第七部分：练习题
// ============================================================================
//
// 试试看，用你学到的知识完成以下练习：
//

/*
 * 练习 1（基础）：给 Note 添加标签功能
 *
 * 提示：
 *   - Note 结构体已经有了 tags: Vec<String> 字段
 *   - 在 add 命令中，我们已经支持了标签的解析
 *   - 现在请实现一个按标签搜索的命令：
 *     例如：tag 购物 → 列出所有包含"购物"标签的笔记
 *
 * 思路：
 *   1. 在 Command 枚举中添加 Tag(String) 变体
 *   2. 在 parse_command 中添加 "tag" 命令的解析
 *   3. 在 Notebook 中添加 search_by_tag 方法
 *   4. 用迭代器过滤：self.notes.iter().filter(|n| n.tags.contains(&tag))
 *
 * 代码骨架：
 *
 *   fn search_by_tag(&self, tag: &str) -> Vec<&Note> {
 *       self.notes
 *           .iter()
 *           .filter(|note| note.tags.iter().any(|t| t == tag))
 *           .collect()
 *   }
 *
 */

// ===== 测试版练习 =====
// 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
// 取消下面的注释，运行 cargo test --example lesson14

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_search_by_tag() {
        // 练习 1 的核心功能：按标签搜索笔记
        let mut notebook = Notebook::new();

        notebook.add_note(Note::new(
            "买菜清单", "鸡蛋、牛奶",
            vec!["购物".to_string(), "日常".to_string()],
        ));
        notebook.add_note(Note::new(
            "Rust 学习", "所有权和生命周期",
            vec!["学习".to_string(), "编程".to_string()],
        ));
        notebook.add_note(Note::new(
            "周末计划", "去超市购物",
            vec!["购物".to_string(), "周末".to_string()],
        ));

        // 按标签"购物"搜索，应该找到 2 条
        let results: Vec<&Note> = notebook.notes
            .iter()
            .filter(|note| note.tags.iter().any(|t| t == "购物"))
            .collect();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].title, "买菜清单");
        assert_eq!(results[1].title, "周末计划");

        // 按标签"学习"搜索，应该找到 1 条
        let results: Vec<&Note> = notebook.notes
            .iter()
            .filter(|note| note.tags.iter().any(|t| t == "学习"))
            .collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Rust 学习");

        // 按不存在的标签搜索，应该找到 0 条
        let results: Vec<&Note> = notebook.notes
            .iter()
            .filter(|note| note.tags.iter().any(|t| t == "不存在"))
            .collect();
        assert_eq!(results.len(), 0);
    }
}
*/

/*
 * 练习 2（应用）：添加编辑功能
 *
 * 提示：
 *   - 我们已经在 Command 中添加了 Edit 变体
 *   - 但目前只修改了标题和内容
 *   - 请扩展编辑功能，让用户也能修改标签
 *     例如：edit 1 | | 新内容 | 新标签1,新标签2
 *
 * 思路：
 *   1. 解析新的标签参数
 *   2. 用 if let Some(tags) = new_tags 来更新标签
 *
 * 关键代码：
 *
 *   let new_tags: Option<Vec<String>> = segments
 *       .get(3)
 *       .map(|s| s.split(',').map(|t| t.trim().to_string()).collect());
 *
 *   if let Some(tags) = new_tags {
 *       note.tags = tags;
 *       println!("标签已更新");
 *   }
 *
 */

/*
 * 练习 3（进阶）：添加撤销功能
 *
 * 提示：
 *   - 用一个 Vec<Note> 作为"撤销栈"（undo stack）
 *   - 每次删除笔记时，把被删除的笔记压入栈中
 *   - 用户输入 undo 时，从栈顶弹出笔记，放回记事本
 *
 * 思路：
 *   1. 在 Notebook 中添加 undo_stack: Vec<Note>
 *   2. 在 delete_note 中，把删除的笔记 push 到 undo_stack
 *   3. 添加 undo() 方法，从栈中 pop 并放回 notes
 *   4. 在 Command 中添加 Undo 变体
 *
 * 关键代码：
 *
 *   struct Notebook {
 *       notes: Vec<Note>,
 *       undo_stack: Vec<Note>, // 撤销栈
 *   }
 *
 *   fn undo(&mut self) -> Option<Note> {
 *       if let Some(note) = self.undo_stack.pop() {
 *           self.notes.push(note.clone());
 *           println!("✓ 撤销成功！");
 *           Some(note)
 *       } else {
 *           println!("没有可以撤销的操作");
 *           None
 *       }
 *   }
 *
 */

/*
 * =============================================
 * 恭喜你完成了 Rust 基础课程！
 * =============================================
 *
 * 你已经掌握了：
 * ✓ 环境与第一个程序
 * ✓ 变量与数据类型
 * ✓ 控制流与函数
 * ✓ 所有权系统
 * ✓ 结构体与方法
 * ✓ 生命周期
 * ✓ 枚举与模式匹配
 * ✓ 错误处理
 * ✓ 泛型与 Trait
 * ✓ 集合类型
 * ✓ 迭代器与闭包
 * ✓ 模块系统
 * ✓ Cargo 项目管理
 * ✓ 综合实战：命令行记事本
 *
 * 接下来你可以：
 * 1. 学习 async/await 异步编程（推荐 tokio）
 * 2. 学习 Web 开发（推荐 axum/actix-web）
 * 3. 学习系统编程（推荐《Rust 程序设计语言》）
 * 4. 参与开源项目
 *
 * "如果编译通过，内存就是安全的！"
 */

/*
 * ============================================================
 * 课程总结：核心收获
 * ============================================================
 *
 * 1. 结构体 + 枚举 + impl 是 Rust 面向对象的三驾马车
 * 2. 所有权/借用/生命周期贯穿整个程序的数据管理
 * 3. match + Result + ? 运算符让错误处理优雅而安全
 * 4. 迭代器链式调用让数据处理代码简洁而强大
 * 5. 枚举 + 模式匹配是 Rust 最强大的武器之一
 *
 * ============================================================
 * 常见陷阱
 * ============================================================
 *
 * 1. 在循环中同时持有可变和不可变引用（编译器会阻止你）
 * 2. 忘记处理 Option/Result 的 None/Err 情况（用 unwrap 会 panic）
 */
