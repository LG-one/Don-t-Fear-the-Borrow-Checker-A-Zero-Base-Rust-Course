// ============================================================================
// 第 08 课：泛型与 Trait —— 写一次代码，适用无数种类型
// ============================================================================
//
// 学习目标：
// 1. 理解泛型的概念，学会编写"万能"的函数和结构体
// 2. 掌握 Trait 的定义与实现，理解 Rust 的"能力系统"
// 3. 学会使用 Trait 对象实现多态
// 4. 完成图形计算器实战项目
//
// 前置知识：变量、控制流、所有权、结构体、生命周期、枚举、错误处理
//
// 核心比喻：
//   泛型 = 万能盒子：不管装什么，盒子的形状都能自动适应
//   Trait = 能力证书：有了驾照就能开车，有了 Trait 就能做某件事
//   Trait 对象 = 通用遥控器：不管你家是什么品牌的电视，都能控制

// ============================================================================
// 第一部分：泛型函数 —— 一个函数，处理任意类型
// ============================================================================

/// 先看一个"笨笨的"问题：
/// 假设你要写一个函数，找出一组数字中最大的那个。
/// 如果只有 i32，很简单。但如果还要支持 f64、i64 呢？
/// 难道要写三个几乎一模一样的函数？
///
/// 生活类比：
///   想象你是一个快递分拣员。你不需要为"装鞋子的包裹"写一套规则，
///   为"装书的包裹"写另一套规则。你只需要说"比较重量，选最重的"，
///   不管里面装的是什么，规则都一样。
///   泛型就是这个"不管装什么"的能力。

// ---------- 不用泛型的笨办法（了解即可） ----------

/// 找出 i32 切片中最大的值
/// 这个函数只能处理 i32 类型
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// 找出 f64 切片中最大的值
/// 看，代码几乎一模一样，只是类型不同！
fn largest_f64(list: &[f64]) -> &f64 {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ---------- 用泛型的聪明办法 ----------

/// 泛型版本：一个函数搞定所有可比较的类型！
///
/// 语法解读：
///   <T: PartialOrd> 表示：T 是一个类型参数，它必须满足 PartialOrd 这个 Trait
///   PartialOrd 的意思是可以用 <, >, <=, >= 来比较大小
///   就像"能力证书"：我不管你是什么类型，但你必须能被比较大小
///
/// 生活类比：
///   这就像公司的招聘要求："不管你是哪个学校毕业的，
///   只要你有程序员证书（Trait），你就能来面试（调用这个函数）"
fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// 演示泛型函数的使用
fn demo_generic_function() {
    println!("===== 泛型函数 =====\n");

    // 用笨办法：两个函数
    let numbers_i32 = vec![34, 50, 25, 100, 65];
    let numbers_f64 = vec![3.14, 2.71, 1.41, 9.99];

    println!("i32 最大值: {}", largest_i32(&numbers_i32));
    println!("f64 最大值: {}", largest_f64(&numbers_f64));

    // 用泛型：一个函数搞定！
    // Rust 会自动推导出 T 的具体类型（叫做"单态化"）
    println!("\n用泛型函数：");
    println!("i32 最大值: {}", largest_generic(&numbers_i32));
    println!("f64 最大值: {}", largest_generic(&numbers_f64));

    // 字符也能比较大小（按字典序）
    let words = vec!["hello", "world", "apple", "zebra"];
    println!("单词最大值: {}", largest_generic(&words));

    // 注意：泛型不是魔法！它在编译时会被替换成具体类型的代码
    // 这叫做"单态化"（monomorphization）：
    //   编译器看到你用 i32 和 f64 调用了 largest_generic
    //   就自动生成 largest_i32 和 largest_f64 两个版本
    //   所以泛型没有任何运行时开销！跟手写两个函数一样快！
    println!("\n泛型的秘密：编译时自动生成具体类型的代码，零运行时开销！");
}

// ============================================================================
// 第二部分：泛型结构体 —— 一个结构体，装下任何类型
// ============================================================================

/// 二维坐标点：x 和 y 类型相同
///
/// 生活类比：
///   想象一个快递盒，盒子上写着"里面装的东西，x 和 y 必须是同一种"
///   比如 Point<i32> 就是"装整数坐标的盒子"
///   Point<f64> 就是"装浮点数坐标的盒子"
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

/// 二维坐标点：x 和 y 可以是不同类型
/// 使用两个泛型参数 T 和 U
///
/// 生活类比：
///   这个盒子更灵活了，左边格子装一种东西，右边格子可以装另一种
///   比如 Point<i32, f64> 可以表示"整数经度 + 浮点纬度"
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

/// 为泛型结构体实现方法
///
/// 注意语法：impl<T> 中的 <T> 是必须的
/// 它告诉 Rust："下面的 T 是一个类型参数，不是某个具体类型"
impl<T> Point<T> {
    /// 创建新的 Point
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    /// 获取 x 的引用
    fn x(&self) -> &T {
        &self.x
    }

    /// 获取 y 的引用
    fn y(&self) -> &T {
        &self.y
    }
}

/// 只为特定类型的 Point 实现方法
/// 比如只有 f64 类型的点才能计算到原点的距离
impl Point<f64> {
    /// 计算到原点 (0.0, 0.0) 的距离
    /// 只有 Point<f64> 才有这个方法
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// 为 MixedPoint 实现方法
/// 注意这里有两个类型参数 <T, U>
impl<T, U> MixedPoint<T, U> {
    /// 混合两个不同的点，取第一个点的 x 和第二个点的 y
    /// 返回一个新的 MixedPoint，类型可能和原来的都不一样
    fn mixup<T2, U2>(self, other: MixedPoint<T2, U2>) -> MixedPoint<T, U2> {
        MixedPoint {
            x: self.x,      // 来自 self
            y: other.y,     // 来自 other
        }
    }
}

/// 演示泛型结构体
fn demo_generic_struct() {
    println!("\n===== 泛型结构体 =====\n");

    // Point<T>：x 和 y 必须是同一类型
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);

    println!("整数点: {:?}", integer_point);
    println!("浮点点: {:?}", float_point);

    // 只有 Point<f64> 才有 distance_from_origin 方法
    println!("浮点点到原点的距离: {:.4}", float_point.distance_from_origin());

    // integer_point 没有 distance_from_origin 方法
    // 因为它不是 Point<f64>
    // 下面这行会报错：
    // println!("{}", integer_point.distance_from_origin()); // 编译错误！

    // MixedPoint<T, U>：x 和 y 可以是不同类型
    println!("\n混合类型点：");
    let p1 = MixedPoint::new(5, 10.4);
    let p2 = MixedPoint::new("Hello", 'c');
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    // mixup：混合两个不同的点
    let p3 = p1.mixup(p2);
    // p3 的 x 来自 p1（i32），y 来自 p2（char）
    println!("混合后的点: {:?}", p3);  // MixedPoint { x: 5, y: 'c' }
}

// ============================================================================
// 第三部分：Trait 定义与实现 —— 定义"能力证书"
// ============================================================================

/// Trait 是 Rust 中定义共享行为的方式
///
/// 生活类比：
///   想象"能力证书"系统：
///   - 驾照是一种 Trait，有了它你就能开车
///   - 英语四级证书是一种 Trait，有了它你就能证明英语水平
///   - 不同的人（结构体）可以拥有不同的证书组合
///
/// 在 Rust 中：
///   Trait 定义了一组方法的签名（方法名、参数、返回值）
///   任何类型只要实现了这些方法，就获得了这个 Trait 的"证书"

/// 定义一个 Trait：Summary（可摘要）
/// 就像定义一种证书的要求
///
/// 这个 Trait 要求实现者提供两个方法：
/// - summarize_author() -> String  （获取作者）
/// - summarize() -> String         （获取摘要）
trait Summary {
    /// 获取作者信息
    fn summarize_author(&self) -> String;

    /// 获取摘要内容
    /// 注意：这里没有方法体，说明这是"必须实现"的方法
    fn summarize(&self) -> String {
        // 默认实现：调用 summarize_author
        format!("(阅读更多来自 {} 的内容...)", self.summarize_author())
    }

    /// 带默认实现的方法：default_summary
    /// 如果不覆盖，就用这个默认版本
    fn default_summary(&self) -> String {
        String::from("[默认摘要]")
    }
}

/// 新闻文章结构体
struct NewsArticle {
    headline: String,   // 标题
    location: String,   // 地点
    author: String,     // 作者
    content: String,    // 内容
}

/// 为 NewsArticle 实现 Summary Trait
/// 就像给 NewsArticle 颁发"可摘要证书"
impl Summary for NewsArticle {
    /// 实现必须的方法：summarize_author
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    /// 覆盖默认的 summarize 方法，提供自己的版本
    fn summarize(&self) -> String {
        format!("{}, 作者 {} ({})", self.headline, self.author, self.location)
    }

    // default_summary 使用默认实现，不需要写
}

/// 推文结构体
struct Tweet {
    username: String,   // 用户名
    content: String,    // 内容
    reply: bool,        // 是否回复
    retweet: bool,      // 是否转发
}

/// 为 Tweet 实现 Summary Trait
impl Summary for Tweet {
    /// 实现必须的方法：summarize_author
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    /// 覆盖默认的 summarize 方法
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    // default_summary 同样使用默认实现
}

/// 演示 Trait 定义与实现
fn demo_trait_basic() {
    println!("\n===== Trait 定义与实现 =====\n");

    let article = NewsArticle {
        headline: String::from("Rust 语言发布新版本"),
        location: String::from("北京"),
        author: String::from("张三"),
        content: String::from("Rust 语言今日发布了 2024 年最新版本..."),
    };

    let tweet = Tweet {
        username: String::from("rust_fan"),
        content: String::from("Rust 太好用了！"),
        reply: false,
        retweet: false,
    };

    // 调用 summarize 方法
    println!("新闻摘要: {}", article.summarize());
    println!("推文摘要: {}", tweet.summarize());

    // 调用默认实现的 default_summary
    println!("\n默认摘要：");
    println!("新闻: {}", article.default_summary());
    println!("推文: {}", tweet.default_summary());

    // 注意：NewsArticle 和 Tweet 是完全不同的类型
    // 但它们都实现了 Summary Trait，所以都可以调用 summarize()
    // 这就是 Trait 的威力：让不同类型共享同一种行为！
}

// ============================================================================
// 第四部分：Trait 作为参数 —— 接受任何有"证书"的类型
// ============================================================================

/// 使用 impl Trait 语法：最简洁的写法
/// 这个函数接受任何实现了 Summary Trait 的类型
///
/// 语法糖含义：item: impl Summary
/// 意思是："给我任何类型，只要它有 Summary 证书"
///
/// 生活类比：
///   公司前台说："不管你是谁，只要有工牌（Trait），就能进大楼"
fn notify_impl(item: &impl Summary) {
    println!("【impl Trait 语法】紧急通知: {}", item.summarize());
}

/// 使用完整的 Trait Bound 语法：更灵活的写法
/// 这是 impl Trait 的"完整版"
///
/// 语法：<T: Summary> 声明 T 必须满足 Summary
/// 然后参数写 item: &T
///
/// 两种写法效果完全一样，Trait Bound 在需要更多约束时更强大
fn notify_bound<T: Summary>(item: &T) {
    println!("【Trait Bound 语法】紧急通知: {}", item.summarize());
}

/// 当需要多个参数类型不同时，impl Trait 更简洁
/// 这里 item1 和 item2 可以是不同的类型，只要都实现了 Summary
fn notify_two_impl(item1: &impl Summary, item2: &impl Summary) {
    println!("两条通知: {} | {}", item1.summarize(), item2.summarize());
}

/// 如果用 Trait Bound，强制两个参数必须是同一类型
fn notify_two_bound<T: Summary>(item1: &T, item2: &T) {
    println!("两条通知(同类型): {} | {}", item1.summarize(), item2.summarize());
}

/// 演示 Trait 作为参数
fn demo_trait_as_param() {
    println!("\n===== Trait 作为参数 =====\n");

    let article = NewsArticle {
        headline: String::from("重大新闻"),
        location: String::from("上海"),
        author: String::from("李四"),
        content: String::from("今天发生了一件大事..."),
    };

    let tweet = Tweet {
        username: String::from("news_lover"),
        content: String::from("大家快看新闻！"),
        reply: false,
        retweet: false,
    };

    // impl Trait 语法：简单直观
    notify_impl(&article);
    notify_impl(&tweet);

    // Trait Bound 语法：效果一样
    notify_bound(&article);
    notify_bound(&tweet);

    // 两个不同类型都可以传入
    notify_two_impl(&article, &tweet);

    // 注意：notify_two_bound 要求两个参数是同一类型
    // 下面这行会报错（article 和 tweet 类型不同）：
    // notify_two_bound(&article, &tweet); // 编译错误！
}

// ============================================================================
// 第五部分：where 子句与多重约束 —— 更复杂的"证书要求"
// ============================================================================

/// 有时一个类型参数需要满足多个 Trait
/// 直接写会很长很难看
///
/// 生活类比：
///   招聘要求："应聘者必须同时拥有程序员证书和英语四级证书"
///   在 Rust 中用 + 连接多个 Trait

/// 不好读的写法：泛型参数堆在一起
fn _long_version<T: std::fmt::Display + Clone, U: std::fmt::Debug + PartialOrd>(
    _t: &T,
    _u: &U,
) -> String {
    String::from("参数满足多个 Trait")
}

/// 好读的写法：使用 where 子句
/// 把约束条件移到函数签名后面，代码更清晰
///
/// 语法解读：
///   T: Display + Clone       表示 T 必须同时满足 Display 和 Clone
///   U: Debug + PartialOrd    表示 U 必须同时满足 Debug 和 PartialOrd
fn better_version<T, U>(_t: &T, _u: &U) -> String
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug + PartialOrd,
{
    String::from("参数满足多个 Trait（where 子句写法）")
}

/// 实际例子：打印任何能 Display 且能 Clone 的值
fn print_and_clone<T>(value: &T) -> T
where
    T: std::fmt::Display + Clone,
{
    println!("值为: {}", value);
    value.clone()
}

/// 演示 where 子句
fn demo_where_clause() {
    println!("\n===== where 子句与多重约束 =====\n");

    let number = 42;
    let cloned = print_and_clone(&number);
    println!("克隆结果: {}", cloned);

    let text = String::from("Hello Rust");
    let cloned_text = print_and_clone(&text);
    println!("克隆结果: {}", cloned_text);

    let result = better_version(&42, &3.14);
    println!("{}", result);
}

// ============================================================================
// 第六部分：常用标准库 Trait —— Rust 内置的"能力证书"
// ============================================================================

/// Rust 标准库预定义了很多常用的 Trait
/// 掌握它们能让你的类型更强大
///
/// 生活类比：
///   就像社会上有很多通用证书：
///   - 学历证书（Display）：能被人看懂
///   - 调试证书（Debug）：能被开发者检查
///   - 复印证书（Clone）：能被复制
///   - 身份证（Copy）：能被自动复制（更轻量）
///   - 等号证书（PartialEq）：能被比较是否相等

/// 用 derive 自动实现常见 Trait
/// #[derive(...)] 是 Rust 的魔法：让编译器帮你写实现代码！
#[derive(Debug, Clone, PartialEq)]
struct Student {
    name: String,
    age: u32,
    grade: String,
}

/// 手动实现 Display Trait
/// 因为 Display 无法自动 derive，需要自己写
impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}岁, {})", self.name, self.age, self.grade)
    }
}

/// 用 From Trait 实现类型转换
/// 这样可以从其他类型"变出"一个 Student
impl From<&str> for Student {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').collect();
        Student {
            name: parts[0].to_string(),
            age: parts[1].parse().unwrap_or(0),
            grade: parts[2].to_string(),
        }
    }
}

/// 演示标准库常用 Trait
fn demo_standard_traits() {
    println!("\n===== 常用标准库 Trait =====\n");

    // Debug: 用 {:?} 打印调试信息
    let student1 = Student {
        name: String::from("小明"),
        age: 18,
        grade: String::from("高三"),
    };
    println!("Debug 格式: {:?}", student1);

    // Display: 用 {} 打印人类可读信息
    println!("Display 格式: {}", student1);

    // Clone: 创建深拷贝
    let student2 = student1.clone();
    println!("克隆: {}", student2);

    // PartialEq: 用 == 比较是否相等
    println!("相等比较: {} == {} -> {}", student1, student2, student1 == student2);

    // From/Into: 类型转换
    // From 已经实现，自动获得 Into
    let student3: Student = "李四,17,高二".into();  // 使用 Into（From 的反向）
    println!("从字符串转换: {}", student3);

    // Copy Trait 说明：
    // Copy 是一个特殊的 Trait，实现了它，赋值时会自动复制而不是移动
    // 但 Copy 要求类型的所有字段都是 Copy 的
    // String 不是 Copy 的，所以 Student 不能 derive Copy
    // 但 i32, f64, bool, char 等基本类型都是 Copy 的
    let x: i32 = 42;
    let y = x;  // i32 是 Copy 的，所以 x 不会被移动
    println!("\ni32 是 Copy 的: x = {}, y = {} (两个都能用)", x, y);

    println!("\n常用 Trait 总结：");
    println!("  Debug      - 调试打印 {{:?}}");
    println!("  Display    - 用户友好的打印 {{}}");
    println!("  Clone      - 深拷贝 .clone()");
    println!("  Copy       - 自动复制（仅限简单类型）");
    println!("  PartialEq  - 相等比较 == / !=");
    println!("  From/Into  - 类型转换");
}

// ============================================================================
// 第七部分：Trait 对象 —— 通用遥控器
// ============================================================================

/// Trait 对象是 Rust 实现多态的方式
///
/// 生活类比：
///   想象一个"通用遥控器"：
///   - 你可以用它控制任何品牌的电视（Samsung、LG、Sony...）
///   - 遥控器不关心具体是什么品牌，只要电视能响应"开/关/音量"这些指令
///   - 这些"指令"就是 Trait 定义的方法
///
/// 在 Rust 中：
///   dyn Trait 就是这个"通用遥控器"
///   Box<dyn Trait> 是把它装在盒子里（堆上分配）

/// 定义一个 Drawable Trait（可绘制）
trait Drawable {
    /// 绘制自己
    fn draw(&self);
    /// 获取描述
    fn description(&self) -> String;
}

/// 圆形
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("  画一个半径为 {} 的圆形 ○", self.radius);
    }
    fn description(&self) -> String {
        format!("圆形(半径={})", self.radius)
    }
}

/// 矩形
struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("  画一个 {}x{} 的矩形 □", self.width, self.height);
    }
    fn description(&self) -> String {
        format!("矩形({}x{})", self.width, self.height)
    }
}

/// 三角形
struct Triangle {
    base: f64,
    height: f64,
}

impl Drawable for Triangle {
    fn draw(&self) {
        println!("  画一个底{}高{}的三角形 △", self.base, self.height);
    }
    fn description(&self) -> String {
        format!("三角形(底={},高={})", self.base, self.height)
    }
}

/// 演示 Trait 对象
fn demo_trait_object() {
    println!("\n===== Trait 对象（通用遥控器） =====\n");

    // 问题：我想在一个 Vec 里放不同类型的形状
    // 但 Vec<T> 要求所有元素类型相同...
    //
    // 解决方案：用 Trait 对象！
    // Box<dyn Drawable> 表示"一个装在盒子里的、实现了 Drawable 的某种类型"
    // dyn = dynamic（动态），运行时决定调用哪个版本的方法

    // 创建一个可以装任何 Drawable 的 Vec
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 3.0 }),
        Box::new(Triangle { base: 6.0, height: 4.0 }),
        Box::new(Circle { radius: 2.5 }),
    ];

    // 遍历所有形状，调用同一个方法
    // 这就是多态！不同类型，同一个接口
    println!("绘制所有形状：");
    for shape in &shapes {
        shape.draw();
    }

    // 获取所有形状的描述
    println!("\n形状列表：");
    for (i, shape) in shapes.iter().enumerate() {
        println!("  {}. {}", i + 1, shape.description());
    }

    // Trait 对象 vs 泛型：
    // 泛型：编译时确定类型，零运行时开销（静态分发）
    // Trait 对象：运行时确定类型，有一点点运行时开销（动态分发）
    //
    // 什么时候用 Trait 对象？
    // - 需要在集合中存储不同类型时（Vec<Box<dyn Trait>>）
    // - 类型在运行时才能确定时
    // - 不需要极致性能时
    println!("\n注意：Trait 对象有微小的运行时开销，但换来了极大的灵活性！");
}

// ============================================================================
// 实战项目：图形计算器
// ============================================================================

/// 实战项目：图形计算器
///
/// 项目目标：
///   1. 定义 Shape Trait（形状的能力）
///   2. 实现 Circle、Rectangle、Triangle 三种形状
///   3. 用 Trait 对象存储不同形状
///   4. 遍历计算总面积

/// 定义 Shape Trait
/// 就像定义"形状"这种证书的要求：
///   - 必须能计算面积
///   - 必须能描述自己
trait Shape {
    /// 计算面积
    fn area(&self) -> f64;

    /// 获取形状描述
    fn describe(&self) -> String;

    /// 周长（默认实现：不支持）
    fn perimeter(&self) -> f64 {
        0.0  // 默认返回 0，表示"未实现"
    }

    /// 打印形状信息（默认实现，利用其他方法）
    fn print_info(&self) {
        println!("  {} -> 面积: {:.2}, 周长: {:.2}",
            self.describe(), self.area(), self.perimeter());
    }
}

/// 圆形
struct ShapeCircle {
    radius: f64,
}

impl ShapeCircle {
    fn new(radius: f64) -> Self {
        ShapeCircle { radius }
    }
}

impl Shape for ShapeCircle {
    fn area(&self) -> f64 {
        // 面积 = π * r²
        std::f64::consts::PI * self.radius * self.radius
    }

    fn describe(&self) -> String {
        format!("圆形(半径={:.2})", self.radius)
    }

    // 覆盖周长方法
    fn perimeter(&self) -> f64 {
        // 周长 = 2πr
        2.0 * std::f64::consts::PI * self.radius
    }
}

/// 矩形
struct ShapeRectangle {
    width: f64,
    height: f64,
}

impl ShapeRectangle {
    fn new(width: f64, height: f64) -> Self {
        ShapeRectangle { width, height }
    }
}

impl Shape for ShapeRectangle {
    fn area(&self) -> f64 {
        // 面积 = 宽 * 高
        self.width * self.height
    }

    fn describe(&self) -> String {
        format!("矩形({:.2}x{:.2})", self.width, self.height)
    }

    fn perimeter(&self) -> f64 {
        // 周长 = 2 * (宽 + 高)
        2.0 * (self.width + self.height)
    }
}

/// 三角形（已知三边长，用海伦公式求面积）
struct ShapeTriangle {
    a: f64,  // 边 a
    b: f64,  // 边 b
    c: f64,  // 边 c
}

impl ShapeTriangle {
    fn new(a: f64, b: f64, c: f64) -> Self {
        // 验证三角形合法性：任意两边之和大于第三边
        assert!(
            a + b > c && a + c > b && b + c > a,
            "无效的三角形：三边 {}, {}, {} 无法构成三角形", a, b, c
        );
        ShapeTriangle { a, b, c }
    }
}

impl Shape for ShapeTriangle {
    fn area(&self) -> f64 {
        // 海伦公式：area = sqrt(s * (s-a) * (s-b) * (s-c))
        // 其中 s = (a + b + c) / 2 是半周长
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    fn describe(&self) -> String {
        format!("三角形({:.2},{:.2},{:.2})", self.a, self.b, self.c)
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

/// 辅助函数：计算所有形状的总面积
/// 参数是 Trait 对象的切片
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

/// 辅助函数：打印所有形状的详细信息
fn print_all_shapes(shapes: &[Box<dyn Shape>]) {
    for (i, shape) in shapes.iter().enumerate() {
        println!("  {}. ", i + 1);
        shape.print_info();
    }
}

/// 辅助函数：找到面积最大的形状
fn largest_shape(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    shapes
        .iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
        .map(|s| s.as_ref())
}

/// 演示实战项目
fn demo_shape_calculator() {
    println!("\n===== 实战项目：图形计算器 =====\n");

    // 创建各种形状
    let circle1 = ShapeCircle::new(5.0);
    let circle2 = ShapeCircle::new(3.0);
    let rect1 = ShapeRectangle::new(4.0, 6.0);
    let rect2 = ShapeRectangle::new(10.0, 2.5);
    let tri1 = ShapeTriangle::new(3.0, 4.0, 5.0);  // 经典直角三角形
    let tri2 = ShapeTriangle::new(7.0, 8.0, 9.0);

    // 用 Vec<Box<dyn Shape>> 存储不同形状
    // 这就是 Trait 对象的威力！不同类型可以放在同一个集合里
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(circle1),
        Box::new(circle2),
        Box::new(rect1),
        Box::new(rect2),
        Box::new(tri1),
        Box::new(tri2),
    ];

    // 打印所有形状信息
    println!("所有形状：");
    print_all_shapes(&shapes);

    // 计算总面积
    let total = total_area(&shapes);
    println!("\n形状总数: {}", shapes.len());
    println!("总面积: {:.2}", total);

    // 找到最大面积的形状
    if let Some(biggest) = largest_shape(&shapes) {
        println!("\n面积最大的形状: {} ({:.2})", biggest.describe(), biggest.area());
    }

    // 演示特有方法（不通过 Trait 对象调用）
    println!("\n圆形特有计算：");
    let r = 5.0;
    println!("  半径 {} 的圆：", r);
    println!("    面积 = π × {}² = {:.4}", r, std::f64::consts::PI * r * r);
    println!("    周长 = 2π × {} = {:.4}", r, 2.0 * std::f64::consts::PI * r);
    println!("    直径 = {} × 2 = {:.2}", r, r * 2.0);
}

// ============================================================================
// 练习题
// ============================================================================

// ---------- 练习 1：泛型 Stack<T>（基础巩固，5-10 分钟） ----------
//
// 任务：实现一个泛型栈（Stack），支持以下操作：
//   - push(item)：压入元素
//   - pop()：弹出栈顶元素
//   - peek()：查看栈顶元素（不弹出）
//   - is_empty()：判断是否为空
//   - size()：返回元素个数
//
// 提示：
//   - 内部用 Vec<T> 存储元素
//   - 栈的特点是"后进先出"（LIFO）

/// 泛型栈
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    /// 创建一个空栈
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    /// 压入元素到栈顶
    /// TODO: 请实现这个方法
    fn push(&mut self, _item: T) {
        todo!("练习1：实现 push 方法")
    }

    /// 弹出栈顶元素
    /// 返回 Some(item) 或 None（栈空时）
    /// TODO: 请实现这个方法
    fn pop(&mut self) -> Option<T> {
        todo!("练习1：实现 pop 方法")
    }

    /// 查看栈顶元素（不弹出）
    /// 返回 Some(&item) 或 None（栈空时）
    /// TODO: 请实现这个方法
    fn peek(&self) -> Option<&T> {
        todo!("练习1：实现 peek 方法")
    }

    /// 判断栈是否为空
    /// TODO: 请实现这个方法
    fn is_empty(&self) -> bool {
        todo!("练习1：实现 is_empty 方法")
    }

    /// 返回栈中元素个数
    /// TODO: 请实现这个方法
    fn size(&self) -> usize {
        todo!("练习1：实现 size 方法")
    }
}

/// 练习 1 的测试函数
fn exercise_1_stack() {
    println!("\n===== 练习 1：泛型 Stack<T> =====\n");

    // 整数栈
    let mut int_stack: Stack<i32> = Stack::new();
    println!("整数栈是否为空: {}", int_stack.is_empty());

    int_stack.push(10);
    int_stack.push(20);
    int_stack.push(30);
    println!("压入 10, 20, 30 后，大小: {}", int_stack.size());
    println!("栈顶元素: {:?}", int_stack.peek());

    println!("弹出: {:?}", int_stack.pop());
    println!("弹出: {:?}", int_stack.pop());
    println!("剩余大小: {}", int_stack.size());

    // 字符串栈
    let mut str_stack: Stack<String> = Stack::new();
    str_stack.push(String::from("Hello"));
    str_stack.push(String::from("World"));
    println!("\n字符串栈顶: {:?}", str_stack.peek());
}

// ===== 测试版练习 =====
// 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
// 取消下面的注释，运行 cargo test --example lesson08

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_basic_operations() {
        let mut stack: Stack<i32> = Stack::new();
        // 新建栈应该为空
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);

        // 压入元素
        stack.push(10);
        stack.push(20);
        stack.push(30);
        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 3);
        assert_eq!(stack.peek(), Some(&30)); // 栈顶是最后压入的

        // 弹出元素（后进先出）
        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.peek(), Some(&10));

        // 弹出最后一个
        assert_eq!(stack.pop(), Some(10));
        assert!(stack.is_empty());
    }
}
*/

// ---------- 练习 2：Printable Trait（应用练习，15-20 分钟） ----------
//
// 任务：
//   1. 定义一个 Printable Trait，包含：
//      - to_string(&self) -> String（必须实现）
//      - print(&self)（默认实现，调用 to_string 并打印）
//   2. 为以下结构体实现 Printable：
//      - Color { r: u8, g: u8, b: u8 } -> 格式: "RGB(r, g, b)"
//      - Temperature { value: f64, unit: String } -> 格式: "25.0°C"

/// 可打印 Trait
trait Printable {
    /// 转换为字符串
    /// TODO: 这个方法需要各类型自己实现
    fn to_string(&self) -> String;

    /// 打印自己（默认实现）
    /// 大多数情况下不需要覆盖
    fn print(&self) {
        println!("[Printable] {}", self.to_string());
    }
}

/// 颜色结构体
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

/// TODO: 为 Color 实现 Printable Trait
/// 格式: "RGB(r, g, b)"
impl Printable for Color {
    fn to_string(&self) -> String {
        todo!("练习2：为 Color 实现 to_string")
    }
}

/// 温度结构体
struct Temperature {
    value: f64,
    unit: String,
}

/// TODO: 为 Temperature 实现 Printable Trait
/// 格式: "25.0°C" 或 "77.0°F"
impl Printable for Temperature {
    fn to_string(&self) -> String {
        todo!("练习2：为 Temperature 实现 to_string")
    }
}

/// 练习 2 的测试函数
fn exercise_2_printable() {
    println!("\n===== 练习 2：Printable Trait =====\n");

    let red = Color { r: 255, g: 0, b: 0 };
    let sky_blue = Color { r: 135, g: 206, b: 235 };

    red.print();       // 应输出: [Printable] RGB(255, 0, 0)
    sky_blue.print();  // 应输出: [Printable] RGB(135, 206, 235)

    let temp_c = Temperature { value: 25.0, unit: String::from("°C") };
    let temp_f = Temperature { value: 77.0, unit: String::from("°F") };

    temp_c.print();    // 应输出: [Printable] 25.0°C
    temp_f.print();    // 应输出: [Printable] 77.0°F
}

// ---------- 练习 3：Summarizable Trait 对象（进阶挑战，选做） ----------
//
// 任务：
//   1. 定义 Summarizable Trait：
//      - title(&self) -> String
//      - summary(&self) -> String（默认实现）
//      - word_count(&self) -> usize（默认实现）
//   2. 为以下类型实现：
//      - BlogPost { title, content, author }
//      - Book { title, author, pages }
//      - Podcast { title, host, duration_minutes }
//   3. 用 Vec<Box<dyn Summarizable>> 存储不同类型
//   4. 遍历打印所有内容的摘要

/// 可摘要 Trait
trait Summarizable {
    /// 获取标题
    fn title(&self) -> String;

    /// 获取摘要（默认实现：截取前 50 个字符）
    fn summary(&self) -> String {
        let content = self.title();
        if content.len() > 50 {
            format!("{}...", &content[..50])
        } else {
            content
        }
    }

    /// 获取字数统计（默认实现：返回 0）
    fn word_count(&self) -> usize {
        0
    }

    /// 打印完整信息（默认实现）
    fn print_full_info(&self) {
        println!("  标题: {}", self.title());
        println!("  摘要: {}", self.summary());
        println!("  字数: {}", self.word_count());
    }
}

/// 博客文章
struct BlogPost {
    title: String,
    content: String,
    author: String,
}

/// TODO: 为 BlogPost 实现 Summarizable
impl Summarizable for BlogPost {
    fn title(&self) -> String {
        todo!("练习3：实现 BlogPost 的 title")
    }
}

/// 书籍
struct Book {
    title: String,
    author: String,
    pages: u32,
}

/// TODO: 为 Book 实现 Summarizable
impl Summarizable for Book {
    fn title(&self) -> String {
        todo!("练习3：实现 Book 的 title")
    }
}

/// 播客
struct Podcast {
    title: String,
    host: String,
    duration_minutes: u32,
}

/// TODO: 为 Podcast 实现 Summarizable
impl Summarizable for Podcast {
    fn title(&self) -> String {
        todo!("练习3：实现 Podcast 的 title")
    }
}

/// 练习 3 的测试函数
fn exercise_3_summarizable() {
    println!("\n===== 练习 3：Summarizable Trait 对象 =====\n");

    // 创建不同类型的内容
    let post = BlogPost {
        title: String::from("Rust 学习笔记：泛型与 Trait"),
        content: String::from("泛型和 Trait 是 Rust 最强大的特性之一..."),
        author: String::from("G-one"),
    };

    let book = Book {
        title: String::from("Rust 程序设计语言"),
        author: String::from("Steve Klabnik"),
        pages: 560,
    };

    let podcast = Podcast {
        title: String::from("Rustacean Station"),
        host: String::from("Rust 团队"),
        duration_minutes: 45,
    };

    // 用 Trait 对象存储不同类型
    let contents: Vec<Box<dyn Summarizable>> = vec![
        Box::new(post),
        Box::new(book),
        Box::new(podcast),
    ];

    // 遍历打印所有内容的完整信息
    println!("所有内容：");
    for (i, item) in contents.iter().enumerate() {
        println!("\n--- 第 {} 项 ---", i + 1);
        item.print_full_info();
    }
}

// ============================================================================
// 主函数：运行所有演示
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       第 08 课：泛型与 Trait —— 写一次，适用万种类型       ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    // 基础演示
    demo_generic_function();
    demo_generic_struct();
    demo_trait_basic();
    demo_trait_as_param();
    demo_where_clause();
    demo_standard_traits();
    demo_trait_object();

    // 实战项目
    demo_shape_calculator();

    // 练习题（取消注释以运行）
    // exercise_1_stack();      // 取消注释运行练习 1
    // exercise_2_printable();  // 取消注释运行练习 2
    // exercise_3_summarizable(); // 取消注释运行练习 3

    println!("\n课程演示结束！");
    println!("请完成练习题（取消 main 函数中对应函数的注释）");
}

/*
 * 核心收获：
 * - 泛型让你写一次代码就能处理多种类型，编译时单态化保证零运行时开销
 * - Trait 是 Rust 的"能力系统"，定义共享行为，让不同类型可以有统一接口
 * - Trait 对象（dyn Trait）实现运行时多态，用 Box<dyn Trait> 存储不同类型
 *
 * 常见陷阱：
 * - 忘记在 impl<T> 中声明泛型参数 T，直接写 impl Point<T> 会报错
 * - 混淆 impl Trait（语法糖）和 dyn Trait（Trait 对象）的使用场景：
 *   impl Trait 用于函数参数（静态分发），dyn Trait 用于存储不同类型（动态分发）
 *
 * 下节课预告：
 * - 学习集合类型和迭代器，掌握处理批量数据的利器！
 */
