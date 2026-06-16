// -*- coding: utf-8 -*-
// ============================================================================
// 第 10 课：迭代器与闭包
// ============================================================================
// 上节课我们学了集合类型。
// 这节课我们要学习迭代器和闭包——它们是 Rust 函数式编程的核心。
// 迭代器让你用声明式的方式处理数据，闭包让你写出更灵活的代码。
// 两者配合，可以写出既简洁又高效的代码。

use std::collections::HashMap;
use std::collections::HashSet;

// ============================================================================
//  辅助函数 / 闭包工厂（模块级，供各 demo 调用）
// ============================================================================

/// 给定阈值，返回一个过滤闭包：保留 >= threshold 的元素
fn make_filter(threshold: i32) -> impl Fn(&i32) -> bool {
    move |&x| x >= threshold
}

/// 给定因子，返回一个变换闭包：将 i32 乘以 factor 后取整
fn make_transformer(factor: f64) -> impl Fn(&i32) -> i32 {
    move |&x| (x as f64 * factor) as i32
}

/// 把闭包应用到 5 上
fn apply_to_five(f: impl Fn(i32) -> i32) -> i32 {
    f(5)
}

/// 对两个数执行给定的二元操作
fn apply_operation<F: Fn(i32, i32) -> i32>(a: i32, b: i32, op: F) -> i32 {
    op(a, b)
}

/// 调用闭包两次（Fn）
fn call_twice(f: impl Fn()) { f(); f(); }

/// 调用闭包两次（FnMut，允许修改捕获的变量）
fn call_and_modify(mut f: impl FnMut()) { f(); f(); }

/// 调用闭包一次（FnOnce，消耗所有权）
fn call_once(f: impl FnOnce()) { f(); }

/// 闭包工厂：返回一个乘以 factor 的闭包
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

/// 闭包工厂：返回一个从 start 开始的计数器闭包
fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        let current = count;
        count += 1;
        current
    }
}

// ============================================================================
//  demo 函数
// ============================================================================

/// 第一部分：迭代器基础 —— 传送带的故事
fn demo_iterator_basics() {
    // iter() 创建一个迭代器，每次 next() 返回 &T（借用引用）
    // 原集合不受影响，你只是"看了看"每个元素
    let numbers = vec![10, 20, 30, 40, 50];

    println!("1.1 iter() 借用遍历：");
    let mut iter = numbers.iter(); // 创建迭代器
    // next() 返回 Option<&i32>
    println!("   next(): {:?}", iter.next()); // Some(10)
    println!("   next(): {:?}", iter.next()); // Some(20)
    println!("   next(): {:?}", iter.next()); // Some(30)

    // 也可以用 for 循环（最常见的用法）
    print!("   for 循环：");
    for n in numbers.iter() {
        print!("{} ", n);
    }
    println!();
    println!("   numbers 还在：{:?}", numbers); // 没有被 move

    // iter_mut()：可变借用遍历，可以通过引用修改原集合中的元素
    let mut scores = vec![60, 70, 80, 90, 100];
    println!("\n1.2 iter_mut() 可变遍历：");
    println!("   修改前：{:?}", scores);

    for score in scores.iter_mut() {
        *score += 5; // 每个成绩加5分（通过可变引用修改）
    }
    println!("   每人加5分后：{:?}", scores);

    // into_iter()：消耗遍历，原集合被 move
    let names = vec![String::from("Alice"), String::from("Bob"), String::from("Charlie")];
    println!("\n1.3 into_iter() 消耗遍历：");
    for name in names.into_iter() {
        println!("   你好，{}！", name);
    }
    // println!("{:?}", names); // 取消注释会报错！names 已经被 into_iter 消耗了

    // 【三种 iter 总结】
    // iter()      -> &T   （借用，原集合不动）
    // iter_mut()  -> &mut T（可变借用，可以修改）
    // into_iter() -> T    （消耗，原集合被 move）
}

/// 第二部分：迭代器适配器 —— 传送带上的加工站
fn demo_adapters() {
    // map：变形
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("2.1 map（每个数乘2）：{:?} -> {:?}", numbers, doubled);

    // map 可以改变类型！
    let nums = vec![1, 2, 3, 4, 5];
    let strings: Vec<String> = nums.iter().map(|x| format!("第{}号", x)).collect();
    println!("   map 改变类型：{:?}", strings);

    // filter：过滤
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("\n2.2 filter（只保留偶数）：{:?}", evens);

    // filter + map 组合使用！
    let scores = vec![55, 68, 42, 90, 73, 38, 85];
    let passed_doubled: Vec<i32> = scores.iter()
        .filter(|&&s| s >= 60)     // 先过滤出及格的
        .map(|&s| s * 2)           // 再把分数翻倍
        .collect();
    println!("   filter+map（及格分数翻倍）：{:?}", passed_doubled);

    // enumerate：编号
    let fruits = vec!["苹果", "香蕉", "橘子", "葡萄"];
    println!("\n2.3 enumerate（编号）：");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("   第{}种水果：{}", index, fruit);
    }

    // zip：拉链
    let keys = vec!["a", "b", "c"];
    let values = vec![1, 2, 3];
    let pairs: Vec<(&str, i32)> = keys.iter().map(|&s| s).zip(values.iter().copied()).collect();
    println!("\n2.4 zip（配对）：{:?}", pairs);

    // 如果两个迭代器长度不同，以短的为准
    let short = vec![1, 2];
    let long = vec![10, 20, 30, 40];
    let zipped: Vec<(i32, i32)> = short.iter().copied().zip(long.iter().copied()).collect();
    println!("   长度不等时 zip：{:?}", zipped); // [(1,10), (2,20)]

    // chain：串联
    let first = vec![1, 2, 3];
    let second = vec![4, 5, 6];
    let chained: Vec<&i32> = first.iter().chain(second.iter()).collect();
    println!("\n2.5 chain（串联）：{:?}", chained);

    // take 和 skip
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_seven: Vec<&i32> = numbers.iter().skip(7).collect();
    let middle: Vec<&i32> = numbers.iter().skip(3).take(4).collect();
    println!("\n2.6 take/skip：");
    println!("   take(3)：{:?}", first_three);   // [1, 2, 3]
    println!("   skip(7)：{:?}", skip_seven);     // [8, 9, 10]
    println!("   skip(3).take(4)：{:?}", middle); // [4, 5, 6, 7]

    // flat_map：展平
    let sentences = vec!["hello world", "foo bar baz"];
    let words: Vec<&str> = sentences.iter().flat_map(|s| s.split(' ')).collect();
    println!("\n2.7 flat_map（拆分并展平）：{:?}", words);

    // 另一个例子：为每个数字生成它和它的平方
    let nums = vec![1, 2, 3];
    let expanded: Vec<i32> = nums.iter().flat_map(|&x| vec![x, x * x]).collect();
    println!("   每个数和它的平方：{:?}", expanded); // [1, 1, 2, 4, 3, 9]
}

/// 第三部分：消费者适配器 —— 传送带尽头的收货员
fn demo_consumers() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // sum：求和
    let total: i32 = numbers.iter().sum();
    println!("3.1 sum（求和）：{}", total); // 55

    // count：计数
    let count = numbers.iter().count();
    println!("3.2 count（计数）：{}", count); // 10

    // 配合 filter 使用：数一数有多少个偶数
    let even_count = numbers.iter().filter(|&&x| x % 2 == 0).count();
    println!("   偶数个数：{}", even_count); // 5

    // any 和 all
    let scores = vec![55, 68, 42, 90, 73];
    let any_pass = scores.iter().any(|&s| s >= 60);
    let all_pass = scores.iter().all(|&s| s >= 60);
    println!("\n3.3 any/all：");
    println!("   有没有人及格？{}", any_pass);   // true
    println!("   全部及格了吗？{}", all_pass);   // false

    // min 和 max
    let min = numbers.iter().min();
    let max = numbers.iter().max();
    println!("\n3.4 min/max：");
    println!("   最小值：{:?}", min); // Some(1)
    println!("   最大值：{:?}", max); // Some(10)

    // collect：收集
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("\n3.5 collect：{:?}", doubled);

    // 用 turbofish 语法 ::<Vec<i32>>() 指定类型
    let tripled = numbers.iter().map(|&x| x * 3).collect::<Vec<i32>>();
    println!("   turbofish collect：{:?}", tripled);

    // 收集成 HashSet（自动去重）
    let with_dup = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let unique: HashSet<i32> = with_dup.into_iter().collect();
    println!("   collect 到 HashSet：{:?}", unique);

    // fold：折叠
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = numbers.iter().fold(0, |acc, &x| {
        acc + x
    });
    println!("\n3.6 fold（求和）：{}", sum); // 15

    // fold 实现求积
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("   fold（求积）：{}", product); // 120

    // fold 实现字符串拼接
    let words = vec!["Hello", " ", "World", "!"];
    let sentence = words.iter().fold(String::new(), |mut acc, &s| {
        acc.push_str(s);
        acc
    });
    println!("   fold（拼接）：{}", sentence);

    // reduce：归约
    let numbers = vec![10, 20, 30, 40, 50];
    let sum = numbers.iter().copied().reduce(|acc, x| acc + x);
    println!("\n3.7 reduce（无初始值求和）：{:?}", sum); // Some(150)

    let empty: Vec<i32> = vec![];
    let nothing = empty.iter().copied().reduce(|acc, x| acc + x);
    println!("   空集合 reduce：{:?}", nothing); // None
}

/// 第四部分：迭代器链 —— 组合技
fn demo_iterator_chains() {
    // 找出所有偶数，把它们平方，然后求和
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: i32 = data.iter()
        .filter(|&&x| x % 2 == 0) // 第1步：只留偶数 -> [2,4,6,8,10]
        .map(|&x| x * x)          // 第2步：求平方   -> [4,16,36,64,100]
        .sum();                    // 第3步：求和     -> 220
    println!("4.1 偶数的平方和：{}", result);

    // 把一段文字中长度大于3的单词转成大写
    let text = "the quick brown fox jumps over the lazy dog";
    let long_words: Vec<String> = text.split_whitespace()
        .filter(|word| word.len() > 3)        // 长度 > 3 的单词
        .map(|word| word.to_uppercase())       // 转成大写
        .collect();
    println!("4.2 长单词大写：{:?}", long_words);

    // 链 + enumerate + filter
    let data = vec!["a", "b", "c", "d", "e", "f"];
    let odd_indexed: Vec<(usize, &str)> = data.iter()
        .enumerate()                   // 编号：(0,"a"), (1,"b"), ...
        .filter(|(i, _)| i % 2 == 1)  // 只留奇数编号
        .map(|(i, &s)| (i, s))        // 解引用
        .collect();
    println!("4.3 奇数位置的元素：{:?}", odd_indexed);
}

/// 第五部分：闭包基础 —— 随身携带的工具箱
fn demo_closure_basics() {
    // 最简单的闭包
    let say_hi = || println!("嗨！我是闭包！");
    say_hi();  // 调用闭包，就像调用函数一样
    say_hi();  // 可以反复调用

    // 带参数的闭包
    let add = |a, b| a + b;
    println!("5.2 3 + 5 = {}", add(3, 5));

    // 显式标注类型
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("5.3 4 * 6 = {}", multiply(4, 6));

    // 多行闭包
    let calculate = |x: i32| {
        let doubled = x * 2;     // 第一步：翻倍
        let result = doubled + 10; // 第二步：加10
        result                     // 最后一行是返回值（不加分号）
    };
    println!("5.4 calculate(5) = {}", calculate(5));   // 20

    // 闭包 vs 函数
    fn double_fn(x: i32) -> i32 { x * 2 }
    let double_closure = |x: i32| x * 2;
    println!("5.5 函数: double_fn(5) = {}", double_fn(5));
    println!("   闭包: double_closure(5) = {}", double_closure(5));
    // 效果一样！但闭包更灵活，可以捕获环境变量
}

/// 第六部分：闭包捕获环境 —— 工具箱里的材料
fn demo_closure_capture() {
    // 不可变借用捕获
    println!("6.1 不可变借用捕获：");
    let greeting = String::from("你好");
    let name = String::from("Rustacean");

    let say_hello = || println!("   {}, {}!", greeting, name);
    say_hello();
    say_hello(); // 可以反复调用，因为只是借用

    // 原始变量仍然可用！
    println!("   greeting 还在: {}", greeting);
    println!("   name 还在: {}", name);

    // 可变借用捕获
    println!("\n6.2 可变借用捕获：");
    let mut scores = vec![85, 92, 78];
    println!("   修改前：{:?}", scores);

    let mut add_score = |new_score: i32| {
        scores.push(new_score);
    };
    add_score(95);
    add_score(88);
    println!("   修改后：{:?}", scores); // [85, 92, 78, 95, 88]

    // 获取所有权（move）
    println!("\n6.3 获取所有权（move）：");
    let secret = String::from("绝密数据");

    let vault = move || {
        println!("   保险箱里的数据: {}", secret);
    };
    vault();
    vault(); // 闭包本身可以多次调用
    // println!("{}", secret); // 取消注释会报错！secret 已经被 move 了
    println!("   (secret 已经被 move 进闭包，原始变量不可用了)");
}

/// 第七部分：Fn / FnMut / FnOnce —— 工具箱的三种使用方式
fn demo_fn_traits() {
    // Fn：只读借用，可以反复调用
    println!("7.1 Fn trait（只读借用）：");
    let multiplier = 3;
    let times_three = |x: i32| x * multiplier; // 只读取了 multiplier
    println!("   times_three(4) = {}", times_three(4));   // 12
    println!("   times_three(5) = {}", times_three(5));   // 15
    // 可以无限次调用！

    // FnMut：可变借用，可以修改捕获的变量
    println!("\n7.2 FnMut trait（可变借用）：");
    let mut total = 0;
    let mut accumulate = |x: i32| {
        total += x;
        println!("   累加 {}，当前总计: {}", x, total);
    };
    accumulate(10);  // total = 10
    accumulate(20);  // total = 30
    accumulate(5);   // total = 35
    println!("   最终总计: {}", total);

    // FnOnce：获取所有权，只能调用一次
    println!("\n7.3 FnOnce trait（消耗所有权）：");
    let data = String::from("重要数据");
    let consume = || {
        println!("   正在处理: {}", data);
        drop(data); // 消耗了 data！
        println!("   数据已销毁");
    };
    consume();
    // consume(); // 再调用会报错！
}

/// 第八部分：闭包作为参数和返回值
fn demo_closure_as_param() {
    // 8.1 闭包作为函数参数
    println!("8.1 闭包作为参数：");
    println!("   apply_to_five(x * 3) = {}", apply_to_five(|x| x * 3));    // 15
    println!("   apply_to_five(x + 100) = {}", apply_to_five(|x| x + 100)); // 105
    println!("   10 + 3 = {}", apply_operation(10, 3, |a, b| a + b));
    println!("   10 * 3 = {}", apply_operation(10, 3, |a, b| a * b));

    // 8.2 区分 Fn、FnMut、FnOnce 参数
    println!("\n8.2 三种参数类型：");

    let msg = "你好";
    call_twice(|| print!("   Fn: {} ", msg));
    println!();

    let mut count = 0;
    call_and_modify(|| {
        count += 1;
        print!("   FnMut: {} ", count);
    });
    println!();

    let data = String::from("一次性数据");
    call_once(|| {
        println!("   FnOnce: {}", data);
        drop(data);
    });

    // 8.3 闭包作为返回值 —— 闭包工厂
    println!("\n8.3 闭包工厂：");

    let double = make_multiplier(2);
    let triple = make_multiplier(3);
    let ten_times = make_multiplier(10);
    println!("   double(5) = {}", double(5));       // 10
    println!("   triple(5) = {}", triple(5));       // 15
    println!("   ten_times(5) = {}", ten_times(5)); // 50

    // 8.4 计数器工厂
    println!("\n8.4 计数器工厂：");

    let mut counter_a = make_counter(0);
    let mut counter_b = make_counter(100);
    println!("   counter_a: {}, {}, {}", counter_a(), counter_a(), counter_a()); // 0, 1, 2
    println!("   counter_b: {}, {}", counter_b(), counter_b());                 // 100, 101
    println!("   counter_a 继续: {}", counter_a()); // 3（两个计数器互不影响！）
}

/// 第九部分：闭包与迭代器配合 —— 核心！
fn demo_closure_with_iterator() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 9.1 闭包作为 map 的参数
    println!("9.1 闭包作为 map 的参数：");

    let offset = 100;
    let with_offset: Vec<i32> = numbers.iter().map(|&x| x + offset).collect();
    println!("   每个数加 offset({})：{:?}", offset, with_offset);

    let labels: Vec<String> = numbers.iter()
        .map(|x| format!("No.{:03}", x))  // 1 -> "No.001"
        .collect();
    println!("   格式化标签：{:?}", &labels[..5]); // 只显示前5个

    // 9.2 闭包作为 filter 的参数
    println!("\n9.2 闭包作为 filter 的参数：");

    let threshold = 5;
    let above: Vec<&i32> = numbers.iter().filter(|&&x| x > threshold).collect();
    println!("   大于 {} 的数：{:?}", threshold, above);

    let special: Vec<&i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0 && x > 4)  // 偶数且大于4
        .collect();
    println!("   偶数且大于4：{:?}", special);

    // 9.3 闭包作为 fold 的参数
    println!("\n9.3 闭包作为 fold 的参数：");

    let stats = numbers.iter().fold(
        (0i32, 0i32, i32::MAX),  // (总和, 计数, 最小值)
        |(sum, count, min), &x| {
            (sum + x, count + 1, if x < min { x } else { min })
        }
    );
    println!("   (总和={}, 计数={}, 最小值={})", stats.0, stats.1, stats.2);

    let csv = numbers.iter().fold(String::new(), |mut acc, x| {
        if !acc.is_empty() { acc.push_str(", "); }
        acc.push_str(&x.to_string());
        acc
    });
    println!("   CSV: {}", csv);

    // 9.4 闭包捕获外部状态的迭代器链
    println!("\n9.4 闭包捕获外部状态的迭代器链：");

    let min_score = 60;
    let bonus = 10;
    let raw_scores = vec![55, 72, 48, 88, 63, 91, 42];

    let boosted_passing: Vec<i32> = raw_scores.iter()
        .filter(|&&s| s >= min_score)       // 闭包捕获 min_score
        .map(|&s| (s + bonus).min(100))     // 闭包捕获 bonus
        .collect();
    println!("   原始成绩：{:?}", raw_scores);
    println!("   及格(>={})且加分(+{})后：{:?}", min_score, bonus, boosted_passing);

    // 9.5 闭包工厂 + 迭代器 = 高度复用
    println!("\n9.5 闭包工厂 + 迭代器 = 高度复用：");

    let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // 用工厂生产的闭包来构建迭代器链
    let result: Vec<i32> = data.iter()
        .filter(make_filter(50))          // 留下 >= 50 的
        .map(make_transformer(1.5))       // 每个数乘以 1.5
        .collect();
    println!("   原始数据：{:?}", data);
    println!("   >= 50 且 x1.5：{:?}", result);
}

// ============================================================================
//  main：只负责调用各 demo 函数
// ============================================================================

fn main() {
    println!("============================================================");
    println!(" 第一部分：迭代器基础 —— 传送带");
    println!("============================================================\n");
    demo_iterator_basics();

    println!("\n============================================================");
    println!(" 第二部分：迭代器适配器 —— 加工站");
    println!("============================================================\n");
    demo_adapters();

    println!("\n============================================================");
    println!(" 第三部分：消费者适配器 —— 收货员");
    println!("============================================================\n");
    demo_consumers();

    println!("\n============================================================");
    println!(" 第四部分：迭代器链 —— 组合技");
    println!("============================================================\n");
    demo_iterator_chains();

    println!("\n============================================================");
    println!(" 第五部分：闭包基础 —— 随身携带的工具箱");
    println!("============================================================\n");
    demo_closure_basics();

    println!("\n============================================================");
    println!(" 第六部分：闭包捕获环境");
    println!("============================================================\n");
    demo_closure_capture();

    println!("\n============================================================");
    println!(" 第七部分：Fn / FnMut / FnOnce");
    println!("============================================================\n");
    demo_fn_traits();

    println!("\n============================================================");
    println!(" 第八部分：闭包作为参数和返回值");
    println!("============================================================\n");
    demo_closure_as_param();

    println!("\n============================================================");
    println!(" 第九部分：闭包与迭代器配合 —— 核心！");
    println!("============================================================\n");
    demo_closure_with_iterator();

    println!("\n============================================================");
    println!(" 第十部分：实战项目");
    println!("============================================================\n");

    // 项目1：data_pipeline_demo 已降级为练习 3 参考答案（见练习题区域）

    // 项目2：SQL-like 查询引擎
    sql_like_query_demo();
}

// ============================================================================
//  实战项目：SQL-like 查询引擎
// ============================================================================

/// SQL-like 查询引擎
/// 用迭代器链 + 闭包模拟 SQL 的 WHERE / ORDER BY / SELECT / GROUP BY
fn sql_like_query_demo() {
    println!("--- 项目2：SQL-like 查询引擎 ---\n");

    // 模拟数据库表：学生信息
    #[derive(Debug, Clone)]
    struct Student {
        name: String,
        age: u32,
        score: f64,
        city: String,
    }

    let students = vec![
        Student { name: "Alice".to_string(),   age: 20, score: 92.5, city: "北京".to_string() },
        Student { name: "Bob".to_string(),     age: 22, score: 78.0, city: "上海".to_string() },
        Student { name: "Charlie".to_string(), age: 19, score: 88.5, city: "北京".to_string() },
        Student { name: "David".to_string(),   age: 21, score: 95.0, city: "广州".to_string() },
        Student { name: "Eve".to_string(),     age: 20, score: 65.0, city: "上海".to_string() },
        Student { name: "Frank".to_string(),   age: 23, score: 82.0, city: "北京".to_string() },
        Student { name: "Grace".to_string(),   age: 19, score: 91.0, city: "广州".to_string() },
        Student { name: "Helen".to_string(),   age: 20, score: 73.5, city: "上海".to_string() },
    ];

    // ---- 查询1：WHERE + ORDER BY + SELECT ----
    // SQL: SELECT name, score FROM students WHERE score >= 85 ORDER BY score DESC
    println!("查询1: SELECT name, score WHERE score >= 85 ORDER BY score DESC");

    let mut result: Vec<(&str, f64)> = students.iter()
        .filter(|s| s.score >= 85.0)                     // WHERE score >= 85
        .map(|s| (s.name.as_str(), s.score))             // SELECT name, score
        .collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // ORDER BY score DESC

    for (name, score) in &result {
        println!("   {} - {:.1}分", name, score);
    }

    // ---- 查询2：复合 WHERE ----
    // SQL: SELECT name, age, city WHERE age <= 20 AND city = '北京' ORDER BY age
    println!("\n查询2: SELECT name, age, city WHERE age <= 20 AND city = '北京'");

    let mut result2: Vec<(&str, u32, &str)> = students.iter()
        .filter(|s| s.age <= 20 && s.city == "北京")
        .map(|s| (s.name.as_str(), s.age, s.city.as_str()))
        .collect();
    result2.sort_by(|a, b| a.1.cmp(&b.1));

    for (name, age, city) in &result2 {
        println!("   {} - {}岁 - {}", name, age, city);
    }

    // ---- 查询3：聚合函数 ----
    // SQL: SELECT AVG(score), MAX(score), MIN(score), COUNT(*) FROM students
    println!("\n查询3: 聚合统计 (AVG, MAX, MIN, COUNT)");

    let count = students.len() as f64;
    let total: f64 = students.iter().map(|s| s.score).sum();
    let avg = total / count;
    let max_score = students.iter().map(|s| s.score).fold(f64::MIN, f64::max);
    let min_score = students.iter().map(|s| s.score).fold(f64::MAX, f64::min);

    println!("   人数: {}", students.len());
    println!("   平均分: {:.2}", avg);
    println!("   最高分: {:.1}", max_score);
    println!("   最低分: {:.1}", min_score);

    // ---- 查询4：GROUP BY ----
    // SQL: SELECT city, COUNT(*), AVG(score) FROM students GROUP BY city
    println!("\n查询4: GROUP BY city (人数 + 平均分)");

    let mut groups: HashMap<&str, Vec<f64>> = HashMap::new();
    for s in &students {
        groups.entry(s.city.as_str())
            .or_insert_with(Vec::new)
            .push(s.score);
    }

    // 按平均分降序输出
    let mut group_vec: Vec<(&str, usize, f64)> = groups.iter()
        .map(|(&city, scores)| {
            let avg = scores.iter().sum::<f64>() / scores.len() as f64;
            (city, scores.len(), avg)
        })
        .collect();
    group_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    for (city, count, avg) in &group_vec {
        println!("   {}: {} 人, 平均分 {:.2}", city, count, avg);
    }

    // ---- 查询5：子查询 ----
    // 找出每个城市最高分的学生
    println!("\n查询5: 每个城市最高分的学生");

    for &city in &["北京", "上海", "广州"] {
        let top = students.iter()
            .filter(|s| s.city == city)
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        if let Some(s) = top {
            println!("   {}: {} ({:.1}分)", city, s.name, s.score);
        }
    }
}

// ============================================================================
//  练习题
// ============================================================================

/*
 * ============================================================
 * 练习 1（基础巩固，5-10 分钟）：apply_twice
 * ============================================================
 *
 * 题目：
 *   实现 apply_twice(f, x)，把闭包 f 应用到 x 上两次，即 f(f(x))
 *
 * 提示：
 *   - 参数类型：f: impl Fn(i32) -> i32, x: i32
 *   - 返回类型：i32
 *   - 先算 f(x)，再把结果传给 f
 *
 * 测试用例：
 *   apply_twice(|x| x * 2, 3)   == 12  （3*2=6, 6*2=12）
 *   apply_twice(|x| x + 1, 10)  == 12  （10+1=11, 11+1=12）
 *   apply_twice(|x| x * x, 3)   == 81  （3*3=9, 9*9=81）
 */

// fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
//     // 在这里写你的代码：
//     f(f(x))
// }

// ===== 测试版练习 =====
// 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
// 取消下面的注释，运行 cargo test --example lesson10

/*
#[cfg(test)]
mod tests {
    // apply_twice 的参考实现
    fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(f(x))
    }

    #[test]
    fn test_apply_twice() {
        // 翻倍两次：3 -> 6 -> 12
        assert_eq!(apply_twice(|x| x * 2, 3), 12);
        // 加一两次：10 -> 11 -> 12
        assert_eq!(apply_twice(|x| x + 1, 10), 12);
        // 平方两次：3 -> 9 -> 81
        assert_eq!(apply_twice(|x| x * x, 3), 81);
        // 负数也适用：-2 -> -4 -> -8
        assert_eq!(apply_twice(|x| x * 2, -2), -8);
    }
}
*/

/*
 * ============================================================
 * 练习 2（应用练习，10-15 分钟）：闭包工厂
 * ============================================================
 *
 * 题目：
 *   实现一个"阶梯折扣"闭包工厂 make_discount(threshold, rate)
 *   - 当金额 >= threshold 时，打折 rate（如 0.8 表示八折）
 *   - 当金额 < threshold 时，不打折
 *   - 返回一个闭包：fn(f64) -> f64
 *
 * 提示：
 *   - 用 move 获取 threshold 和 rate 的所有权
 *   - 返回 impl Fn(f64) -> f64
 *
 * 测试用例：
 *   let vip_discount = make_discount(100.0, 0.8);
 *   vip_discount(50.0)   == 50.0   （不到100，不打折）
 *   vip_discount(200.0)  == 160.0  （200 * 0.8 = 160）
 */

// fn make_discount(threshold: f64, rate: f64) -> impl Fn(f64) -> f64 {
//     // 在这里写你的代码：
//     move |amount| {
//         if amount >= threshold {
//             amount * rate
//         } else {
//             amount
//         }
//     }
// }

/*
 * ============================================================
 * 练习 3（进阶挑战，15-20 分钟）：迭代器链式操作
 * ============================================================
 *
 * 题目：
 *   给定一组学生成绩数据，用迭代器链完成以下任务：
 *
 *   let students = vec![
 *       ("Alice", 85),
 *       ("Bob", 52),
 *       ("Charlie", 73),
 *       ("Diana", 46),
 *       ("Eve", 91),
 *       ("Frank", 67),
 *       ("Grace", 78),
 *       ("Helen", 55),
 *   ];
 *
 *   1. 过滤出及格（>=60）的学生
 *   2. 把他们的成绩提高 10%（乘以 1.1，上限100）
 *   3. 按成绩从高到低排序
 *   4. 收集成 Vec<(String, f64)>
 *   5. 输出排名
 *
 * 提示：
 *   - filter -> map -> collect -> sort_by
 *   - f64 的比较用 partial_cmp 而不是 cmp
 *   - 用 .min(100.0) 限制最大值
 *
 * 期望输出：
 *   第1名：Eve - 100.0分
 *   第2名：Alice - 93.5分
 *   第3名：Grace - 85.8分
 *   第4名：Charlie - 80.3分
 *   第5名：Frank - 73.7分
 */

// fn exercise_3_data_pipeline() {
//     let students = vec![
//         ("Alice", 85.0),
//         ("Bob", 52.0),
//         ("Charlie", 73.0),
//         ("Diana", 46.0),
//         ("Eve", 91.0),
//         ("Frank", 67.0),
//         ("Grace", 78.0),
//         ("Helen", 55.0),
//     ];
//
//     // 在这里写你的代码：
//     let mut results: Vec<(String, f64)> = students.iter()
//         .filter(|(_, score)| *score >= 60.0)           // 过滤及格的
//         .map(|(name, score)| {                          // 提分
//             let boosted = (score * 1.1).min(100.0);
//             (name.to_string(), boosted)
//         })
//         .collect();
//
//     results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // 降序排序
//
//     for (rank, (name, score)) in results.iter().enumerate() {
//         println!("  第{}名：{} - {:.1}分", rank + 1, name, score);
//     }
// }

/*
 * ============================================================
 * 练习 3 参考答案：完整数据管道（原实战项目 1）
 * ============================================================
 *
 * 下面是 data_pipeline_demo()，展示如何用迭代器链 + 闭包
 * 处理真实的电商订单数据，包括：
 *   - filter + map + sum 求已完成订单总金额
 *   - filter + map + collect + sort_by 多条件过滤排序
 *   - HashMap 分组统计各类别订单数和金额
 *
 * 运行方式：取消下方 main 中 data_pipeline_demo() 的注释
 */

/// 数据管道演示（练习 3 参考答案）
fn data_pipeline_demo() {
    println!("--- 练习3 参考答案：数据管道 ---\n");

    #[derive(Debug)]
    struct Order {
        id: u32,
        product: String,
        amount: f64,
        category: String,
        is_completed: bool,
    }

    let orders = vec![
        Order { id: 1, product: "笔记本电脑".to_string(), amount: 5999.0, category: "电子".to_string(), is_completed: true },
        Order { id: 2, product: "鼠标".to_string(), amount: 99.0, category: "电子".to_string(), is_completed: true },
        Order { id: 3, product: "T恤".to_string(), amount: 129.0, category: "服装".to_string(), is_completed: false },
        Order { id: 4, product: "显示器".to_string(), amount: 2499.0, category: "电子".to_string(), is_completed: true },
        Order { id: 5, product: "牛仔裤".to_string(), amount: 299.0, category: "服装".to_string(), is_completed: true },
        Order { id: 6, product: "键盘".to_string(), amount: 399.0, category: "电子".to_string(), is_completed: false },
        Order { id: 7, product: "运动鞋".to_string(), amount: 599.0, category: "服装".to_string(), is_completed: true },
        Order { id: 8, product: "耳机".to_string(), amount: 899.0, category: "电子".to_string(), is_completed: true },
    ];

    // 管道1：已完成订单的总金额
    let completed_total: f64 = orders.iter()
        .filter(|o| o.is_completed)
        .map(|o| o.amount)
        .sum();
    println!("管道1 - 已完成订单总金额: {:.2} 元", completed_total);

    // 管道2：电子类已完成订单，按金额降序排列
    println!("\n管道2 - 电子类已完成订单（按金额降序）：");
    let mut electronics: Vec<(&str, f64)> = orders.iter()
        .filter(|o| o.is_completed && o.category == "电子")
        .map(|o| (o.product.as_str(), o.amount))
        .collect();
    electronics.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (i, (product, amount)) in electronics.iter().enumerate() {
        println!("   {}. {} - {:.2} 元", i + 1, product, amount);
    }

    // 管道3：各类别的订单统计
    println!("\n管道3 - 各类别统计：");
    let mut category_stats: HashMap<&str, (u32, f64)> = HashMap::new();
    for order in &orders {
        let entry = category_stats.entry(order.category.as_str()).or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += order.amount;
    }
    for (category, (count, total)) in &category_stats {
        println!("   {}: {} 笔订单, 总金额 {:.2} 元, 平均 {:.2} 元",
            category, count, total, total / *count as f64);
    }
}

// 取消下面的注释来运行练习题：
// fn main() {
//     println!("--- 练习1：apply_twice ---");
//     println!("apply_twice(double, 3) = {}", apply_twice(|x| x * 2, 3));
//     println!("apply_twice(add_one, 10) = {}", apply_twice(|x| x + 1, 10));
//     println!("apply_twice(square, 3) = {}", apply_twice(|x| x * x, 3));
//
//     println!("\n--- 练习2：闭包工厂 ---");
//     let vip = make_discount(100.0, 0.8);
//     println!("vip(50) = {}", vip(50.0));
//     println!("vip(200) = {}", vip(200.0));
//
//     println!("\n--- 练习3：数据管道 ---");
//     exercise_3_data_pipeline();
// }

// ============================================================================
// 结尾
// ============================================================================

/*
 * ============================================================
 * 核心收获：
 * ============================================================
 *
 * 1. 迭代器是"传送带"，让元素一个一个到你面前：
 *    - iter() 借用、iter_mut() 可变借用、into_iter() 消耗
 *    - 适配器（map/filter/enumerate/zip/chain/take/skip/flat_map）是"加工站"
 *    - 消费者（sum/count/any/all/min/max/collect/fold/reduce）是"收货员"
 *    - 迭代器链式调用 = 流水线，惰性求值，高效优雅
 *
 * 2. 闭包 = 匿名函数 + 捕获环境，就像随身携带的工具箱：
 *    - |参数| 表达式 是闭包的语法
 *    - Fn（只读借用） ⊂ FnMut（可变借用） ⊂ FnOnce（消耗）
 *    - 闭包可以作为函数参数（impl Fn / F: Fn）
 *    - 闭包可以作为返回值（必须用 move，闭包工厂模式）
 *
 * 3. 闭包 + 迭代器 = Rust 函数式编程的核心：
 *    - map/filter/fold 的参数就是闭包
 *    - 闭包可以捕获外部状态，让迭代器链更灵活
 *    - 闭包工厂 + 迭代器 = 高度可复用的数据处理管道
 *
 * ============================================================
 * 常见陷阱：
 * ============================================================
 *
 * 1. 返回闭包时忘记 move：
 *    - 局部变量被借用后函数结束就销毁了，必须 move 进闭包
 *    - 错误写法：fn make() -> impl Fn() { let x = 1; || println!("{}", x) }
 *    - 正确写法：fn make() -> impl Fn() { let x = 1; move || println!("{}", x) }
 *
 * 2. 闭包类型推断的"首次确定"规则：
 *    - 第一次调用确定类型后，不能用不同类型再调用
 *    - let f = |x| x; f(1); f("hi"); // 错误！类型已确定为 i32
 *
 * 3. 迭代器的惰性求值：
 *    - 适配器不会立刻执行！只有消费者才会触发
 *    - let _ = vec![1,2,3].iter().map(|x| x * 2); // 什么都不做！
 *    - 必须加 .collect() 或 .sum() 等消费者
 *
 * ============================================================
 * 下节课预告：
 * ============================================================
 *
 * 下节课我们学习模块系统（Module System）—— 把代码组织得井井有条。
 * 你会学到：
 *   - mod 关键字定义模块
 *   - pub 控制可见性
 *   - use 引入路径
 *   - 文件级别的模块拆分
 *   - 包（Package）和 Crate 的概念
 *
 * 模块系统让你的代码从"一个大文件"变成"多个小模块"，
 * 就像把一间乱糟糟的房间整理成多个抽屉一样！
 */
