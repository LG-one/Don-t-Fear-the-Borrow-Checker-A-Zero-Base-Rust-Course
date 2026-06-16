// =============================================================================
//  第 06 课：枚举与模式匹配
// =============================================================================
//
//  前置知识：00-05（环境搭建、变量与类型、控制流、所有权、结构体、生命周期）
//
//  本课目标：
//    1. 理解枚举的概念，学会定义基本枚举和带数据的枚举
//    2. 掌握 Option<T>，理解 Rust 为什么没有 null
//    3. 学会使用 match 表达式进行模式匹配和解构
//    4. 掌握 if let、while let、matches! 等简化写法
//    5. 完成扑克牌发牌器实战项目
//
// =============================================================================

// ---------------------------------------------------------------------------
//  第一部分：什么是枚举？
// ---------------------------------------------------------------------------
//
//  【生活类比：交通信号灯】
//
//  想象一下马路上的交通信号灯——它永远只能是三种状态之一：
//    - 红灯（停）
//    - 绿灯（行）
//    - 黄灯（等一等）
//
//  它不会同时是红灯又是绿灯，也不会出现第四种颜色。
//  这就是"枚举"的核心思想：一个值只能是若干个"变体"中的某一个。
//
//  在 Rust 中，我们用 enum 关键字来定义枚举。
// ---------------------------------------------------------------------------

// 定义一个交通信号灯枚举
// enum 关键字后面跟枚举的名字（首字母大写）
// 花括号里列出所有可能的变体（也叫"成员"）
#[derive(Debug)] // 加上 Debug，这样可以用 {:?} 打印出来
enum TrafficLight {
    Red,    // 红灯
    Green,  // 绿灯
    Yellow, // 黄灯
}

// 定义一个方向枚举——只能是上下左右四个方向之一
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// 辅助函数：演示基本枚举的使用
fn demo_basic_enum() {
    println!("========== 基本枚举演示 ==========");

    // 创建一个枚举值：用 枚举名::变体名 的形式
    let light = TrafficLight::Green;
    println!("当前信号灯：{:?}", light);

    // 枚举可以用在函数参数中
    fn check_light(light: &TrafficLight) {
        // 这里先用 if 来简单判断，后面我们会用更强大的 match
        // 比较枚举值时，用 == 连接两个枚举变体
        if let TrafficLight::Red = light {
            println!("  红灯停！");
        } else if let TrafficLight::Green = light {
            println!("  绿灯行！");
        } else if let TrafficLight::Yellow = light {
            println!("  黄灯等一等！");
        }
    }

    check_light(&TrafficLight::Red);
    check_light(&TrafficLight::Green);
    check_light(&TrafficLight::Yellow);

    // 使用方向枚举
    let dir = Direction::Up;
    println!("\n当前方向：{:?}", dir);
}

// ---------------------------------------------------------------------------
//  第二部分：带数据的枚举（枚举的四种变体形式）
// ---------------------------------------------------------------------------
//
//  【生活类比：快递包裹】
//
//  想象一个快递站，包裹有不同的类型：
//    - 普通信件：只需要知道收件人名字
//    - 标准包裹：需要知道重量（公斤）
//    - 易碎品：需要知道重量和易碎等级
//    - 国际快递：需要知道目的国家和运费
//
//  每种类型携带的数据不同，但它们都是"快递"这个大类下的变体。
//  Rust 的枚举可以做到这一点——每个变体可以携带不同类型、不同数量的数据。
// ---------------------------------------------------------------------------

// ---- 变体形式 1：无数据变体（最简单，和上面的交通灯一样） ----
#[derive(Debug)]
enum Season {
    Spring, // 春天——没有附带数据
    Summer,
    Autumn,
    Winter,
}

// ---- 变体形式 2：带命名字段的变体（像结构体一样有字段名） ----
#[derive(Debug)]
enum Shape {
    // 圆形：需要一个半径
    Circle { radius: f64 },
    // 矩形：需要宽和高
    Rectangle { width: f64, height: f64 },
}

// ---- 变体形式 3：带元组数据的变体（没有字段名，按位置） ----
#[derive(Debug)]
enum Color {
    // RGB 颜色：三个值分别代表红、绿、蓝
    Rgb(u8, u8, u8),
    // 灰度：只有一个值
    Grayscale(u8),
}

// ---- 变体形式 4：混合形式（不同变体用不同的形式） ----
#[derive(Debug)]
enum Message {
    Quit,                       // 无数据——用户想退出
    Echo(String),               // 带一个字符串——显示一条消息
    MoveTo { x: i32, y: i32 }, // 带命名字段——移动到某个坐标
    Colorize(Color),            // 带另一个枚举——给消息上色
}

// 辅助函数：演示带数据的枚举
fn demo_enum_with_data() {
    println!("\n========== 带数据的枚举演示 ==========");

    // --- 季节（无数据变体） ---
    let current_season = Season::Spring;
    println!("当前季节：{:?}", current_season);

    // --- 形状（命名字段变体） ---
    let circle = Shape::Circle { radius: 5.0 };
    let rect = Shape::Rectangle {
        width: 10.0,
        height: 3.0,
    };
    println!("形状1：{:?}", circle);
    println!("形状2：{:?}", rect);

    // --- 颜色（元组变体） ---
    let red = Color::Rgb(255, 0, 0);
    let gray = Color::Grayscale(128);
    println!("颜色1：{:?}", red);
    println!("颜色2：{:?}", gray);

    // --- 消息（混合变体） ---
    let msgs = vec![
        Message::Quit,
        Message::Echo(String::from("你好，世界！")),
        Message::MoveTo { x: 100, y: 200 },
        Message::Colorize(Color::Rgb(0, 128, 255)),
    ];

    for msg in &msgs {
        println!("消息：{:?}", msg);
    }
}

// ---------------------------------------------------------------------------
//  第三部分：Option<T> —— Rust 没有 null
// ---------------------------------------------------------------------------
//
//  【生活类比：快递柜】
//
//  想象你去快递柜取包裹：
//    - 要么柜子里有包裹（Some），你拿到了一个真实的东西
//    - 要么柜子是空的（None），你什么也没拿到
//
//  在很多编程语言中，用 "null" 或 "nil" 表示"没有值"。
//  但这很危险——如果你以为有值，去用了 null，程序就会崩溃！
//
//  Rust 的做法更聪明：用 Option<T> 枚举来明确表示"可能有值，也可能没有"。
//
//  Option<T> 是 Rust 标准库中定义的枚举，它只有两个变体：
//    - Some(T)  —— 包含一个 T 类型的值
//    - None      —— 没有值
//
//  因为 Option 太常用了，所以 Some 和 None 不需要写 Option:: 前缀，
//  直接就能用。
//
//  关键规则：你不能直接把 Option<i32> 当作 i32 来用！
//  必须先处理"可能为 None"的情况，才能拿到里面的值。
//  这就是 Rust 帮你杜绝"空指针崩溃"的方式。
// ---------------------------------------------------------------------------

// 辅助函数：演示 Option 的使用
fn demo_option() {
    println!("\n========== Option<T> 演示 ==========");

    // 一个有值的 Option
    let has_value: Option<i32> = Some(42);
    // 一个没有值的 Option
    let no_value: Option<i32> = None;

    println!("has_value = {:?}", has_value); // Some(42)
    println!("no_value = {:?}", no_value);   // None

    // --- 不能直接把 Option<i32> 和 i32 相加 ---
    // 下面这行如果取消注释会报错：
    // let result = has_value + 1; // 错误！不能直接用 Option<i32>
    //
    // Rust 说："你必须先确认里面有值，才能用它！"

    // --- 正确方式 1：用 unwrap()（不推荐，如果 None 会崩溃） ---
    let val = has_value.unwrap(); // 如果是 None，程序会直接 panic
    println!("unwrap 拿到的值：{}", val);

    // --- 正确方式 2：用 unwrap_or() 提供默认值（安全） ---
    let val2 = no_value.unwrap_or(0); // 如果是 None，就用 0 代替
    println!("unwrap_or 拿到的值：{}", val2);

    // --- 正确方式 3：用 is_some() / is_none() 判断（初学者友好） ---
    if has_value.is_some() {
        println!("has_value 里面有值：{}", has_value.unwrap());
    }
    if no_value.is_none() {
        println!("no_value 是空的，没有值");
    }

    // --- 正确方式 4：用 match（最推荐，后面详细讲） ---
    match has_value {
        Some(val) => println!("match 拿到的值：{}", val),
        None => println!("match：没有值"),
    }

    // --- 实际应用场景：查找可能失败 ---
    // 模拟一个根据名字查年龄的函数
    fn find_age(name: &str) -> Option<u32> {
        match name {
            "Alice" => Some(30),  // 找到了，返回年龄
            "Bob" => Some(25),
            _ => None,            // 没找到，返回 None
        }
    }

    // 使用查找结果
    let names = vec!["Alice", "Charlie", "Bob"];
    for name in names {
        match find_age(name) {
            Some(age) => println!("  {} 的年龄是 {}", name, age),
            None => println!("  没有找到 {} 的信息", name),
        }
    }

    // --- Option 的链式方法 ---
    // Option 提供了很多方便的链式方法，让我们可以优雅地处理可能缺失的值

    // map：如果 Some，对里面的值做变换；如果 None，保持 None
    let doubled = has_value.map(|x| x * 2);
    println!("map(乘以2)：{:?}", doubled); // Some(84)

    let none_doubled = no_value.map(|x| x * 2);
    println!("None 的 map：{:?}", none_doubled); // None（什么都不会发生）

    // and_then：如果 Some，用函数返回新的 Option；如果 None，保持 None
    // 这在链式操作中非常有用
    let result = has_value
        .and_then(|x| {
            if x > 0 {
                Some(x * 10) // 大于 0，返回 Some
            } else {
                None // 不大于 0，返回 None
            }
        });
    println!("and_then 结果：{:?}", result); // Some(420)
}

// ---------------------------------------------------------------------------
//  第四部分：match 表达式 —— Rust 最强大的控制流
// ---------------------------------------------------------------------------
//
//  【生活类比：分拣机器】
//
//  想象一个快递分拣中心的机器：
//    - 来了一个包裹，机器先看它的类型
//    - 如果是"标准件"，走左边的传送带
//    - 如果是"加急件"，走右边的快通道
//    - 如果是"易碎品"，走中间的小心处理通道
//    - 如果是"国际件"，走最右边的海关通道
//
//  match 表达式就是这样的"分拣机器"——它检查一个值，
//  然后根据值的不同形式（模式），执行不同的代码。
//
//  match 的核心规则：
//    1. 必须穷尽所有可能——不能漏掉任何一种情况（编译器会帮你检查）
//    2. 用 _ 作为通配符，匹配"其他所有情况"
//    3. match 是表达式，可以返回值
//    4. 每个分支用 => 分隔，用逗号结束（最后一个逗号可省略）
// ---------------------------------------------------------------------------

// 辅助函数：演示 match 基本用法
fn demo_match_basic() {
    println!("\n========== match 基本用法演示 ==========");

    // --- 用 match 判断交通信号灯 ---
    let light = TrafficLight::Yellow;
    let action = match light {
        TrafficLight::Red => "停！",
        TrafficLight::Green => "行！",
        TrafficLight::Yellow => "等一等！",
    };
    println!("信号灯 {:?} -> {}", light, action);

    // --- 用 match 处理数字 ---
    let number = 13;
    let description = match number {
        1 => "一",
        2 => "二",
        3 => "三",
        // 用 ..= 表示范围匹配（包含两端）
        4..=10 => "四到十之间",
        // _ 是通配符，匹配所有其他情况
        _ => "其他数字",
    };
    println!("数字 {} 是：{}", number, description);

    // --- match 是表达式，可以返回值 ---
    let grade = 85;
    let level = match grade {
        90..=100 => "优秀",
        80..=89 => "良好",
        70..=79 => "中等",
        60..=69 => "及格",
        _ => "不及格",
    };
    println!("成绩 {} 分 -> {}", grade, level);

    // --- 穷尽性检查 ---
    // 如果你注释掉 _ 那一行，编译器会报错：
    // "non-exhaustive patterns: values not covered"
    // Rust 要求你必须处理所有可能的情况，这避免了意外遗漏！

    // --- 多个模式用 | 分隔 ---
    let day = "周六";
    let is_weekend = match day {
        "周六" | "周日" => true, // 周六或周日都匹配
        _ => false,
    };
    println!("{} 是周末吗？{}", day, is_weekend);
}

// 辅助函数：演示 match 解构枚举
fn demo_match_destructure_enum() {
    println!("\n========== match 解构枚举演示 ==========");

    // 用 match 解构带数据的枚举——把枚举里的数据"拆"出来
    let msg = Message::Echo(String::from("Hello Rust!"));

    match msg {
        Message::Quit => {
            println!("  收到退出指令");
        }
        // 用 (text) 把 String 从 Echo 变体中解构出来
        Message::Echo(text) => {
            println!("  收到回显消息：{}", text);
        }
        // 用 { x, y } 把坐标从 MoveTo 变体中解构出来
        Message::MoveTo { x, y } => {
            println!("  移动到坐标：({}, {})", x, y);
        }
        // 嵌套解构：先解构 Colorize，再解构里面的 Color
        Message::Colorize(color) => {
            match color {
                Color::Rgb(r, g, b) => {
                    println!("  设置颜色：RGB({}, {}, {})", r, g, b);
                }
                Color::Grayscale(v) => {
                    println!("  设置灰度：{}", v);
                }
            }
        }
    }

    // 更多例子：用 match 处理形状，计算面积
    let shapes: Vec<Shape> = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle {
            width: 4.0,
            height: 6.0,
        },
    ];

    for shape in &shapes {
        let area = match shape {
            // 解构 Circle，取出 radius 字段
            Shape::Circle { radius } => {
                std::f64::consts::PI * radius * radius
            }
            // 解构 Rectangle，取出 width 和 height 字段
            Shape::Rectangle { width, height } => {
                width * height
            }
        };
        println!("  {:?} 的面积是 {:.2}", shape, area);
    }
}

// 辅助函数：演示 match 解构元组
fn demo_match_destructure_tuple() {
    println!("\n========== match 解构元组演示 ==========");

    // 可以用 match 同时解构多个值
    let point = (3, -5);

    match point {
        (0, 0) => println!("  原点"),
        (x, 0) => println!("  在 x 轴上，x = {}", x),
        (0, y) => println!("  在 y 轴上，y = {}", y),
        (x, y) if x > 0 && y > 0 => println!("  第一象限：({}, {})", x, y),
        (x, y) if x < 0 && y > 0 => println!("  第二象限：({}, {})", x, y),
        (x, y) if x < 0 && y < 0 => println!("  第三象限：({}, {})", x, y),
        (x, y) => println!("  第四象限：({}, {})", x, y),
    }

    // 解构元组嵌套 Option
    let a: Option<i32> = Some(10);
    let b: Option<i32> = Some(20);

    match (a, b) {
        (Some(x), Some(y)) => println!("  两个都有值：{} + {} = {}", x, y, x + y),
        (Some(x), None) => println!("  只有 a 有值：{}", x),
        (None, Some(y)) => println!("  只有 b 有值：{}", y),
        (None, None) => println!("  两个都没有值"),
    }
}

// 辅助函数：演示 match 解构结构体
fn demo_match_destructure_struct() {
    println!("\n========== match 解构结构体演示 ==========");

    // 定义一个简单的结构体
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 5 };

    // 用 match 解构结构体
    match p {
        Point { x: 0, y } => println!("  在 y 轴上，y = {}", y),
        Point { x, y: 0 } => println!("  在 x 轴上，x = {}", x),
        Point { x, y } => println!("  普通点：({}, {})", x, y),
    }

    // 也可以用 .. 忽略不关心的字段
    let p2 = Point { x: 10, y: 20 };
    match p2 {
        Point { x, .. } => println!("  只关心 x：{}", x),
    }
}

// ---------------------------------------------------------------------------
//  第五部分：if let —— 简化的匹配
// ---------------------------------------------------------------------------
//
//  有时候你只关心一种情况，用 match 写起来太啰嗦。
//  if let 就是 match 的"简化版"——只匹配一个模式，忽略其他情况。
//
//  语法：if let 模式 = 表达式 { 匹配成功时执行 } else { 其他情况 }
// ---------------------------------------------------------------------------

// 辅助函数：演示 if let
fn demo_if_let() {
    println!("\n========== if let 演示 ==========");

    // --- 对比：match vs if let ---

    // 用 match 写法（完整但啰嗦）
    let some_value: Option<i32> = Some(42);
    match some_value {
        Some(val) => println!("  match 拿到值：{}", val),
        None => {} // 什么都不做，但必须写上以满足穷尽性
    }

    // 用 if let 写法（简洁）
    if let Some(val) = some_value {
        println!("  if let 拿到值：{}", val);
    }
    // 不需要处理 None 的情况，代码更简洁

    // if let 也可以有 else
    let maybe_name: Option<&str> = None;
    if let Some(name) = maybe_name {
        println!("  你好，{}！", name);
    } else {
        println!("  你好，陌生人！");
    }

    // --- 用 if let 解构枚举 ---
    let msg = Message::MoveTo { x: 50, y: 100 };
    if let Message::MoveTo { x, y } = msg {
        println!("  移动到：({}, {})", x, y);
    }

    // --- 实际场景：只关心成功的情况 ---
    let input = "42";
    // parse() 返回 Result，Ok 表示成功，Err 表示失败
    // 这里我们只关心能解析成功的情况
    if let Ok(number) = input.parse::<i32>() {
        println!("  解析成功，数字是：{}", number);
    }

    // 对比：如果也关心失败的情况，就用 match
    let bad_input = "abc";
    match bad_input.parse::<i32>() {
        Ok(n) => println!("  解析成功：{}", n),
        Err(e) => println!("  解析失败：{}", e),
    }
}

// ---------------------------------------------------------------------------
//  第六部分：while let —— 循环中的模式匹配
// ---------------------------------------------------------------------------
//
//  while let 是"只要模式匹配成功，就一直循环"。
//  它特别适合处理一系列可能为空的操作，比如从栈中弹出元素。
//
//  语法：while let 模式 = 表达式 { 循环体 }
// ---------------------------------------------------------------------------

// 辅助函数：演示 while let
fn demo_while_let() {
    println!("\n========== while let 演示 ==========");

    // --- 示例 1：模拟栈（用 Vec 当栈） ---
    // Vec 的 pop() 返回 Option<T>
    //   - 有元素时返回 Some(元素)
    //   - 空了返回 None
    let mut stack = vec![1, 2, 3, 4, 5];
    println!("  原始栈：{:?}", stack);

    print!("  弹出顺序：");
    // 只要 pop() 返回 Some，就继续循环
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("\n  栈现在是空的：{:?}", stack);

    // --- 对比：如果用 loop + match 写同样的逻辑 ---
    let mut stack2 = vec!["a", "b", "c"];
    println!("\n  用 loop + match 模拟：");
    loop {
        match stack2.pop() {
            Some(val) => println!("    弹出：{}", val),
            None => {
                println!("    栈空了，退出循环");
                break;
            }
        }
    }

    // --- 示例 2：处理迭代器 ---
    // iter().next() 也返回 Option
    let words = vec!["hello", "world"];
    let mut iter = words.iter();
    println!("\n  用 while let 遍历迭代器：");
    while let Some(word) = iter.next() {
        println!("    {}", word);
    }
}

// ---------------------------------------------------------------------------
//  第七部分：matches! 宏 —— 快速判断是否匹配
// ---------------------------------------------------------------------------
//
//  matches! 是一个非常方便的宏，它把 match 表达式简化成一个返回 bool 的表达式。
//  语法：matches!(值, 模式)
//  如果值匹配模式，返回 true；否则返回 false。
//
//  特别适合用在 filter、any 等需要布尔值的场景。
// ---------------------------------------------------------------------------

// 辅助函数：演示 matches! 宏
fn demo_matches_macro() {
    println!("\n========== matches! 宏演示 ==========");

    // --- 基本用法 ---
    let number = 42;
    // 判断 number 是否在 1..=100 范围内
    let is_in_range = matches!(number, 1..=100);
    println!("  {} 在 1~100 范围内？{}", number, is_in_range); // true

    let char = 'c';
    // 判断是否是元音字母
    let is_vowel = matches!(char, 'a' | 'e' | 'i' | 'o' | 'u');
    println!("  '{}' 是元音？{}", char, is_vowel); // false

    // --- 用于判断枚举变体 ---
    let msg = Message::Echo(String::from("test"));
    let is_echo = matches!(msg, Message::Echo(_));
    //     _ 表示"我不关心里面的数据，只要它是 Echo 就行"
    println!("  msg 是 Echo 消息？{}", is_echo); // true

    let is_quit = matches!(msg, Message::Quit);
    println!("  msg 是 Quit 消息？{}", is_quit); // false

    // --- 带条件的匹配（match guard） ---
    let num = Some(42);
    let is_positive = matches!(num, Some(x) if x > 0);
    //     if x > 0 是额外的条件，x 必须大于 0 才算匹配
    println!("  {:?} 是正数？{}", num, is_positive); // true

    // --- 在迭代中使用（filter + matches!） ---
    let messages = vec![
        Message::Quit,
        Message::Echo(String::from("hello")),
        Message::MoveTo { x: 1, y: 2 },
        Message::Echo(String::from("world")),
        Message::Quit,
    ];

    // 只筛选出 Echo 消息
    let echo_count = messages
        .iter()
        .filter(|msg| matches!(msg, Message::Echo(_)))
        .count();
    println!("  Echo 消息数量：{}", echo_count); // 2

    // 只筛选出 Quit 消息
    let quit_count = messages
        .iter()
        .filter(|msg| matches!(msg, Message::Quit))
        .count();
    println!("  Quit 消息数量：{}", quit_count); // 2

    // --- 综合示例：判断字符类型 ---
    fn classify_char(c: char) -> &'static str {
        if matches!(c, 'a'..='z' | 'A'..='Z') {
            "字母"
        } else if matches!(c, '0'..='9') {
            "数字"
        } else if matches!(c, ' ' | '\t' | '\n') {
            "空白字符"
        } else {
            "其他字符"
        }
    }

    let test_chars = vec!['A', '5', ' ', '!', 'z'];
    for c in &test_chars {
        println!("  '{}' -> {}", c, classify_char(*c));
    }
}

// =============================================================================
//  实战项目：扑克牌发牌器
// =============================================================================
//
//  接下来我们要用枚举和结构体构建一个扑克牌发牌器！
//
//  这个项目会用到：
//    - 枚举定义（花色和点数）
//    - 结构体（扑克牌）
//    - match 表达式（判断牌型）
//    - Vec 操作（创建牌组、洗牌、发牌）
//
// =============================================================================

// 花色枚举：黑桃、红心、方块、梅花
#[derive(Debug, Clone, Copy, PartialEq)]
// Clone: 可以复制; Copy: 可以隐式复制; PartialEq: 可以用 == 比较
enum Suit {
    Spades,   // 黑桃
    Hearts,   // 红心
    Diamonds, // 方块
    Clubs,    // 梅花
}

// 点数枚举：A, 2-10, J, Q, K
#[derive(Debug, Clone, Copy, PartialEq)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

// 扑克牌结构体：由花色和点数组成
#[derive(Debug, Clone, Copy)]
struct Card {
    suit: Suit, // 花色
    rank: Rank, // 点数
}

// 为 Rank 实现一些方便的方法
impl Rank {
    // 获取点数的数字值（用于比较大小）
    fn value(&self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }

    // 获取点数的显示名称
    fn display(&self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }
}

// 为 Suit 实现显示方法
impl Suit {
    fn display(&self) -> &'static str {
        match self {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
        }
    }
}

// 为 Card 实现方法
impl Card {
    // 创建一张新牌
    fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }

    // 显示牌面
    fn display(&self) -> String {
        format!("{}{}", self.suit.display(), self.rank.display())
    }
}

// 创建一副完整的 52 张扑克牌
fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    // 四种花色
    let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
    // 13 种点数
    let ranks = [
        Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
        Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
        Rank::Jack, Rank::Queen, Rank::King,
    ];

    // 嵌套循环：每种花色配每种点数，生成 4 × 13 = 52 张牌
    for &suit in &suits {
        for &rank in &ranks {
            deck.push(Card::new(suit, rank));
        }
    }

    deck
}

// 洗牌算法：Fisher-Yates 洗牌（简单随机交换）
// 注意：这里用伪随机方式，基于索引计算，不依赖外部随机库
fn shuffle_deck(deck: &mut Vec<Card>) {
    let len = deck.len();
    // 用一个简单的"种子"来生成伪随机效果
    // 在真实项目中，应该用 rand 库，但这里为了不引入外部依赖
    // 我们用一种确定性的"看起来随机"的交换方式
    let seed: u64 = 42; // 种子值，可以修改来得到不同的"洗牌"结果

    // 使用一个简单的线性同余生成器来产生"随机"索引
    let mut state = seed;
    for i in (1..len).rev() {
        // 线性同余公式：state = state * 6364136223846793005 + 1442695040888963407
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let j = (state as usize) % (i + 1);
        // 交换第 i 张和第 j 张牌
        deck.swap(i, j);
    }
}

// 发牌：从牌堆顶部发 n 张牌
fn deal_cards(deck: &mut Vec<Card>, n: usize) -> Vec<Card> {
    let mut hand = Vec::new();
    for _ in 0..n {
        if let Some(card) = deck.pop() {
            hand.push(card);
        }
    }
    hand
}

// 牌型枚举：表示不同的扑克牌组合
#[derive(Debug, PartialEq)]
enum HandType {
    HighCard,       // 高牌（什么都不匹配）
    OnePair,        // 对子（两张相同点数）
    TwoPairs,       // 两对
    ThreeOfAKind,   // 三条
    Straight,       // 顺子（五张连续）
    Flush,          // 同花（五张同花色）
    FullHouse,      // 葫芦（三条 + 一对）
    FourOfAKind,    // 四条
    StraightFlush,  // 同花顺
}

// 判断手牌的牌型
fn evaluate_hand(hand: &[Card]) -> HandType {
    if hand.len() < 5 {
        return HandType::HighCard;
    }

    // 检查是否同花：所有牌花色相同
    let is_flush = hand.iter().all(|card| card.suit == hand[0].suit);

    // 检查是否顺子：点数连续
    // 先把点数排好序
    let mut values: Vec<u8> = hand.iter().map(|c| c.rank.value()).collect();
    values.sort();
    // 检查连续性：相邻两张牌的差值都是 1
    let is_straight = values.windows(2).all(|w| w[1] - w[0] == 1);

    // 统计每种点数出现的次数
    // 用一个简单的方式：遍历所有点数，统计出现次数
    let mut counts = Vec::new();
    for i in 0..values.len() {
        if counts.iter().any(|&(v, _)| v == values[i]) {
            continue; // 已经统计过了
        }
        let count = values.iter().filter(|&&v| v == values[i]).count();
        counts.push((values[i], count));
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1)); // 按次数从大到小排序

    // 根据统计结果判断牌型
    match (is_flush, is_straight, counts[0].1) {
        (true, true, _) => HandType::StraightFlush,
        (_, _, 4) => HandType::FourOfAKind,
        (_, _, 3) if counts.len() > 1 && counts[1].1 == 2 => HandType::FullHouse,
        (true, _, _) => HandType::Flush,
        (_, true, _) => HandType::Straight,
        (_, _, 3) => HandType::ThreeOfAKind,
        (_, _, 2) if counts.iter().filter(|&&(_, c)| c == 2).count() == 2 => HandType::TwoPairs,
        (_, _, 2) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

// 打印手牌信息
fn print_hand(hand: &[Card]) {
    print!("  手牌：");
    for card in hand {
        print!("{} ", card.display());
    }
    println!();
}

// 辅助函数：完整的扑克牌发牌器演示
fn demo_poker() {
    println!("\n========== 实战项目：扑克牌发牌器 ==========");

    // 第一步：创建一副新牌
    let mut deck = create_deck();
    println!("  创建了一副 {} 张牌的牌组", deck.len());

    // 打印前几张牌，确认牌组正确
    print!("  前 5 张牌：");
    for i in 0..5 {
        print!("{} ", deck[i].display());
    }
    println!();

    // 第二步：洗牌
    shuffle_deck(&mut deck);
    println!("\n  洗牌完成！");
    print!("  洗牌后前 5 张：");
    for i in 0..5 {
        print!("{} ", deck[i].display());
    }
    println!();

    // 第三步：发牌（发 5 张）
    println!("\n  --- 发牌 ---");
    let hand = deal_cards(&mut deck, 5);
    print_hand(&hand);
    println!("  牌堆剩余：{} 张", deck.len());

    // 第四步：判断牌型
    let hand_type = evaluate_hand(&hand);
    println!("  牌型：{:?}", hand_type);

    // 第五步：演示一些特殊牌型
    println!("\n  --- 特殊牌型演示 ---");

    // 模拟一对
    let pair_hand = vec![
        Card::new(Suit::Spades, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Three),
        Card::new(Suit::Clubs, Rank::Seven),
        Card::new(Suit::Spades, Rank::King),
    ];
    print!("  一对测试：");
    for card in &pair_hand {
        print!("{} ", card.display());
    }
    println!(" -> {:?}", evaluate_hand(&pair_hand));

    // 模拟同花
    let flush_hand = vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Hearts, Rank::Five),
        Card::new(Suit::Hearts, Rank::Eight),
        Card::new(Suit::Hearts, Rank::Jack),
        Card::new(Suit::Hearts, Rank::King),
    ];
    print!("  同花测试：");
    for card in &flush_hand {
        print!("{} ", card.display());
    }
    println!(" -> {:?}", evaluate_hand(&flush_hand));

    // 模拟葫芦（三条 + 一对）
    let full_house_hand = vec![
        Card::new(Suit::Spades, Rank::Ten),
        Card::new(Suit::Hearts, Rank::Ten),
        Card::new(Suit::Diamonds, Rank::Ten),
        Card::new(Suit::Clubs, Rank::Four),
        Card::new(Suit::Spades, Rank::Four),
    ];
    print!("  葫芦测试：");
    for card in &full_house_hand {
        print!("{} ", card.display());
    }
    println!(" -> {:?}", evaluate_hand(&full_house_hand));
}

// =============================================================================
//  主函数：运行所有演示
// =============================================================================

fn main() {
    println!("============================================");
    println!("  第 06 课：枚举与模式匹配");
    println!("============================================");

    // 基本枚举
    demo_basic_enum();

    // 带数据的枚举
    demo_enum_with_data();

    // Option<T>
    demo_option();

    // match 基本用法
    demo_match_basic();

    // match 解构枚举
    demo_match_destructure_enum();

    // match 解构元组
    demo_match_destructure_tuple();

    // match 解构结构体
    demo_match_destructure_struct();

    // if let
    demo_if_let();

    // while let
    demo_while_let();

    // matches! 宏
    demo_matches_macro();

    // 实战项目：扑克牌发牌器
    demo_poker();

    println!("\n============================================");
    println!("  第 06 课程结束！");
    println!("============================================");
}

// ===== 测试版练习 =====
// 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
// 取消下面的注释，运行 cargo test --example 06_枚举与模式匹配

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise1_command_enum() {
        // 测试练习 1 的核心功能：Command 枚举的 execute 函数
        #[derive(Debug)]
        enum Command {
            Add(f64, f64),
            Sub(f64, f64),
            Mul(f64, f64),
            Div(f64, f64),
            Help,
            Exit,
        }

        fn execute(cmd: &Command) -> Option<f64> {
            match cmd {
                Command::Add(a, b) => Some(a + b),
                Command::Sub(a, b) => Some(a - b),
                Command::Mul(a, b) => Some(a * b),
                Command::Div(a, b) => {
                    if *b == 0.0 { None } else { Some(a / b) }
                }
                Command::Help | Command::Exit => None,
            }
        }

        assert_eq!(execute(&Command::Add(10.0, 5.0)), Some(15.0));
        assert_eq!(execute(&Command::Sub(10.0, 3.0)), Some(7.0));
        assert_eq!(execute(&Command::Mul(4.0, 7.0)), Some(28.0));
        assert_eq!(execute(&Command::Div(15.0, 3.0)), Some(5.0));
        assert_eq!(execute(&Command::Div(10.0, 0.0)), None);
        assert_eq!(execute(&Command::Help), None);
        assert_eq!(execute(&Command::Exit), None);
    }
}
*/

// =============================================================================
//  练习题
// =============================================================================

/*
 * 练习 1：Command 枚举（基础巩固，5-10 分钟）
 *
 * 定义一个 Command 枚举，包含以下变体：
 *   - Add(f64, f64)      —— 加法，携带两个操作数
 *   - Sub(f64, f64)      —— 减法
 *   - Mul(f64, f64)      —— 乘法
 *   - Div(f64, f64)      —— 除法（注意除以零的情况）
 *   - Help               —— 显示帮助信息
 *   - Exit               —— 退出程序
 *
 * 要求：
 *   1. 定义 Command 枚举
 *   2. 编写 execute 函数，用 match 处理每种命令
 *   3. 对于 Div，当除数为 0 时打印错误信息而不是崩溃
 *   4. 在 main 中测试所有命令变体
 *
 * 提示：
 *   - 用 match 处理每种 Command 变体
 *   - Div 的除数为零可以用 if 判断
 *   - match 是表达式，可以让它返回计算结果
 *
 * 骨架代码：
 */

/*
#[derive(Debug)]
enum Command {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
    Help,
    Exit,
}

fn execute(cmd: &Command) -> Option<f64> {
    match cmd {
        Command::Add(a, b) => {
            println!("  计算：{} + {} = {}", a, b, a + b);
            Some(a + b)
        }
        // TODO: 在这里实现 Sub、Mul、Div 的处理
        // 注意：Div 需要检查除数是否为 0
        Command::Help => {
            println!("  可用命令：Add, Sub, Mul, Div, Help, Exit");
            None
        }
        Command::Exit => {
            println!("  再见！");
            None
        }
        _ => None, // 这行会在你实现完所有变体后变得多余
    }
}

fn main() {
    let commands = vec![
        Command::Add(10.0, 5.0),
        Command::Sub(10.0, 3.0),
        Command::Mul(4.0, 7.0),
        Command::Div(15.0, 3.0),
        Command::Div(10.0, 0.0), // 测试除以零
        Command::Help,
        Command::Exit,
    ];

    for cmd in &commands {
        println!("\n执行命令：{:?}", cmd);
        execute(cmd);
    }
}
*/

/*
 * 练习 2：Option 链式处理（应用练习，15-20 分钟）
 *
 * 实现一个学生信息查询系统，演示 Option 的链式处理：
 *   find_student_by_id(id) -> Option<&str>     返回学生姓名
 *   get_score(name) -> Option<f64>              返回学生分数
 *   grade(score) -> &'static str                返回等级
 *
 * 要求：
 *   1. 用 match 或 if let 实现链式调用
 *   2. 在任何一步失败时，给出友好的错误提示
 *   3. 尝试用 and_then 实现更优雅的链式写法
 *   4. 测试存在的学生和不存在的学生
 *
 * 提示：
 *   - 这个练习模拟了真实开发中的"数据可能存在也可能不存在"的场景
 *   - 链式处理是 Rust 中非常常见的模式
 *   - and_then 接收一个闭包，闭包返回新的 Option
 *
 * 骨架代码：
 */

/*
fn find_student_by_id(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("Alice"),
        2 => Some("Bob"),
        3 => Some("Charlie"),
        _ => None,
    }
}

fn get_score(name: &str) -> Option<f64> {
    match name {
        "Alice" => Some(92.5),
        "Bob" => Some(78.0),
        // Charlie 的成绩还没录入
        _ => None,
    }
}

fn grade(score: f64) -> &'static str {
    match score as u32 {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

// 方式一：用 match 逐层处理
fn query_student_match(id: u32) {
    println!("\n查询学生 ID={}：", id);
    match find_student_by_id(id) {
        Some(name) => {
            println!("  找到学生：{}", name);
            match get_score(name) {
                Some(score) => {
                    println!("  分数：{}", score);
                    println!("  等级：{}", grade(score));
                }
                None => println!("  错误：没有找到 {} 的成绩记录", name),
            }
        }
        None => println!("  错误：没有找到 ID={} 的学生", id),
    }
}

// 方式二：用 and_then 链式处理（更优雅）
// TODO: 实现这个函数
fn query_student_chain(id: u32) {
    println!("\n链式查询学生 ID={}：", id);
    let result = find_student_by_id(id)
        .and_then(|name| {
            // TODO: 在这里获取成绩，并返回 Some(结果字符串)
            // 提示：get_score(name).map(|score| ...)
            None::<String> // 替换为你的实现
        });

    match result {
        Some(info) => println!("  {}", info),
        None => println!("  查询失败"),
    }
}

fn main() {
    // 测试存在的学生（有成绩）
    query_student_match(1); // Alice
    query_student_chain(1);

    // 测试存在的学生（没有成绩）
    query_student_match(3); // Charlie
    query_student_chain(3);

    // 测试不存在的学生
    query_student_match(99);
    query_student_chain(99);
}
*/

/*
 * 练习 3：matches! 宏与 while let（进阶挑战，选做）
 *
 * 第一部分：matches! 宏
 *   1. 创建一个 TrafficLight 枚举（Red, Green, Yellow）
 *   2. 用 matches! 宏判断一个灯是否可以通行（Green）
 *   3. 用 matches! + filter 筛选出所有绿灯
 *   4. 用 matches! + 带条件的匹配（match guard）判断特殊灯
 *
 * 第二部分：while let
 *   1. 实现一个简单的括号匹配检查器
 *   2. 用 while let 逐个取出字符
 *   3. 遇到开括号就压栈，遇到闭括号就检查栈顶
 *   4. 最终检查栈是否为空（为空则括号匹配正确）
 *
 * 骨架代码：
 */

/*
#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
    BlinkingGreen, // 闪烁绿灯（即将变黄）
}

fn main() {
    // --- matches! 宏练习 ---
    println!("=== matches! 宏练习 ===");

    let lights = vec![
        TrafficLight::Red,
        TrafficLight::Green,
        TrafficLight::Yellow,
        TrafficLight::BlinkingGreen,
    ];

    for light in &lights {
        // TODO: 用 matches! 判断是否可以通行
        // 提示：Green 和 BlinkingGreen 都可以通行
        // 提示：用 | 分隔多个模式
        let can_go = false; // 替换为你的 matches! 表达式
        println!("  {:?} 可以通行？{}", light, can_go);
    }

    // TODO: 用 filter + matches! 筛选出所有可以通行的灯
    let go_lights: Vec<_> = lights.iter().collect(); // 替换为带 filter 的版本
    println!("  可以通行的灯：{:?}", go_lights);

    // --- while let 括号匹配练习 ---
    println!("\n=== 括号匹配练习 ===");

    fn check_brackets(s: &str) -> bool {
        let mut stack: Vec<char> = Vec::new();

        // TODO: 用 while let 和 chars().next() 的方式
        // 或者直接用 for 循环遍历 chars
        // 对于每个字符：
        //   - 如果是 '(' '{' '[' 就压入栈
        //   - 如果是 ')' '}' ']' 就检查栈顶是否匹配
        //   - 不匹配则返回 false

        // 最终检查栈是否为空
        stack.is_empty()
    }

    let test_cases = vec![
        ("(hello)", true),
        ("{[()]}", true),
        ("((())", false),
        ("{[}]", false),
        ("", true),
    ];

    for (input, expected) in test_cases {
        let result = check_brackets(input);
        let status = if result == expected { "PASS" } else { "FAIL" };
        println!("  [{}] \"{}\" -> {} (期望：{})", status, input, result, expected);
    }
}
*/

// =============================================================================
//  核心收获
// =============================================================================
/*
 * 核心收获：
 * - 枚举是 Rust 的核心类型之一，用 enum 定义，每个值只能是其中一个变体
 * - Option<T> 用 Some/None 替代 null，编译器强制你处理"值可能不存在"的情况
 * - match 是 Rust 最强大的控制流，支持解构、范围匹配、守卫条件，且必须穷尽所有情况
 *
 * 常见陷阱：
 * - 直接 unwrap() Option 而不处理 None，会导致程序 panic 崩溃
 * - match 漏掉某个分支（尤其是新增枚举变体时），编译器会报错——这是 Rust 在保护你
 *
 * 下节课预告：
 * - 下节课学错误处理（Result 和 ? 运算符），让程序遇到问题时不会崩溃，而是优雅地处理错误
 */
