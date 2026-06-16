// ============================================================
//  第 11 课：模块系统 —— 给代码建一个整洁的文件柜
// ============================================================
//
//  学习目标：
//  1. 理解模块的概念 —— 用生活类比理解"把代码分门别类"
//  2. 掌握 mod / use / pub 三大核心语法
//  3. 理解 pub / pub(crate) / 私有 的可见性层级
//  4. 掌握 self / super 路径前缀
//  5. 学会 pub use 重导出，简化外部调用路径
//  6. 理解多文件模块结构（mod.rs 和文件名.rs 的对应关系）
//  7. 实战：把学生管理系统拆分成多文件模块
//
//  前置知识：
//  已学 00-10 课，包括变量、控制流、所有权、结构体、
//  生命周期、枚举、错误处理、泛型与 Trait、集合与迭代器、闭包
//
// ============================================================

// ============================================================
//  第一部分：生活类比 —— 模块是什么？
// ============================================================
//
//  想象你有一个巨大的文件柜：
//
//  ┌─────────────────────────────────────────┐
//  │            文件柜（你的程序）              │
//  │                                         │
//  │  ┌──────────┐  ┌──────────┐             │
//  │  │ 抽屉 A   │  │ 抽屉 B   │  ...        │
//  │  │ 数学公式  │  │ 学生信息  │             │
//  │  │ (模块)   │  │ (模块)   │             │
//  │  └──────────┘  └──────────┘             │
//  │                                         │
//  │  有的抽屉没锁（pub）—— 谁都能看           │
//  │  有的抽屉锁着（私有）—— 只有自己能看       │
//  │  有的抽屉只给家人看（pub(crate)）         │
//  │                                         │
//  │  use = 从抽屉里拿出文件放到桌上           │
//  └─────────────────────────────────────────┘
//
//  在 Rust 中：
//  - module（模块）= 文件柜的一个抽屉，把相关的代码放在一起
//  - pub = 公开的抽屉，谁都能打开看
//  - 私有 = 锁着的抽屉，只有同一模块内的代码能看
//  - use = 从抽屉里把需要的东西拿出来，放到当前作用域
//
// ============================================================

// ============================================================
//  第二部分：用 mod 定义内联模块
// ============================================================
//
//  "mod" 是 "module" 的缩写，意思是"定义一个模块"。
//  就像在文件柜里新开一个抽屉。
//
//  内联模块 = 直接写在当前文件里的模块（用花括号包起来）
//

// --- 示例 1：最简单的模块 ---
mod greetings {
    // 这个函数默认是私有的（锁着的抽屉）
    // 只有 greetings 模块内部的代码能调用它
    fn hello_chinese() -> String {
        String::from("你好！")
    }

    // 用 pub 标记，变成公开的（没锁的抽屉）
    // 外部代码也能调用
    pub fn hello_english() -> String {
        String::from("Hello!")
    }

    // 公开函数可以调用私有函数（自己抽屉里的东西当然能用）
    pub fn hello_both() -> String {
        let cn = hello_chinese(); // 内部调用私有函数，没问题
        let en = hello_english();
        format!("{} / {}", cn, en)
    }
}

// --- 示例 2：模块可以嵌套（抽屉里还有小格子）---
mod animals {
    pub mod dogs {
        pub fn bark() -> String {
            String::from("汪汪！")
        }
    }

    pub mod cats {
        pub fn meow() -> String {
            String::from("喵~")
        }
    }
}

// ============================================================
//  第三部分：用 use 导入模块 —— 从抽屉里拿出文件
// ============================================================
//
//  use = 把模块里的东西引入当前作用域，这样就不用每次都写完整路径
//
//  没有 use 时，每次都要写：animals::dogs::bark()
//  有了 use 后，直接写：bark()
//

// 用 use 把模块里的函数导入到顶层
use animals::cats::meow;
use animals::dogs::bark;

// ============================================================
//  第四部分：pub 可见性详解 —— 谁能看这个抽屉？
// ============================================================
//
//  Rust 的可见性就像文件柜的安全级别：
//
//  ┌────────────────────────────────────────────────────┐
//  │  关键字        │  类比              │  谁能访问？    │
//  ├────────────────────────────────────────────────────┤
//  │  (无标记)      │  锁着的抽屉        │  只有本模块    │
//  │  pub           │  没锁的抽屉        │  任何人        │
//  │  pub(crate)    │  只给家人看的抽屉   │  同一个 crate  │
//  │  pub(super)    │  只给上一级看       │  父模块        │
//  │  pub(in path)  │  指定给谁看        │  指定路径的模块 │
//  └────────────────────────────────────────────────────┘
//
//  注意：子模块可以访问父模块的私有内容（就像你能看自己家的东西）
//  但父模块不能直接访问子模块的私有内容（你不能看别人锁着的抽屉）
//

mod visibility_demo {
    // 私有函数 —— 只有 visibility_demo 内部能用
    fn secret_function() -> String {
        String::from("这是秘密！")
    }

    // 公开函数 —— 任何人都能调用
    pub fn public_function() -> String {
        // 公开函数可以调用私有函数（同模块内）
        format!("我能调用秘密函数：{}", secret_function())
    }

    // pub(crate) —— 同一个 crate（项目）内的代码都能访问
    // 但在外部 crate（别人引用你的库）中看不到
    pub(crate) fn crate_function() -> String {
        String::from("只有本项目内的代码能调用我")
    }

    // 嵌套模块演示可见性
    pub mod inner {
        // 子模块可以访问父模块的 pub(crate) 函数
        pub fn call_parent() -> String {
            // 用 super 访问父模块（后面会详细讲）
            super::crate_function()
        }

        // 子模块的私有函数
        fn inner_secret() -> String {
            String::from("inner 的秘密")
        }

        // 子模块的公开函数，可以调用自己的私有函数
        pub fn inner_public() -> String {
            format!("inner 公开：{}", inner_secret())
        }
    }
}

// ============================================================
//  第五部分：self 和 super —— 路径导航
// ============================================================
//
//  想象你在文件柜的某个抽屉里找东西：
//
//  - self = "就在当前这个抽屉里找"
//  - super = "去上一级找"（打开父抽屉）
//
//  ┌──────────────────────────────────┐
//  │  main（顶层）                     │
//  │  ┌──────────────────────────┐   │
//  │  │  mod_a                   │   │
//  │  │  ┌──────────────────┐   │   │
//  │  │  │  mod_b           │   │   │
//  │  │  │  super → mod_a   │   │   │
//  │  │  │  self → mod_b    │   │   │
//  │  │  └──────────────────┘   │   │
│  │  └──────────────────────────┘   │
//  └──────────────────────────────────┘
//

mod path_demo {
    pub fn top_level() -> String {
        String::from("我在 path_demo 顶层")
    }

    pub mod child_a {
        pub fn in_a() -> String {
            String::from("我在 child_a 里")
        }

        // super 表示"父模块"，也就是 path_demo
        pub fn call_parent() -> String {
            format!("child_a 调用了父模块：{}", super::top_level())
        }

        pub mod grandchild {
            // super 的 super = 父模块的父模块
            pub fn call_grandparent() -> String {
                let via_parent = super::in_a(); // super = child_a
                let via_grandparent = super::super::top_level(); // super 的 super = path_demo
                format!("孙模块调用：{} + {}", via_parent, via_grandparent)
            }

            // self 表示"当前模块"，也就是 grandchild 自己
            pub fn call_self() -> String {
                self::helper()
            }

            fn helper() -> String {
                String::from("grandchild 的 helper 被 self:: 调用了")
            }
        }
    }
}

// ============================================================
//  第六部分：pub use 重导出 —— 简化调用路径
// ============================================================
//
//  问题：模块嵌套太深，每次调用都要写一长串路径
//    animals::dogs::bark()  ← 太长了！
//
//  解决：用 pub use 把深层的东西"重导出"到更浅的位置
//    就像把抽屉深处的文件复印一份放到桌面上
//

mod company {
    pub mod departments {
        pub mod engineering {
            pub mod team_a {
                pub fn develop_feature() -> String {
                    String::from("Team A 开发了新功能")
                }
            }
            pub mod team_b {
                pub fn fix_bug() -> String {
                    String::from("Team B 修复了 bug")
                }
            }
        }
    }

    // 用 pub use 重导出，把深层路径的东西暴露到 company 层
    // 这样外部只需要 company::develop_feature() 就行了
    pub use departments::engineering::team_a::develop_feature;
    pub use departments::engineering::team_b::fix_bug;
}

// ============================================================
//  第七部分：多文件模块结构 —— 真正的文件柜
// ============================================================
//
//  在实际项目中，我们不会把所有代码都写在一个文件里。
//  每个模块通常对应一个独立的文件。
//
//  Rust 的文件结构规则：
//  ┌──────────────────────────────────────────────────────────┐
//  │  写法                         │  对应的文件/目录          │
//  ├──────────────────────────────────────────────────────────┤
//  │  mod math_utils;              │  src/math_utils.rs       │
//  │  （或者）                      │  src/math_utils/mod.rs   │
//  ├──────────────────────────────────────────────────────────┤
//  │  mod models;                  │  src/models/mod.rs       │
//  │  models 中有子模块 user       │  src/models/user.rs      │
//  │  models 中有子模块 product    │  src/models/product.rs   │
//  └──────────────────────────────────────────────────────────┘
//
//  一个完整的项目文件结构示例：
//
//  my_project/
//  └── src/
//      ├── main.rs              ← 程序入口，用 mod 声明模块
//      ├── math_utils.rs        ← mod math_utils; 对应的文件
//      └── models/
//          ├── mod.rs           ← mod models; 对应的目录入口
//          ├── user.rs          ← models 子模块 user
//          └── product.rs       ← models 子模块 product
//
//  在 main.rs 中的写法：
//  ```
//  mod math_utils;        // Rust 会去找 src/math_utils.rs 或 src/math_utils/mod.rs
//  mod models;            // Rust 会去找 src/models/mod.rs
//
//  use math_utils::add;
//  use models::user::User;
//  ```
//
//  在 models/mod.rs 中声明子模块：
//  ```
//  pub mod user;          // Rust 会去找 src/models/user.rs
//  pub mod product;       // Rust 会去找 src/models/product.rs
//  ```
//
//  ┌──────────────────────────────────────────────────────────┐
//  │  两种目录风格的区别：                                      │
//  │                                                          │
//  │  风格 1：mod.rs（旧风格，类似 Python 的 __init__.py）      │
//  │    src/models/mod.rs                                     │
//  │    src/models/user.rs                                    │
//  │                                                          │
//  │  风格 2：同名文件（新风格，Rust 2018+ 推荐）               │
//  │    src/models.rs                                         │
//  │    src/models/user.rs                                    │
//  │                                                          │
//  │  注意：两种风格不能混用！一个模块只能用其中一种。            │
//  │  新项目推荐用风格 2，因为打开 tab 时文件名更有辨识度。      │
//  └──────────────────────────────────────────────────────────┘
//

// ============================================================
//  第八部分：实战项目 —— 学生管理系统（模块化重构）
// ============================================================
//
//  把之前课程中的学生管理系统拆分成多个模块。
//  由于这是单文件教学，我们用内联模块模拟多文件结构。
//  注释中会标注：如果拆成真正的多文件，每个部分应该放在哪个文件。
//

// ---------- 相当于 src/errors.rs ----------
// 自定义错误类型模块
mod errors {
    // 这个模块模拟：src/errors.rs

    use std::fmt;

    // 学生管理系统的错误枚举
    #[derive(Debug)]
    pub enum StudentError {
        NotFound(String),           // 找不到学生
        InvalidScore(f64),          // 分数不合法
        DuplicateId(u32),           // ID 重复
        EmptyName,                  // 名字为空
    }

    // 实现 Display trait，让错误能被友好地打印
    impl fmt::Display for StudentError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                StudentError::NotFound(name) => write!(f, "找不到学生：{}", name),
                StudentError::InvalidScore(score) => write!(f, "分数 {} 不合法（应在 0-100 之间）", score),
                StudentError::DuplicateId(id) => write!(f, "学号 {} 已存在", id),
                StudentError::EmptyName => write!(f, "学生姓名不能为空"),
            }
        }
    }
}

// ---------- 相当于 src/models/mod.rs ----------
// 数据模型模块
mod models {
    // 这个模块模拟：src/models/mod.rs
    // 在真正的多文件项目中，这里只需要声明子模块：
    //   pub mod user;
    //   pub mod product;

    // --- 相当于 src/models/user.rs ---
    pub mod user {
        // 学生结构体
        #[derive(Debug, Clone)]
        pub struct Student {
            pub id: u32,
            pub name: String,
            pub scores: Vec<f64>,
        }

        impl Student {
            // 创建新学生，返回 Option 因为名字可能为空
            pub fn new(id: u32, name: &str) -> Option<Self> {
                if name.trim().is_empty() {
                    None
                } else {
                    Some(Student {
                        id,
                        name: String::from(name),
                        scores: Vec::new(),
                    })
                }
            }

            // 添加成绩
            pub fn add_score(&mut self, score: f64) -> bool {
                if (0.0..=100.0).contains(&score) {
                    self.scores.push(score);
                    true
                } else {
                    false
                }
            }

            // 计算平均分
            pub fn average(&self) -> Option<f64> {
                if self.scores.is_empty() {
                    None
                } else {
                    let sum: f64 = self.scores.iter().sum();
                    Some(sum / self.scores.len() as f64)
                }
            }

            // 获取成绩等级
            pub fn grade(&self) -> &str {
                match self.average() {
                    None => "暂无成绩",
                    Some(avg) if avg >= 90.0 => "优秀",
                    Some(avg) if avg >= 80.0 => "良好",
                    Some(avg) if avg >= 70.0 => "中等",
                    Some(avg) if avg >= 60.0 => "及格",
                    Some(_) => "不及格",
                }
            }
        }

        // 用 Display trait 让打印更美观
        impl std::fmt::Display for Student {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let avg_str = match self.average() {
                    Some(avg) => format!("{:.1}", avg),
                    None => String::from("无"),
                };
                write!(
                    f,
                    "[{}] {} | 平均分: {} | 等级: {} | 成绩: {:?}",
                    self.id, self.name, avg_str, self.grade(), self.scores
                )
            }
        }
    }

    // --- 相当于 src/models/product.rs ---
    pub mod course {
        // 课程结构体
        #[derive(Debug, Clone)]
        pub struct Course {
            pub id: u32,
            pub name: String,
            pub credit: f64,
        }

        impl Course {
            pub fn new(id: u32, name: &str, credit: f64) -> Self {
                Course {
                    id,
                    name: String::from(name),
                    credit,
                }
            }
        }

        impl std::fmt::Display for Course {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[{}] {} ({} 学分)", self.id, self.name, self.credit)
            }
        }
    }
}

// ---------- 相当于 src/utils.rs ----------
// 工具函数模块
mod utils {
    // 这个模块模拟：src/utils.rs

    use crate::models::user::Student;  // crate 表示当前项目根

    // 打印分隔线
    pub fn print_separator(title: &str) {
        println!("\n{:=^60}", format!(" {} ", title));
    }

    // 打印所有学生信息
    pub fn print_all_students(students: &[Student]) {
        if students.is_empty() {
            println!("  （暂无学生数据）");
            return;
        }
        for student in students {
            println!("  {}", student);
        }
    }

    // 找出平均分最高的学生
    pub fn find_top_student(students: &[Student]) -> Option<&Student> {
        students.iter().max_by(|a, b| {
            let avg_a = a.average().unwrap_or(0.0);
            let avg_b = b.average().unwrap_or(0.0);
            avg_a.partial_cmp(&avg_b).unwrap()
        })
    }

    // 按平均分排序（从高到低）
    pub fn sort_by_average(students: &mut Vec<Student>) {
        students.sort_by(|a, b| {
            let avg_a = a.average().unwrap_or(0.0);
            let avg_b = b.average().unwrap_or(0.0);
            avg_b.partial_cmp(&avg_a).unwrap()  // 降序
        });
    }

    // 统计各等级人数
    pub fn grade_distribution(students: &[Student]) -> Vec<(String, usize)> {
        let mut dist = std::collections::HashMap::new();
        for student in students {
            let grade = student.grade().to_string();
            *dist.entry(grade).or_insert(0) += 1;
        }
        dist.into_iter().collect()
    }
}

// ============================================================
//  第九部分：pub use 重导出实战 —— 简化对外接口
// ============================================================
//
//  在真实项目中，我们希望对外暴露简洁的接口。
//  用户不需要知道内部的模块结构，只需要 use 一个简单的路径。
//

mod student_system {
    // 内部结构很深
    mod core {
        pub mod student_manager {
            pub fn create_student(id: u32, name: &str) -> String {
                format!("创建学生：[{}] {}", id, name)
            }
        }
        pub mod score_manager {
            pub fn add_score(student_id: u32, score: f64) -> String {
                format!("为学生 {} 添加成绩：{:.1}", student_id, score)
            }
        }
    }

    // 用 pub use 重导出，提供简洁的公共 API
    // 外部只需要 student_system::create_student() 即可
    pub use core::student_manager::create_student;
    pub use core::score_manager::add_score;

    // 这样，外部调用者不需要知道 core::student_manager 这些内部结构
    // 如果以后内部重构（比如把 student_manager 改名为 sm），
    // 只需要改 pub use 这一行，外部代码完全不用动！
}

// ============================================================
//  第十部分：crate、self、super 路径总结
// ============================================================
//
//  ┌────────────────────────────────────────────────────────┐
//  │  路径前缀    │  含义                  │  类比          │
//  ├────────────────────────────────────────────────────────┤
//  │  crate       │  当前 crate 的根       │  整个文件柜    │
//  │  self        │  当前模块              │  当前这个抽屉  │
//  │  super       │  父模块               │  上一级抽屉    │
//  │  (无前缀)    │  从当前模块开始查找     │  从这里开始找  │
//  └────────────────────────────────────────────────────────┘
//
//  使用示例见 path_demo 模块（前面已演示）
//

// ============================================================
//  辅助函数：演示各种模块功能
// ============================================================

// 辅助函数 1：演示基本模块用法
fn demo_basic_modules() {
    utils::print_separator("模块基础：mod 和 use");

    // 调用模块中的公开函数
    // 方式 1：完整路径
    println!("完整路径调用：{}", greetings::hello_english());

    // 方式 2：通过 use 导入后直接调用
    println!("use 导入调用：{}", bark());
    println!("use 导入调用：{}", meow());

    // 方式 3：调用嵌套模块的函数
    println!("嵌套模块：{}", greetings::hello_both());

    // 注意：不能调用私有函数！
    // greetings::hello_chinese();  ← 编译错误！
}

// 辅助函数 2：演示可见性
fn demo_visibility() {
    utils::print_separator("可见性：pub / pub(crate) / 私有");

    // 公开函数 —— 任何人都能调用
    println!("公开函数：{}", visibility_demo::public_function());

    // pub(crate) 函数 —— 同 crate 内能调用
    println!("pub(crate)：{}", visibility_demo::crate_function());

    // 子模块的公开函数
    println!("子模块公开：{}", visibility_demo::inner::inner_public());

    // 子模块调用父模块的 pub(crate)
    println!("子模块→父模块：{}", visibility_demo::inner::call_parent());

    // 注意：不能调用私有函数！
    // visibility_demo::secret_function();  ← 编译错误！
}

// 辅助函数 3：演示路径前缀 self / super
fn demo_path_prefixes() {
    utils::print_separator("路径前缀：self / super");

    // 调用各层函数
    println!("{}", path_demo::top_level());
    println!("{}", path_demo::child_a::in_a());
    println!("{}", path_demo::child_a::call_parent());

    // 孙模块通过 super::super 访问顶层
    println!("{}", path_demo::child_a::grandchild::call_grandparent());

    // self 表示当前模块
    println!("{}", path_demo::child_a::grandchild::call_self());
}

// 辅助函数 4：演示 pub use 重导出
fn demo_re_export() {
    utils::print_separator("pub use 重导出");

    // 不用重导出时，路径很长：
    println!(
        "长路径：{}",
        company::departments::engineering::team_a::develop_feature()
    );

    // 用重导出后，路径很短：
    println!("短路径（pub use）：{}", company::develop_feature());
    println!("短路径（pub use）：{}", company::fix_bug());

    // 学生管理系统的重导出示例
    println!("\n学生管理系统 API：");
    println!("  {}", student_system::create_student(1, "G-one"));
    println!("  {}", student_system::add_score(1, 95.0));
}

// 辅助函数 5：实战 —— 学生管理系统的模块化使用
fn demo_student_system() {
    utils::print_separator("实战：学生管理系统（模块化）");

    // 使用 models 模块中的类型
    use models::course::Course;
    use models::user::Student;

    // 创建学生
    let mut students: Vec<Student> = Vec::new();

    // 用 if let 处理 Option（第 8 课学过）
    if let Some(mut s1) = Student::new(1, "G-one") {
        s1.add_score(92.0);
        s1.add_score(88.5);
        s1.add_score(95.0);
        students.push(s1);
    }

    if let Some(mut s2) = Student::new(2, "小明") {
        s2.add_score(78.0);
        s2.add_score(82.5);
        s2.add_score(75.0);
        students.push(s2);
    }

    if let Some(mut s3) = Student::new(3, "小红") {
        s3.add_score(95.0);
        s3.add_score(98.0);
        s3.add_score(91.5);
        students.push(s3);
    }

    if let Some(mut s4) = Student::new(4, "小刚") {
        s4.add_score(55.0);
        s4.add_score(62.0);
        s4.add_score(58.5);
        students.push(s4);
    }

    // 名字为空的情况
    match Student::new(5, "") {
        Some(_) => println!("不应该到这里"),
        None => println!("名字为空，创建失败（符合预期）"),
    }

    // 使用 utils 模块的工具函数
    utils::print_separator("所有学生");
    utils::print_all_students(&students);

    // 找出最优秀的学生
    if let Some(top) = utils::find_top_student(&students) {
        println!("\n最优秀的学生：{}", top);
    }

    // 按平均分排序
    utils::sort_by_average(&mut students);
    utils::print_separator("按平均分排序（从高到低）");
    utils::print_all_students(&students);

    // 等级分布统计
    let dist = utils::grade_distribution(&students);
    utils::print_separator("等级分布");
    for (grade, count) in &dist {
        println!("  {}：{} 人", grade, count);
    }

    // 创建课程
    utils::print_separator("课程信息");
    let courses = vec![
        Course::new(1, "Rust 程序设计", 4.0),
        Course::new(2, "数据结构", 3.0),
        Course::new(3, "操作系统", 3.5),
    ];
    for course in &courses {
        println!("  {}", course);
    }

    // 演示错误模块
    utils::print_separator("错误类型展示");
    let err1 = errors::StudentError::NotFound(String::from("小李"));
    let err2 = errors::StudentError::InvalidScore(105.0);
    let err3 = errors::StudentError::DuplicateId(1);
    let err4 = errors::StudentError::EmptyName;
    println!("  {}", err1);
    println!("  {}", err2);
    println!("  {}", err3);
    println!("  {}", err4);
}

// ============================================================
//  main 函数 —— 程序入口
// ============================================================
fn main() {
    println!("============================================================");
    println!("  第 11 课：模块系统 —— 给代码建一个整洁的文件柜");
    println!("============================================================");

    // 调用各辅助函数，演示模块系统的各个方面
    demo_basic_modules();     // 基本模块用法
    demo_visibility();        // 可见性演示
    demo_path_prefixes();     // self / super 路径
    demo_re_export();         // pub use 重导出
    demo_student_system();    // 实战：学生管理系统

    // 总结
    utils::print_separator("课程总结");
    println!("  模块 = 文件柜的抽屉，把相关代码分门别类");
    println!("  pub = 公开的抽屉，谁都能看");
    println!("  私有 = 锁着的抽屉，只有自己能看");
    println!("  pub(crate) = 只给家人看的抽屉");
    println!("  use = 从抽屉里拿出文件放到桌上");
    println!("  pub use = 把深层的东西复印一份放到桌面上");
    println!("  self = 当前抽屉");
    println!("  super = 上一级抽屉");
    println!("  crate = 整个文件柜");
    println!();
    println!("  多文件模块对应关系：");
    println!("    mod math_utils;  →  src/math_utils.rs");
    println!("    mod models;      →  src/models/mod.rs 或 src/models.rs");
    println!("    mod models 中 pub mod user;  →  src/models/user.rs");
    println!();
    println!("  实际项目中的文件结构：");
    println!("    src/");
    println!("    ├── main.rs          ← 程序入口");
    println!("    ├── errors.rs        ← 自定义错误");
    println!("    ├── utils.rs         ← 工具函数");
    println!("    └── models/");
    println!("        ├── mod.rs       ← 模型模块入口");
    println!("        ├── user.rs      ← 用户/学生模型");
    println!("        └── course.rs    ← 课程模型");
    println!();
    println!("  下节课预告：并发编程 —— 让程序同时做多件事！");
    println!("============================================================");
}

// ============================================================
//  练习题
// ============================================================
//
//  ★ 基础巩固（5-10 分钟）
//  ─────────────────────────
//  在单文件中用 mod 定义一个 geometry 模块，包含三个子模块：
//  - circle：计算圆的面积和周长
//  - rectangle：计算矩形的面积和周长
//  - triangle：计算三角形的面积
//
//  要求：
//  1. 每个子模块中的计算函数必须是 pub 的
//  2. 用 use 导入后在 main 中调用
//  3. 打印结果
//
//  提示：
//  ```
//  mod geometry {
//      pub mod circle {
//          pub fn area(radius: f64) -> f64 {
//              std::f64::consts::PI * radius * radius
//          }
//          // ... 周长函数
//      }
//      pub mod rectangle { ... }
//      pub mod triangle { ... }
//  }
//  ```

// ===== 测试版练习 =====
// 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
// 取消下面的注释，运行 cargo test --example lesson11

/*
mod geometry {
    pub mod circle {
        pub fn area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
        pub fn circumference(radius: f64) -> f64 {
            2.0 * std::f64::consts::PI * radius
        }
    }
    pub mod rectangle {
        pub fn area(width: f64, height: f64) -> f64 {
            width * height
        }
        pub fn perimeter(width: f64, height: f64) -> f64 {
            2.0 * (width + height)
        }
    }
    pub mod triangle {
        pub fn area(base: f64, height: f64) -> f64 {
            0.5 * base * height
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_modules() {
        // 测试圆的面积和周长
        let r = 5.0;
        let expected_area = std::f64::consts::PI * 25.0;
        assert!((geometry::circle::area(r) - expected_area).abs() < 1e-10);
        let expected_circ = 2.0 * std::f64::consts::PI * 5.0;
        assert!((geometry::circle::circumference(r) - expected_circ).abs() < 1e-10);

        // 测试矩形的面积和周长
        assert_eq!(geometry::rectangle::area(4.0, 6.0), 24.0);
        assert_eq!(geometry::rectangle::perimeter(4.0, 6.0), 20.0);

        // 测试三角形的面积
        assert_eq!(geometry::triangle::area(6.0, 4.0), 12.0);
    }
}
*/
//
//  ─────────────────────────────────────────────────────────
//
//  ★★ 应用练习（15-20 分钟）
//  ─────────────────────────
//  创建一个 permissions 模块，演示 pub / pub(crate) / 私有的区别。
//
//  要求：
//  1. 模块中包含：
//     - 一个私有函数 secret()
//     - 一个 pub 函数 public_api()
//     - 一个 pub(crate) 函数 internal_api()
//     - 一个 pub 子模块 inner，其中有一个函数 call_all()
//  2. 在 call_all() 中尝试调用上述三个函数，哪些能编译通过？
//  3. 在 main 中尝试调用上述三个函数，哪些能编译通过？
//  4. 在注释中标注哪些调用会编译错误，以及为什么
//
//  ─────────────────────────────────────────────────────────
//
//  ★★★ 进阶挑战（选做）
//  ─────────────────────────
//  用 pub use 重导出，实现一个"简化版"的模块公共 API。
//
//  要求：
//  1. 创建一个 app 模块，内部结构很深（至少 3 层嵌套）
//  2. 内部模块包含：user 管理、order 管理、payment 处理
//  3. 用 pub use 把常用函数重导出到 app 层
//  4. 外部调用者只需要 app::xxx() 即可，不需要知道内部结构
//  5. 写一个 main 函数演示简化后的调用
//
//  思考：这种重导出模式在大型项目中有什么好处？
//
// ============================================================
//  多文件模块实战（补充说明）
// ============================================================
//
//  如果你想把本课的代码拆成真正的多文件项目，步骤如下：
//
//  1. 创建项目：
//     cargo new student_system
//
//  2. 创建文件结构：
//     student_system/
//     └── src/
//         ├── main.rs
//         ├── errors.rs
//         ├── utils.rs
//         └── models/
//             ├── mod.rs
//             ├── user.rs
//             └── course.rs
//
//  3. 把 errors 模块的内容剪切到 src/errors.rs（去掉 mod errors { } 外壳）
//
//  4. 把 models 的 pub mod user / pub mod course 声明放到 src/models/mod.rs
//
//  5. 把 user 子模块的内容剪切到 src/models/user.rs（去掉 pub mod user { } 外壳）
//
//  6. 把 course 子模块的内容剪切到 src/models/course.rs
//
//  7. 把 utils 模块的内容剪切到 src/utils.rs
//
//  8. 在 main.rs 中用 mod 声明：
//     mod errors;
//     mod models;
//     mod utils;
//
//  9. 用 use 导入需要的类型和函数
//
//  这样就完成了从单文件到多文件的重构！
//
// ============================================================
//  核心收获：
//  - 模块（mod）是 Rust 组织代码的基本单位，就像文件柜的抽屉
//  - pub / pub(crate) / 私有 控制代码的可见性，保护内部实现
//  - pub use 重导出可以简化深层模块的对外接口，解耦内部结构
//
//  常见陷阱：
//  - 忘记给需要外部访问的函数/结构体加 pub，导致编译错误
//  - 混淆两种目录风格（mod.rs vs 同名文件），导致 Rust 找不到模块
//
//  下节课预告：
//  - 并发编程 —— 让程序同时做多件事，像多条流水线并行工作
// ============================================================
