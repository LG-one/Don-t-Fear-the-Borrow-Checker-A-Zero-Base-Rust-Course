// ============================================================================
// 第 04 课：结构体与方法
// ============================================================================
//
// 【课程目标】
// 1. 理解结构体（struct）是什么，为什么需要它
// 2. 学会定义结构体、创建实例、访问和修改字段
// 3. 掌握结构体的三种变体：经典结构体、元组结构体、单元结构体
// 4. 学会用 impl 块为结构体定义方法和关联函数
// 5. 理解 self 的三种形式：&self / &mut self / self
// 6. 学会用 #[derive(Debug)] 调试打印结构体
// 7. 了解 Drop trait 和析构顺序
//
// 【前置知识】
// - 第 00 课：环境搭建与第一个程序
// - 第 01 课：变量与数据类型
// - 第 02 课：控制流与函数
// - 第 03 课：所有权系统
//
// 【预计学习时间】
// - 概念讲解 + 代码阅读：40-50 分钟
// - 练习题：30-45 分钟
//
// ============================================================================

// ---- 辅助结构体定义 ----
// 这些结构体在 main 之前定义，供 main 和辅助函数使用

// 【生活类比：学生证】
// 想象你有一张学生证，上面印着：姓名、年龄、班级。
// 这三个信息属于"同一个人"，应该打包在一起，而不是散落在各处。
// 结构体就是这样一个"打包工具"——把相关的数据组合成一个有意义的整体。
//
// 对比之前的做法：
//   let name = String::from("张三");
//   let age = 18;
//   let class = String::from("计算机1班");
//   // 三个变量散落各处，容易搞混
//
// 有了结构体：
//   let student = Student { name, age, class };
//   // 一个变量，清清楚楚

/// 一个最简单的学生结构体
/// struct 关键字后面跟结构体名称（大驼峰命名，如 StudentInfo、MyData）
/// 花括号里是"字段"（field），每个字段有名字和类型
struct StudentInfo {
    name: String,   // 姓名：用 String（拥有所有权），不用 &str（引用）
    age: u32,       // 年龄：无符号整数，年龄不会是负数
    class: String,  // 班级：同样用 String
}

// 【字段初始化简写】
// 当变量名和字段名相同时，可以省略 `name: name`，直接写 `name`
// 这个结构体用于演示这个语法
struct Point {
    x: f64,
    y: f64,
}

// 【元组结构体】
// 有时候字段名不重要，只需要把几个值捆在一起。
// 就像你去奶茶店点单，店员只需要知道"三个数字"（甜度、冰度、大小），
// 不需要给每个数字起名字。
// 语法：struct 名字(类型1, 类型2, ...);
struct Color(u8, u8, u8);         // RGB 颜色，三个 u8 值
struct Meters(f64);               // 一个包装类型，表示"米"
struct Seconds(f64);              // 一个包装类型，表示"秒"

// 【单元结构体】
// 没有任何字段的结构体。听起来没用？其实它常用于实现某个"行为"（trait），
// 而不需要存储数据。就像一个"打卡机"——它本身不需要记住谁打了卡，
// 它只需要提供"打卡"这个功能。
struct Marker;

// 【Drop trait 简单示例用的结构体】
// Drop 是 Rust 的一个内置 trait，当值被销毁时自动调用 drop 方法。
// 类似于 C++ 的析构函数，但 Rust 是自动调用的（不需要手动 delete）。
struct Droppable {
    name: String,
}

// impl 块可以有多个！不一定要把所有方法写在一起。
// 这里单独为 Droppable 实现 Drop trait。
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("  >>> {} 被销毁了（Drop 被调用）", self.name);
    }
}

// ============================================================================
// 第一部分：结构体的定义与实例化
// ============================================================================

/// 演示：定义结构体、创建实例、访问和修改字段
fn demo_struct_basics() {
    println!("========== 第一部分：结构体基础 ==========\n");

    // ---- 1. 创建结构体实例 ----
    // 使用 结构体名 { 字段名: 值, ... } 的语法
    // 所有字段都必须赋值，Rust 不允许有未初始化的字段
    let student = StudentInfo {
        name: String::from("张三"),    // String::from 把字符串字面量转为 String
        age: 18,
        class: String::from("计算机1班"),
    };

    // ---- 2. 访问字段 ----
    // 用点号（.）访问字段，就像访问对象的属性
    println!("姓名：{}", student.name);
    println!("年龄：{}", student.age);
    println!("班级：{}", student.class);

    // ---- 3. 修改字段 ----
    // 注意：变量本身必须是 mut（可变的），才能修改字段
    // 这和普通变量一样——Rust 默认不可变
    let mut student2 = StudentInfo {
        name: String::from("李四"),
        age: 19,
        class: String::from("数学1班"),
    };
    println!("\n修改前：{} 的年龄是 {}", student2.name, student2.age);

    student2.age = 20;  // 修改单个字段
    student2.class = String::from("物理1班");  // 修改另一个字段
    println!("修改后：{} 的年龄是 {}，班级是 {}", student2.name, student2.age, student2.class);

    // ---- 4. 从旧实例构建新实例 ----
    // 如果你想创建一个新实例，只有少数字段不同，可以用"结构体更新语法"
    // 语法：..旧实例
    // 意思是："其余字段从旧实例中取值"
    let student3 = StudentInfo {
        name: String::from("王五"),     // 新名字
        ..student2                      // age 和 class 从 student2 拿
        // 等价于：age: student2.age, class: student2.class
    };
    println!("\nstudent3：{}，{}岁，{}", student3.name, student3.age, student3.class);

    // 【重要警告：Move 语义！】
    // 上面的 ..student2 会 Move student2 中没有被显式指定的字段。
    // student2.age 是 Copy 类型（u32），所以 student2.age 仍然可用。
    // 但如果字段是 String（非 Copy），那个字段就被 Move 走了，原实例不能再用那个字段。
    //
    // 实际上，student2.class（String 类型）已经被 Move 到 student3 了。
    // 下面这行如果取消注释，会报错：value borrowed here after move
    // println!("{}", student2.class);  // 编译错误！

    // 但 student2.age（u32，Copy 类型）仍然可以用
    println!("student2 的年龄仍然可用：{}", student2.age);
    // student2.name 仍然可用，因为它没有被 Move（name 是显式指定的，不是从 ..student2 来的）
    println!("student2 的姓名仍然可用：{}", student2.name);

    // 【规则总结】
    // ..旧实例 会 Move 所有"没有显式赋值"的非 Copy 字段。
    // 如果你之后还想用旧实例，要小心！
    // 简单做法：只在确定不再使用旧实例时才用更新语法。

    println!();
}

// ============================================================================
// 第二部分：字段初始化简写
// ============================================================================

/// 演示：当变量名和字段名相同时的简写语法
fn demo_field_init_shorthand() {
    println!("========== 第二部分：字段初始化简写 ==========\n");

    // 【常规写法】
    let x = 3.0;
    let y = 4.0;
    let p1 = Point { x: x, y: y };  // 字段名: 变量名，名字相同，显得啰嗦

    // 【简写写法】
    // 当变量名和字段名完全相同时，可以省略冒号和值
    let p2 = Point { x, y };  // 等价于 Point { x: x, y: y }

    println!("p1 = ({}, {})", p1.x, p1.y);
    println!("p2 = ({}, {})", p2.x, p2.y);

    // 【什么时候不能简写？】
    // 当变量名和字段名不同时，必须写完整
    let my_x = 10.0;
    let my_y = 20.0;
    let p3 = Point { x: my_x, y: my_y };  // 不能简写，因为 my_x != x
    println!("p3 = ({}, {})", p3.x, p3.y);

    // 【实际开发中】
    // 这个简写在函数参数中特别常见，比如：
    //   fn create_point(x: f64, y: f64) -> Point {
    //       Point { x, y }  // 参数名和字段名一样，直接简写
    //   }
    // 后面的关联函数部分会大量用到。

    println!();
}

// ============================================================================
// 第三部分：元组结构体与单元结构体
// ============================================================================

/// 演示：元组结构体和单元结构体
fn demo_tuple_and_unit_structs() {
    println!("========== 第三部分：元组结构体与单元结构体 ==========\n");

    // ---- 元组结构体 ----
    // 元组结构体的字段没有名字，只有类型。用 .0 .1 .2 访问。

    // 创建一个颜色值（RGB）
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);

    // 访问元组结构体的字段：用 .0 .1 .2（和元组一样）
    println!("红色：R={}, G={}, B={}", red.0, red.1, red.2);
    println!("绿色：R={}, G={}, B={}", green.0, green.1, green.2);
    println!("蓝色：R={}, G={}, B={}", blue.0, blue.1, blue.2);

    // 【元组结构体的真正用途：Newtype 模式】
    // 用元组结构体包装一个类型，创建一个"新类型"。
    // 好处：编译器会区分不同的类型，防止混用。
    //
    // 比如，Meters 和 Seconds 都是 f64，但"5米"和"5秒"不能相加！
    // 如果都用 f64，编译器不会阻止你把米和秒加在一起。
    // 但用了元组结构体后，类型不同，编译器会报错。

    let distance = Meters(100.0);
    let time = Seconds(9.58);

    println!("\n距离：{} 米", distance.0);
    println!("时间：{} 秒", time.0);

    // 下面这行如果取消注释，会报错：类型不匹配
    // let wrong = distance.0 + time.0;  // 虽然底层都是 f64，但类型不同
    // 正确做法：先取出内部值再计算
    let speed = distance.0 / time.0;
    println!("速度：{:.2} 米/秒", speed);

    // ---- 单元结构体 ----
    // 没有任何字段。用于实现 trait 但不需要数据的场景。
    // 现在先知道有这个东西就好，后面 trait 章节会深入讲解。

    let _m = Marker;  // 创建实例不需要花括号
    println!("\n单元结构体 Marker 创建成功（它没有字段，所以没什么好打印的）");

    println!();
}

// ============================================================================
// 第四部分：方法与 impl 块
// ============================================================================

/// 圆形结构体——用于演示方法定义
struct Circle {
    radius: f64,
}

/// 为 Circle 实现方法
/// impl 块 = "implementation"，意思是"实现"
/// 所有与 Circle 相关的方法都写在 impl Circle { ... } 里面
impl Circle {
    // ---- 方法（method）----
    // 方法的第一个参数是 &self（或 &mut self / self）
    // &self 是 &Circle 的简写，表示"借用这个 Circle，不拿走所有权"
    // 用点号调用：circle.area()

    /// 计算面积：π * r²
    /// &self 表示：我只需要"看看"这个圆的半径，不会修改它
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
        // self.radius 等价于 (*self).radius，但 Rust 会自动解引用
    }

    /// 计算周长：2 * π * r
    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    /// 判断是否能包含另一个圆（完全覆盖）
    /// &self：借用 self（调用者）
    /// other: &Circle：借用另一个圆
    fn can_contain(&self, other: &Circle) -> bool {
        self.radius > other.radius
    }

    // ---- 关联函数（associated function）----
    // 没有 &self 参数的函数叫"关联函数"
    // 它不属于某个实例，而是属于"这个类型本身"
    // 用双冒号调用：Circle::new(5.0)
    //
    // 【生活类比】
    // 方法 = 学生能做的事（学习、运动、吃饭）→ 需要一个具体的学生
    // 关联函数 = 工厂（造学生的地方）→ 不需要具体学生，而是"生产"学生

    /// 创建一个新圆（关联函数）
    /// 没有 &self！它是一个"工厂方法"，用来创建 Circle 实例
    fn new(radius: f64) -> Circle {
        Circle { radius }  // 字段初始化简写：radius: radius → radius
    }

    /// 创建一个单位圆（半径为1）——另一个关联函数
    fn unit() -> Circle {
        Circle { radius: 1.0 }
    }
}

/// 演示：方法定义与调用
fn demo_methods() {
    println!("========== 第四部分：方法与 impl 块 ==========\n");

    // ---- 用关联函数创建实例 ----
    // 注意：关联函数用 :: 调用（不是点号）
    let c1 = Circle::new(5.0);       // 调用关联函数 new
    let c2 = Circle::new(3.0);
    let c3 = Circle::unit();          // 调用关联函数 unit

    // ---- 用方法操作实例 ----
    // 注意：方法用 . 调用（不是双冒号）
    println!("c1 半径：{}", c1.radius);
    println!("c1 面积：{:.4}", c1.area());           // 调用方法 area
    println!("c1 周长：{:.4}", c1.circumference());   // 调用方法 circumference

    println!("\nc2 半径：{}", c2.radius);
    println!("c2 面积：{:.4}", c2.area());

    println!("\nc3（单位圆）半径：{}", c3.radius);
    println!("c3 面积：{:.4}", c3.area());

    // ---- 方法中借用其他实例 ----
    println!("\nc1 能包含 c2 吗？{}", c1.can_contain(&c2));
    println!("c2 能包含 c1 吗？{}", c2.can_contain(&c1));

    // 【总结：点号 vs 双冒号】
    // 点号（.）  → 方法调用，需要一个实例：instance.method()
    // 双冒号（::）→ 关联函数调用，属于类型本身：Type::function()
    // 类似于其他语言的"静态方法"概念

    println!();
}

// ============================================================================
// 第五部分：&self、&mut self、self 的区别
// ============================================================================

/// 一个可变的计数器，用于演示三种 self
struct Counter {
    count: u32,
}

impl Counter {
    /// 关联函数：创建新计数器
    fn new() -> Counter {
        Counter { count: 0 }
    }

    /// &self：不可变借用
    /// 只是"看看"计数器的值，不会改变它
    /// 可以多次借用（多个 &self 同时存在）
    fn get_count(&self) -> u32 {
        self.count
    }

    /// &mut self：可变借用
    /// 需要"修改"计数器的值
    /// 同一时间只能有一个 &mut self（防止数据竞争）
    fn increment(&mut self) {
        self.count += 1;
        // 注意：这里不能返回 self，因为只是借用
    }

    /// self：获取所有权
    /// 调用后，原来的变量就不能再用了！
    /// 常用于"消费"自身、返回自身（用于链式调用）
    fn into_inner(self) -> u32 {
        self.count  // self 被 Move 进来，函数结束后被销毁
    }
}

/// 演示三种 self 的区别
fn demo_self_types() {
    println!("========== 第五部分：&self / &mut self / self ==========\n");

    let mut counter = Counter::new();  // 需要 mut，因为后面要用 &mut self

    // &self：只读访问
    println!("初始值：{}", counter.get_count());  // 不修改，只是看

    // &mut self：可变访问
    counter.increment();  // 修改内部状态
    counter.increment();
    counter.increment();
    println!("增加 3 次后：{}", counter.get_count());

    // self：获取所有权（消费自身）
    let final_value = counter.into_inner();  // counter 被 Move 了！
    println!("最终值（已消费计数器）：{}", final_value);

    // 下面这行如果取消注释，会报错：value used here after move
    // counter.increment();  // 编译错误！counter 已经被 Move 了

    // 【类比理解】
    // &self      → 借书来看（看完还回去，别人还能看）
    // &mut self  → 借书来做笔记（你用的时候别人不能看，用完还回去）
    // self       → 把书送给你（书归你了，原主人就没有了）

    println!();
}

// ============================================================================
// 第六部分：#[derive(Debug)] 与调试打印
// ============================================================================

/// 加上 #[derive(Debug)] 后，结构体就能用 {:?} 打印了
/// Debug 是一个 trait，derive 会自动生成实现代码
/// 类似于自动帮学生证加上"打印"功能
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    /// 判断 self 是否能完全包含另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// 演示 Debug trait 的使用
fn demo_debug_trait() {
    println!("========== 第六部分：#[derive(Debug)] 调试打印 ==========\n");

    let rect = Rectangle::new(30.0, 50.0);

    // ---- {:?} 单行 Debug 输出 ----
    // 适合在 println! 中简短显示
    println!("rect = {:?}", rect);
    // 输出：rect = Rectangle { width: 30.0, height: 50.0 }

    // ---- {:#?} 多行 Debug 输出 ----
    // 适合在调试时详细查看，每个字段占一行
    println!("\nrect（美化输出）= {:#?}", rect);
    // 输出：
    // Rectangle {
    //     width: 30.0,
    //     height: 50.0,
    // }

    // ---- dbg! 宏 ----
    // dbg! 会打印文件名、行号、表达式和值，非常适合调试
    // 注意：dbg! 会获取值的所有权（对 Copy 类型无影响），并返回值
    let scale = 2.0;
    let scaled_rect = Rectangle::new(
        rect.width * scale,   // 注意：这里直接用了 rect 的字段值（Copy）
        rect.height * scale,
    );
    dbg!(&scaled_rect);  // 用 & 借用，不 Move
    // 输出类似：[src/main.rs:xxx] &scaled_rect = Rectangle { ... }

    // 【为什么需要 Debug？】
    // 结构体默认不能用 {}（Display）打印，也不能用 {:?}（Debug）打印。
    // Display 需要手动实现（后面会做），Debug 可以用 derive 自动生成。
    // 开发调试时，Debug 比 Display 更实用，因为它显示所有字段。

    println!();
}

// ============================================================================
// 第七部分：Drop trait 与析构顺序
// ============================================================================

/// 演示 Drop trait（析构函数）的行为
fn demo_drop() {
    println!("========== 第七部分：Drop trait（析构） ==========\n");

    // 【什么是 Drop？】
    // 当一个值离开作用域（被销毁）时，Rust 自动调用它的 drop 方法。
    // 类似于：你离开教室时，系统自动帮你"注销"。
    // 常用于：释放资源（关闭文件、断开网络连接等）。

    println!("--- 创建 droppable 实例 ---");
    let _a = Droppable { name: String::from("实例A（最先创建）") };
    let _b = Droppable { name: String::from("实例B") };
    let _c = Droppable { name: String::from("实例C（最后创建）") };
    println!("--- 三个实例已创建，即将离开作用域 ---\n");

    // 当函数结束时，_a、_b、_c 离开作用域，drop 被自动调用。
    // 析构顺序：与创建顺序相反！（后进先出，像栈一样）
    // 即：C 先被销毁，然后 B，最后 A。
    // 这很好理解：如果 A 依赖 B，B 依赖 C，那应该先销毁 C，再销毁 B，最后销毁 A。

    // 【手动提前销毁】
    // 如果你想提前销毁，可以用 drop() 函数：
    println!("--- 演示手动 drop ---");
    let d = Droppable { name: String::from("实例D（将被提前销毁）") };
    let _e = Droppable { name: String::from("实例E") };
    println!("手动销毁 d...");
    drop(d);  // d 被提前销毁！注意：这里用的是 std::mem::drop，不是 d.drop()
    println!("d 已销毁，e 还在。函数即将结束...\n");

    // 【为什么不能直接调用 d.drop()？】
    // 因为 Rust 会在函数结束时再次尝试 drop d，导致"双重释放"（double free）。
    // 所以 Rust 禁止直接调用 .drop()，必须用 std::mem::drop() 函数。

    // 函数结束：e 被自动 drop
}

// ============================================================================
// 第八部分：实战项目——学生管理系统
// ============================================================================

/// 学生结构体
/// 这是一个稍微复杂点的结构体，包含基本数据和一个动态数组（Vec）
/// Vec<f64> 表示一个可以动态增长的 f64 数组，用来存成绩
#[derive(Debug)]
struct Student {
    name: String,         // 学生姓名
    age: u32,             // 学生年龄
    scores: Vec<f64>,     // 成绩列表（动态数组）
}

/// 为 Student 实现各种方法
impl Student {
    // ---- 关联函数（工厂）----

    /// 创建一个新学生（没有成绩）
    fn new(name: &str, age: u32) -> Student {
        Student {
            name: String::from(name),  // &str 转 String
            age,
            scores: Vec::new(),        // 创建空的 Vec（动态数组）
        }
    }

    // ---- 方法（学生能做的事）----

    /// 添加一个成绩
    /// &mut self：需要修改内部的 scores，所以是可变借用
    fn add_score(&mut self, score: f64) {
        self.scores.push(score);  // push 在 Vec 末尾添加元素
    }

    /// 计算平均分
    /// &self：只读，不修改任何东西
    fn average(&self) -> f64 {
        if self.scores.is_empty() {
            return 0.0;  // 没有成绩时返回 0
        }
        // iter() 创建迭代器，sum() 求总和
        let total: f64 = self.scores.iter().sum();
        total / self.scores.len() as f64
        // len() 返回元素个数，as f64 转换为浮点数用于除法
    }

    /// 判断是否及格（平均分 >= 60）
    fn is_passed(&self) -> bool {
        self.average() >= 60.0
    }

    /// 获取成绩数量
    fn score_count(&self) -> usize {
        self.scores.len()
    }

    /// 获取最高分
    fn highest_score(&self) -> f64 {
        // iter().max() 返回 Option<&f64>
        // 因为 Vec 可能为空，所以返回 Option
        // unwrap_or 在 None 时返回默认值 0.0
        self.scores.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0)
    }

    /// 获取最低分
    fn lowest_score(&self) -> f64 {
        self.scores.iter().cloned().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0)
    }
}

// 【手动实现 Display trait】
// 之前用的是 #[derive(Debug)]，输出格式是给程序员看的（带字段名）。
// Display trait 是给用户看的，需要手动实现，格式完全自定义。
// 格式：impl std::fmt::Display for 类型名 { ... }
impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write! 宏的用法和 println! 类似，但写入的是 formatter
        write!(
            f,
            "学生信息：{} | 年龄：{} | 成绩数：{} | 平均分：{:.1} | {}",
            self.name,
            self.age,
            self.score_count(),
            self.average(),
            if self.is_passed() { "及格 ✓" } else { "不及格 ✗" }
        )
    }
}

/// 演示：学生管理系统的完整用法
fn demo_student_system() {
    println!("========== 第八部分：实战项目——学生管理系统 ==========\n");

    // ---- 创建学生 ----
    // 注意这里用了 &mut，因为后面要添加成绩
    let mut zhang_san = Student::new("张三", 20);
    let mut li_si = Student::new("李四", 21);
    let mut wang_wu = Student::new("王五", 19);

    // ---- 添加成绩 ----
    zhang_san.add_score(85.0);
    zhang_san.add_score(92.0);
    zhang_san.add_score(78.0);
    zhang_san.add_score(88.0);

    li_si.add_score(55.0);
    li_si.add_score(62.0);
    li_si.add_score(48.0);

    wang_wu.add_score(95.0);
    wang_wu.add_score(98.0);
    wang_wu.add_score(91.0);
    wang_wu.add_score(97.0);
    wang_wu.add_score(93.0);

    // ---- 使用 Display 打印 ----
    // 当实现了 Display trait 后，就可以用 {} 打印了
    println!("=== 学生成绩报告 ===\n");
    println!("{}", zhang_san);
    println!("{}", li_si);
    println!("{}", wang_wu);

    // ---- 使用 Debug 打印 ----
    // Debug 输出适合调试，能看到所有内部数据
    println!("\n=== Debug 详细信息 ===\n");
    println!("{:#?}", zhang_san);

    // ---- 使用各个方法 ----
    println!("\n=== 详细统计 ===\n");

    // 方法可以链式使用，也可以单独调用
    println!("{} 的平均分是：{:.2}", zhang_san.name, zhang_san.average());
    println!("{} 是否及格：{}", zhang_san.name, zhang_san.is_passed());
    println!("{} 的最高分：{:.1}", zhang_san.name, zhang_san.highest_score());
    println!("{} 的最低分：{:.1}", zhang_san.name, zhang_san.lowest_score());

    println!();
    println!("{} 的平均分是：{:.2}", li_si.name, li_si.average());
    println!("{} 是否及格：{}", li_si.name, li_si.is_passed());

    println!();
    println!("{} 的平均分是：{:.2}", wang_wu.name, wang_wu.average());
    println!("{} 是否及格：{}", wang_wu.name, wang_wu.is_passed());
    println!("{} 的最高分：{:.1}", wang_wu.name, wang_wu.highest_score());
    println!("{} 的最低分：{:.1}", wang_wu.name, wang_wu.lowest_score());

    // ---- 用结构体更新语法创建"同名不同龄"的学生 ----
    println!("\n=== 结构体更新语法演示 ===\n");
    let mut new_student = Student {
        name: String::from("赵六"),
        ..Student::new("占位", 0)  // 其余字段从临时实例取
    };
    // 注意：上面的写法只是为了演示，实际开发中直接用 new() 更清晰
    new_student.add_score(70.0);
    println!("{}", new_student);

    println!();
}

// ============================================================================
// 主函数：按顺序运行所有演示
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║       第 04 课：结构体与方法                      ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    // 第一部分：结构体基础（定义、实例化、访问、修改、更新语法）
    demo_struct_basics();

    // 第二部分：字段初始化简写
    demo_field_init_shorthand();

    // 第三部分：元组结构体与单元结构体
    demo_tuple_and_unit_structs();

    // 第四部分：方法与 impl 块
    demo_methods();

    // 第五部分：&self、&mut self、self 的区别
    demo_self_types();

    // 第六部分：#[derive(Debug)] 与调试打印
    demo_debug_trait();

    // 第七部分：Drop trait 与析构顺序
    demo_drop();

    // 第八部分：实战项目——学生管理系统
    demo_student_system();

    // 练习题说明
    println!("╔══════════════════════════════════════════════════╗");
    println!("║                    练习题                         ║");
    println!("╠══════════════════════════════════════════════════╣");
    println!("║                                                  ║");
    println!("║  练习 1（基础，5-10分钟）：                        ║");
    println!("║  创建 Circle 结构体，实现 area() 和               ║");
    println!("║  circumference() 方法。                           ║");
    println!("║  提示：可以参考上面的 Circle 实现                  ║");
    println!("║                                                  ║");
    println!("║  练习 2（应用，15-20分钟）：                       ║");
    println!("║  创建 Rectangle 和 Point 结构体，                 ║");
    println!("║  实现 contains(&Point) 方法，                     ║");
    println!("║  判断一个点是否在矩形内部。                        ║");
    println!("║  提示：矩形左上角 + 宽高，判断点的 x/y             ║");
    println!("║  是否在范围内                                      ║");
    println!("║                                                  ║");
    println!("║  练习 3（进阶挑战，选做）：                        ║");
    println!("║  实现方法链 QueryBuilder：                        ║");
    println!("║  QueryBuilder::new(\"users\")                      ║");
    println!("║      .where_clause(\"age > 18\")                   ║");
    println!("║      .order_by(\"name\")                           ║");
    println!("║      .limit(10)                                   ║");
    println!("║      .build()                                     ║");
    println!("║  返回一个 SQL 字符串。                             ║");
    println!("║  提示：每个方法返回 self（获取所有权后返回）        ║");
    println!("║                                                  ║");
    println!("╚══════════════════════════════════════════════════╝");

    println!("\n运行结束！请尝试完成练习题，巩固今天学到的知识。\n");

    // ---- 练习题参考答案调用 ----
    // 请先自己尝试完成练习，再取消注释查看参考答案！
    // 取消下面三行的注释即可运行参考答案：
    // exercise_1();
    // exercise_2();
    // exercise_3();
}

// ============================================================================
// 练习题参考答案（先自己尝试，再来看！）
// ============================================================================

// ===== 测试版练习 =====
// 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
// 取消下面的注释，运行 cargo test --example 04_结构体与方法

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise1_circle_area_and_circumference() {
        // 测试练习 1 的核心功能：Circle 的 area() 和 circumference()
        struct CircleTest {
            radius: f64,
        }
        impl CircleTest {
            fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius.powi(2)
            }
            fn circumference(&self) -> f64 {
                2.0 * std::f64::consts::PI * self.radius
            }
        }
        let c = CircleTest { radius: 5.0 };
        assert!((c.area() - 78.53981633974483).abs() < 1e-10);
        assert!((c.circumference() - 31.41592653589793).abs() < 1e-10);
    }
}
*/

// ---- 练习 1 参考答案 ----

/// 练习 1：Circle 结构体
/// 在 main 中调用：exercise_1();
fn exercise_1() {
    println!("===== 练习 1：Circle =====\n");

    // 定义结构体（可以在函数内部定义，也可以在外面定义）
    // 这里为了方便，在函数内部定义并实现
    struct CircleEx {
        radius: f64,
    }

    impl CircleEx {
        fn new(radius: f64) -> CircleEx {
            CircleEx { radius }
        }

        /// 面积 = π * r²
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius.powi(2)
            // powi(2) 是 "power integer"，即平方
        }

        /// 周长 = 2 * π * r
        fn circumference(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }

    let c = CircleEx::new(5.0);
    println!("半径：{}", c.radius);
    println!("面积：{:.4}", c.area());
    println!("周长：{:.4}", c.circumference());
    println!();
}

// ---- 练习 2 参考答案 ----

/// 练习 2：Rectangle + Point，判断点是否在矩形内
/// 在 main 中调用：exercise_2();
fn exercise_2() {
    println!("===== 练习 2：Rectangle + Point =====\n");

    #[derive(Debug)]
    struct PointEx {
        x: f64,
        y: f64,
    }

    #[derive(Debug)]
    struct RectangleEx {
        // 左上角坐标
        left: f64,
        top: f64,
        // 宽高
        width: f64,
        height: f64,
    }

    impl RectangleEx {
        fn new(left: f64, top: f64, width: f64, height: f64) -> RectangleEx {
            RectangleEx { left, top, width, height }
        }

        /// 判断点是否在矩形内（包含边界）
        /// 矩形范围：x ∈ [left, left+width]，y ∈ [top, top+height]
        /// 注意：这里假设 y 轴向下为正（屏幕坐标系）
        /// 如果是数学坐标系（y 向上），判断条件需要调整
        fn contains(&self, point: &PointEx) -> bool {
            point.x >= self.left
                && point.x <= self.left + self.width
                && point.y >= self.top
                && point.y <= self.top + self.height
        }
    }

    let rect = RectangleEx::new(0.0, 0.0, 100.0, 50.0);
    let p1 = PointEx { x: 50.0, y: 25.0 };   // 中心点
    let p2 = PointEx { x: 150.0, y: 25.0 };  // 右边外面
    let p3 = PointEx { x: 0.0, y: 0.0 };     // 左上角（边界上）

    println!("矩形：{:?}", rect);
    println!("点 {:?} 在矩形内？{}", p1, rect.contains(&p1));
    println!("点 {:?} 在矩形内？{}", p2, rect.contains(&p2));
    println!("点 {:?} 在矩形内？{}", p3, rect.contains(&p3));
    println!();
}

// ---- 练习 3 参考答案 ----

/// 练习 3：方法链 QueryBuilder
/// 在 main 中调用：exercise_3();
fn exercise_3() {
    println!("===== 练习 3：QueryBuilder 方法链 =====\n");

    /// SQL 查询构建器
    /// 每个方法获取 self 的所有权，然后返回修改后的 self
    /// 这就是"方法链"（method chaining / builder pattern）的核心
    struct QueryBuilder {
        table: String,
        conditions: Vec<String>,  // WHERE 条件可以有多个
        order: Option<String>,    // Option 表示"可能有，可能没有"
        limit_val: Option<usize>,
    }

    impl QueryBuilder {
        /// 创建新的查询构建器（关联函数）
        fn new(table: &str) -> QueryBuilder {
            QueryBuilder {
                table: String::from(table),
                conditions: Vec::new(),
                order: None,
                limit_val: None,
            }
        }

        /// 添加 WHERE 条件
        /// self（不是 &self）：获取所有权，修改后返回
        fn where_clause(mut self, condition: &str) -> QueryBuilder {
            self.conditions.push(String::from(condition));
            self  // 返回修改后的 self，支持链式调用
        }

        /// 设置排序
        fn order_by(mut self, field: &str) -> QueryBuilder {
            self.order = Some(String::from(field));
            self
        }

        /// 设置返回条数限制
        fn limit(mut self, count: usize) -> QueryBuilder {
            self.limit_val = Some(count);
            self
        }

        /// 构建最终的 SQL 字符串
        /// 这里用 self（获取所有权），调用 build 后 QueryBuilder 就不能用了
        fn build(self) -> String {
            let mut sql = format!("SELECT * FROM {}", self.table);

            // 添加 WHERE 子句
            if !self.conditions.is_empty() {
                sql.push_str(" WHERE ");
                sql.push_str(&self.conditions.join(" AND "));
                // join 用 " AND " 连接所有条件
            }

            // 添加 ORDER BY
            if let Some(ref order) = self.order {
                // if let 用于匹配 Option：如果是 Some，取出值
                sql.push_str(&format!(" ORDER BY {}", order));
            }

            // 添加 LIMIT
            if let Some(limit) = self.limit_val {
                sql.push_str(&format!(" LIMIT {}", limit));
            }

            sql.push(';');  // SQL 语句以分号结尾
            sql
        }
    }

    // ---- 使用 QueryBuilder ----
    let query = QueryBuilder::new("users")
        .where_clause("age > 18")         // 第一个条件
        .where_clause("status = 'active'") // 第二个条件
        .order_by("name")                  // 按名字排序
        .limit(10)                         // 最多返回 10 条
        .build();                          // 构建 SQL

    println!("生成的 SQL：\n{}\n", query);
    // 输出：SELECT * FROM users WHERE age > 18 AND status = 'active' ORDER BY name LIMIT 10;

    // 也可以不设置所有选项
    let simple_query = QueryBuilder::new("products")
        .where_clause("price < 100")
        .build();

    println!("简单查询：\n{}\n", simple_query);
    // 输出：SELECT * FROM products WHERE price < 100;

    let all_query = QueryBuilder::new("orders")
        .order_by("created_at")
        .limit(50)
        .build();

    println!("全部订单查询：\n{}\n", all_query);
    // 输出：SELECT * FROM orders ORDER BY created_at LIMIT 50;
}

/*
 * ============================================================================
 * 核心收获：
 * - 结构体（struct）是 Rust 中自定义数据类型的基础，用于把相关数据打包在一起
 * - 方法定义在 impl 块中：&self（只读）、&mut self（可变）、self（获取所有权）
 * - 关联函数没有 self 参数，用 :: 调用，常用于构造函数（如 new()）
 *
 * 常见陷阱：
 * - 结构体更新语法（..old）会 Move 非 Copy 字段，之后旧实例的那些字段不可用
 * - 字段默认私有（模块级别），同模块内可以直接访问，跨模块需要 pub 和方法
 *
 * 下节课预告：
 * - 生命周期入门——解决"结构体里能放引用吗"的问题，让编译器帮你追踪引用的有效期
 * ============================================================================
 */
