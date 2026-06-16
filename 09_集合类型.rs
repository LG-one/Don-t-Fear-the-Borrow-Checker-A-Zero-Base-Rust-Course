// -*- coding: utf-8 -*-
// ============================================================================
// 第 09 课：集合类型
// ============================================================================
//
// 【写给零基础的你】
//
// 欢迎来到第九课！
//
// 到目前为止，我们学过变量、结构体、枚举……但它们一次只能装一个东西。
// 可现实生活中，我们经常需要处理"一堆东西"：
//   - 购物车里有很多商品
//   - 字典里有很多词条
//   - 班级名单里有很多同学
//
// Rust 提供了几种"集合"类型，专门用来装"一堆东西"。
// 今天我们就来认识这些"容器"，学会往里放东西、取东西、查找和统计。
//
// 本节课目标：
//   1. 学会使用 Vec<T>——可变长度的列表（购物车）
//   2. 学会使用 String——字符串的深入操作
//   3. 学会使用 HashMap<K,V>——键值对字典
//   4. 学会使用 HashSet<T>——不重复的集合
//   5. 了解 BTreeMap / BTreeSet——有序集合
//   6. 完成学生成绩管理系统的实战项目
//
// 前置知识：变量、控制流、所有权、结构体、生命周期、枚举、错误处理、泛型与 Trait
//

fn main() {
    // =============================================================
    // 第一部分：Vec<T>——购物车的故事
    // =============================================================
    //
    // 【生活类比：购物车】
    // Vec<T> 就是 Rust 里的"购物车"：
    //   - push 放东西、pop 取东西、len 看数量
    //   - 按编号访问、修改、清空……
    //   - <T> 表示可以装任何类型：Vec<i32> 装整数，Vec<String> 装字符串
    //
    println!("============================================================");
    println!(" 第一部分：Vec<T> —— 购物车");
    println!("============================================================\n");

    // --- 1.1 创建 Vec ---
    // 方式一：Vec::new() 创建空购物车（需要类型标注）
    let mut cart1: Vec<i32> = Vec::new();
    println!("1.1 创建了一个空购物车 cart1，里面有 {} 件商品", cart1.len());

    // 方式二：vec! 宏——直接把东西放进去（类型自动推断）
    let mut cart2 = vec![10, 20, 30, 40, 50];
    println!("   创建了一个带初始商品的购物车 cart2：{:?}", cart2);

    // --- 1.2 push：往购物车末尾放东西 ---
    cart1.push(100);
    cart1.push(200);
    cart1.push(300);
    println!("\n1.2 往 cart1 里放了三件商品：{:?}", cart1);

    // --- 1.3 按编号访问 ---
    // 方括号 + 编号，编号从 0 开始！
    let first_item = cart2[0]; // 第0个商品
    let third_item = cart2[2]; // 第2个商品
    println!("\n1.3 cart2 的第0个商品：{}", first_item);
    println!("   cart2 的第2个商品：{}", third_item);

    // 【安全访问：get 方法】
    // 用 cart2[99] 会崩溃！更安全的方式是 get()，返回 Option<T>
    match cart2.get(99) {
        Some(value) => println!("   cart2 的第99个商品：{}", value),
        None => println!("   cart2 没有第99个商品（越界了）"),
    }

    // --- 1.4 修改和弹出 ---
    cart2[0] = 999; // 直接赋值修改
    println!("\n1.4 修改后 cart2：{:?}", cart2);

    // pop 取出末尾最后一个，返回 Option（可能为空）
    match cart2.pop() {
        Some(item) => println!("   从末尾取出：{}", item),
        None => println!("   cart2 是空的"),
    }
    println!("   取出后 cart2：{:?}", cart2);

    // --- 1.5 插入和删除 ---
    cart2.insert(1, 888); // 在位置1插入888
    println!("\n1.5 在位置1插入888后：{:?}", cart2);
    let removed = cart2.remove(1); // 删除位置1
    println!("   删除位置1（{}）后：{:?}", removed, cart2);

    // --- 1.6 常用方法 ---
    println!("\n1.6 常用方法：");
    println!("   长度：{}", cart2.len());
    println!("   空购物车是空的？{}", Vec::<i32>::new().is_empty());
    println!("   有 20？{}", cart2.contains(&20));
    match cart2.first() {
        Some(v) => println!("   第一件：{}", v),
        None => println!("   空的"),
    }
    match cart2.last() {
        Some(v) => println!("   最后一件：{}", v),
        None => println!("   空的"),
    }

    // --- 1.7 Vec 与所有权 ---
    // Vec 拥有里面的每个元素。String 放进 Vec 后所有权转移！
    let name = String::from("Rust");
    let mut names = Vec::new();
    names.push(name); // name 的所有权转移给了 names
    // println!("{}", name); // 报错！name 已经被 move 了
    println!("\n1.7 所有权转移后，names：{:?}", names);

    // --- 1.8 遍历 Vec ---
    // 用 for 循环遍历，&item 表示借用每个元素
    println!("\n1.8 遍历 cart2：");
    let mut index = 0;
    for &item in &cart2 {
        println!("   第 {} 个商品：{}", index, item);
        index += 1;
    }

    // =============================================================
    // 第二部分：String —— 文字处理的艺术
    // =============================================================
    //
    // 【String vs &str：咖啡的故事】
    //   - String = 你买了一杯咖啡，你拥有它，可以喝、加糖、倒掉
    //   - &str   = 你借了朋友的咖啡看一眼，只能看，不能动
    //   - String 在堆上分配内存，&str 只是借用的"窗口"
    //
    println!("\n============================================================");
    println!(" 第二部分：String —— 文字处理的艺术");
    println!("============================================================\n");

    // --- 2.1 创建 String ---
    let mut s1 = String::new();              // 空字符串
    let s2 = String::from("你好世界");        // 从字面量创建
    let s3 = "Hello Rust".to_string();       // 另一种写法
    println!("2.1 s1='{}', s2={}, s3={}", s1, s2, s3);

    // --- 2.2 push 和 push_str ---
    s1.push('你');           // 加一个字符
    s1.push('好');
    s1.push_str("，世界！"); // 加一段文字
    println!("\n2.2 push 后 s1：{}", s1);

    // --- 2.3 拼接字符串 ---
    // 方式一：+ 运算符（左边 String + 右边 &str，左边会被 move）
    let hello = String::from("Hello");
    let world = String::from(" World");
    let greeting = hello + &world; // hello 被 move 了
    println!("\n2.3 + 拼接：{}", greeting);
    println!("   world 还在：{}", world);

    // 方式二：format! 宏（推荐！不夺取任何变量的所有权）
    let name = "G-one";
    let age = 25;
    let intro = format!("我是 {}，今年 {} 岁", name, age);
    println!("   format! 拼接：{}", intro);

    // --- 2.4 UTF-8：字符 vs 字节 ---
    // Rust 的 String 是 UTF-8 编码：英文=1字节，中文=3字节，emoji=4字节
    // len() 返回字节数，不是字符数！
    let text = "你好Hello";
    println!("\n2.4 '{}' 的字节数：{}，字符数：{}", text, text.len(), text.chars().count());

    print!("   逐字符：");
    for ch in text.chars() { print!("[{}]", ch); }
    println!();
    print!("   逐字节：");
    for b in text.bytes() { print!("[{}]", b); }
    println!();

    // --- 2.5 字符串切片注意事项 ---
    // 不能在 UTF-8 字符中间切片，否则 panic！
    let cn = String::from("你好");
    // let bad = &cn[0..2]; // panic！"你"占3字节
    let good = &cn[0..3];  // 正确！完整的"你"
    println!("\n2.5 切片：{}", good);

    // --- 2.6 常用字符串方法 ---
    let sentence = "  Hello, Rust World!  ";
    println!("\n2.6 常用方法：");
    println!("   trim：'{}'", sentence.trim());
    println!("   to_uppercase：{}", sentence.trim().to_uppercase());
    println!("   contains('Rust')：{}", sentence.contains("Rust"));
    println!("   replace('World','G-one')：{}", sentence.trim().replace("World", "G-one"));

    // split 按分隔符拆分
    let csv = "苹果,香蕉,橘子,葡萄";
    print!("   split：");
    for fruit in csv.split(',') { print!("[{}]", fruit); }
    println!();

    // =============================================================
    // 第三部分：HashMap<K,V> —— 字典的故事
    // =============================================================
    //
    // 【生活类比：字典】
    // 用"词语"（Key）查找"解释"（Value），每个词语唯一，查找极快！
    //
    use std::collections::HashMap;

    println!("\n============================================================");
    println!(" 第三部分：HashMap —— 字典");
    println!("============================================================\n");

    // --- 3.1 创建和插入 ---
    let mut dict: HashMap<String, String> = HashMap::new();
    dict.insert(String::from("apple"), String::from("苹果"));
    dict.insert(String::from("banana"), String::from("香蕉"));
    dict.insert(String::from("cherry"), String::from("樱桃"));
    println!("3.1 插入三个词条：{:?}", dict);

    // insert 会覆盖同名键！
    dict.insert(String::from("apple"), String::from("红苹果"));
    println!("   覆盖 apple 后：{:?}", dict);

    // --- 3.2 查找 ---
    println!("\n3.2 查找：");
    match dict.get("apple") {
        Some(v) => println!("   apple = {}", v),
        None => println!("   没有 apple"),
    }
    match dict.get("dog") {
        Some(v) => println!("   dog = {}", v),
        None => println!("   没有 dog"),
    }

    // --- 3.3 遍历 ---
    // 注意：HashMap 遍历顺序不确定！每次运行可能不同
    println!("\n3.3 遍历：");
    for (key, value) in &dict {
        println!("   {} = {}", key, value);
    }

    // --- 3.4 删除 ---
    match dict.remove("banana") {
        Some(val) => println!("\n3.4 删除了 banana = {}", val),
        None => println!("   没有 banana"),
    }

    // --- 3.5 entry API —— 智能插入 ---
    // 如果键存在就修改，不存在就创建——一步到位！
    // 就像酒店前台："给我301号房，没人住就开一间"
    println!("\n3.5 entry API（单词计数）：");
    let words = vec!["hello", "world", "hello", "rust", "hello", "world"];
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in &words {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("   {:?}", word_count);

    // or_insert_with：用闭包生成默认值（延迟计算）
    let mut prices: HashMap<&str, Vec<f64>> = HashMap::new();
    prices.entry("apple").or_insert_with(Vec::new).push(3.5);
    prices.entry("apple").or_insert_with(Vec::new).push(4.0);
    println!("   价格：{:?}", prices);

    // =============================================================
    // 第四部分：HashSet<T> —— 班级名单的故事
    // =============================================================
    //
    // HashSet 是"不能有重复"的集合。
    // 可以做并集、交集、差集运算。
    //
    use std::collections::HashSet;

    println!("\n============================================================");
    println!(" 第四部分：HashSet —— 班级名单");
    println!("============================================================\n");

    // --- 4.1 创建和插入 ---
    let mut class_a: HashSet<&str> = HashSet::new();
    class_a.insert("小明");
    class_a.insert("小红");
    class_a.insert("小刚");
    class_a.insert("小丽");
    println!("4.1 A班：{:?}", class_a);
    println!("   再次插入小明，成功？{}", class_a.insert("小明")); // false

    // --- 4.2 检查包含 ---
    println!("\n4.2 有小明？{}，有小华？{}", class_a.contains("小明"), class_a.contains("小华"));

    // --- 4.3 集合运算 ---
    let mut class_b: HashSet<&str> = HashSet::new();
    class_b.insert("小红");
    class_b.insert("小刚");
    class_b.insert("小华");
    class_b.insert("小强");

    println!("\n4.3 A班：{:?}", class_a);
    println!("   B班：{:?}", class_b);

    let union: HashSet<_> = class_a.union(&class_b).cloned().collect();
    println!("   并集（A∪B）：{:?}", union);

    let intersection: HashSet<_> = class_a.intersection(&class_b).cloned().collect();
    println!("   交集（A∩B）：{:?}", intersection);

    let diff_a: HashSet<_> = class_a.difference(&class_b).cloned().collect();
    println!("   差集（A-B）：{:?}", diff_a);

    let sym_diff: HashSet<_> = class_a.symmetric_difference(&class_b).cloned().collect();
    println!("   对称差集：{:?}", sym_diff);

    // --- 4.4 去重 ---
    let numbers = vec![1, 2, 3, 2, 1, 4, 3, 5];
    let mut unique_set: HashSet<&i32> = HashSet::new();
    for n in &numbers { unique_set.insert(n); }
    println!("\n4.4 去重：{:?} -> {:?}", numbers, unique_set);

    // =============================================================
    // 第五部分：BTreeMap / BTreeSet —— 有序集合
    // =============================================================
    //
    // HashMap 遍历顺序随机，BTreeMap 按键排序。
    // 需要有序遍历或找最小/最大键时，用 BTreeMap。
    //
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    println!("\n============================================================");
    println!(" 第五部分：BTreeMap / BTreeSet —— 有序集合");
    println!("============================================================\n");

    let mut btree_map = BTreeMap::new();
    btree_map.insert(3, "橘子");
    btree_map.insert(1, "苹果");
    btree_map.insert(2, "香蕉");
    btree_map.insert(5, "葡萄");
    btree_map.insert(4, "樱桃");

    println!("5.1 BTreeMap（按键排序）：");
    for (key, value) in &btree_map {
        println!("   {} -> {}", key, value);
    }

    let mut btree_set = BTreeSet::new();
    btree_set.insert(50);
    btree_set.insert(10);
    btree_set.insert(30);
    btree_set.insert(20);
    btree_set.insert(40);
    println!("\n5.2 BTreeSet（自动排序）：{:?}", btree_set);
    // 输出一定是 {10, 20, 30, 40, 50}

    // =============================================================
    // 第六部分：实战项目 —— 学生成绩管理系统
    // =============================================================
    //
    // 综合运用 Vec、HashMap、结构体、for 循环构建成绩管理系统
    //

    println!("\n============================================================");
    println!(" 第六部分：实战项目 —— 学生成绩管理系统");
    println!("============================================================\n");

    student_grade_system_demo();
}

// ============================================================================
// 学生成绩管理系统
// ============================================================================

/// 学生结构体：名字 + 各科成绩
#[derive(Debug)]
struct Student {
    name: String,
    scores: Vec<f64>,
}

impl Student {
    fn new(name: &str) -> Self {
        Student { name: String::from(name), scores: Vec::new() }
    }

    fn add_score(&mut self, score: f64) {
        self.scores.push(score);
    }

    /// 平均分（用 for 循环累加）
    fn average(&self) -> f64 {
        if self.scores.is_empty() { return 0.0; }
        let mut total = 0.0;
        for &s in &self.scores { total += s; }
        total / self.scores.len() as f64
    }

    /// 最高分
    fn highest(&self) -> Option<f64> {
        if self.scores.is_empty() { return None; }
        let mut max = self.scores[0];
        for &s in &self.scores { if s > max { max = s; } }
        Some(max)
    }

    /// 最低分
    fn lowest(&self) -> Option<f64> {
        if self.scores.is_empty() { return None; }
        let mut min = self.scores[0];
        for &s in &self.scores { if s < min { min = s; } }
        Some(min)
    }

    /// 及格率（>=60）
    fn pass_rate(&self) -> f64 {
        if self.scores.is_empty() { return 0.0; }
        let mut passed = 0;
        for &s in &self.scores { if s >= 60.0 { passed += 1; } }
        passed as f64 / self.scores.len() as f64 * 100.0
    }

    /// 是否有不及格科目
    fn has_failing(&self) -> bool {
        for &s in &self.scores { if s < 60.0 { return true; } }
        false
    }

    /// 不及格科目数
    fn failing_count(&self) -> usize {
        let mut count = 0;
        for &s in &self.scores { if s < 60.0 { count += 1; } }
        count
    }
}

fn student_grade_system_demo() {
    let mut students: HashMap<String, Student> = HashMap::new();

    // --- 录入学生 ---
    println!("--- 录入学生和成绩 ---");
    let mut alice = Student::new("Alice");
    alice.add_score(85.0); alice.add_score(92.0);
    alice.add_score(78.0); alice.add_score(96.0);
    students.insert(String::from("Alice"), alice);

    let mut bob = Student::new("Bob");
    bob.add_score(55.0); bob.add_score(68.0);
    bob.add_score(42.0); bob.add_score(73.0);
    students.insert(String::from("Bob"), bob);

    let mut charlie = Student::new("Charlie");
    charlie.add_score(90.0); charlie.add_score(88.0);
    charlie.add_score(95.0); charlie.add_score(91.0);
    students.insert(String::from("Charlie"), charlie);

    let mut diana = Student::new("Diana");
    diana.add_score(62.0); diana.add_score(58.0);
    diana.add_score(71.0); diana.add_score(45.0);
    students.insert(String::from("Diana"), diana);

    let mut eve = Student::new("Eve");
    eve.add_score(78.0); eve.add_score(82.0);
    eve.add_score(67.0); eve.add_score(90.0);
    students.insert(String::from("Eve"), eve);

    println!("已录入 {} 名学生", students.len());

    // --- 查询学生 ---
    println!("\n--- 查询 Alice ---");
    if let Some(alice) = students.get("Alice") {
        println!("  成绩：{:?}", alice.scores);
        println!("  平均分：{:.2}，最高：{:.1}，最低：{:.1}，及格率：{:.1}%",
            alice.average(), alice.highest().unwrap_or(0.0),
            alice.lowest().unwrap_or(0.0), alice.pass_rate());
    }

    // --- 全体报告 ---
    println!("\n--- 全体成绩报告 ---");
    println!("{:<10} {:>8} {:>8} {:>8} {:>10}", "姓名", "平均分", "最高分", "最低分", "及格率");
    println!("{}", "-".repeat(50));
    for (_, s) in &students {
        println!("{:<10} {:>8.2} {:>8.1} {:>8.1} {:>9.1}%",
            s.name, s.average(), s.highest().unwrap_or(0.0),
            s.lowest().unwrap_or(0.0), s.pass_rate());
    }

    // --- 按平均分排名（冒泡排序）---
    println!("\n--- 按平均分排名 ---");
    let mut ranked: Vec<(&String, &Student)> = Vec::new();
    for (name, student) in &students { ranked.push((name, student)); }

    // 冒泡排序：降序
    let len = ranked.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if ranked[j].1.average() < ranked[j + 1].1.average() {
                ranked.swap(j, j + 1);
            }
        }
    }
    let mut rank = 0;
    for (name, s) in &ranked {
        rank += 1;
        println!("  第{}名：{}（平均分 {:.2}）", rank, name, s.average());
    }

    // --- 不及格学生 ---
    println!("\n--- 有不及格科目的学生 ---");
    let mut found = false;
    for (name, s) in &students {
        if s.has_failing() {
            found = true;
            println!("  {} - 最低：{:.1}，不及格 {} 科",
                name, s.lowest().unwrap_or(0.0), s.failing_count());
        }
    }
    if !found { println!("  全部及格！"); }

    // --- 整体统计 ---
    println!("\n--- 整体统计 ---");
    let mut total_avg = 0.0;
    let mut count = 0;
    for (_, s) in &students { total_avg += s.average(); count += 1; }
    println!("  全校平均分：{:.2}", total_avg / count as f64);

    let mut best_name = ""; let mut best_avg = 0.0_f64;
    let mut worst_name = ""; let mut worst_avg = f64::MAX;
    for (_, s) in &students {
        let avg = s.average();
        if avg > best_avg { best_avg = avg; best_name = &s.name; }
        if avg < worst_avg { worst_avg = avg; worst_name = &s.name; }
    }
    println!("  最高平均分：{}（{:.2}）", best_name, best_avg);
    println!("  最低平均分：{}（{:.2}）", worst_name, worst_avg);

    // --- entry API 动态添加 ---
    println!("\n--- 动态添加 ---");
    students.entry(String::from("Frank"))
        .or_insert_with(|| Student::new("Frank"))
        .add_score(88.0);
    students.entry(String::from("Alice"))
        .or_insert_with(|| Student::new("Alice"))
        .add_score(100.0);
    println!("  Frank：{:?}", students.get("Frank").unwrap().scores);
    println!("  Alice：{:?}", students.get("Alice").unwrap().scores);

    // --- 成绩分布报告 ---
    println!("\n--- 成绩分布 ---");
    let mut dist: HashMap<&str, Vec<f64>> = HashMap::new();
    let mut grand_total = 0;
    for s in students.values() {
        for &score in &s.scores {
            grand_total += 1;
            let grade = if score >= 90.0 { "A(优秀)" }
                else if score >= 80.0 { "B(良好)" }
                else if score >= 70.0 { "C(中等)" }
                else if score >= 60.0 { "D(及格)" }
                else { "F(不及格)" };
            dist.entry(grade).or_insert_with(Vec::new).push(score);
        }
    }
    for grade in &["A(优秀)", "B(良好)", "C(中等)", "D(及格)", "F(不及格)"] {
        if let Some(scores) = dist.get(grade) {
            println!("  {}：{} 门（{:.1}%）", grade, scores.len(),
                scores.len() as f64 / grand_total as f64 * 100.0);
        }
    }

    // --- 删除学生 ---
    println!("\n--- 删除 Bob ---");
    match students.remove("Bob") {
        Some(s) => println!("  已删除 {}（成绩：{:?}）", s.name, s.scores),
        None => println!("  没有 Bob"),
    }
    println!("  剩余 {} 名学生", students.len());

    println!("\n学生成绩管理系统演示完毕！");
}

// ============================================================================
// 练习题
// ============================================================================

/*
 * ============================================================
 * 练习 1（基础，5-10 分钟）：词频统计
 * ============================================================
 *
 * 用 HashMap 统计文本中每个单词出现的次数。
 * 提示：split_whitespace() 拆词 + entry().or_insert(0) 计数
 */

// fn exercise_1_word_frequency() {
//     use std::collections::HashMap;
//     let text = "hello world hello rust hello world programming rust is fun rust";
//     let mut word_count: HashMap<&str, i32> = HashMap::new();
//     for word in text.split_whitespace() {
//         let count = word_count.entry(word).or_insert(0);
//         *count += 1;
//     }
//     println!("词频统计：");
//     for (word, count) in &word_count {
//         println!("  \"{}\" 出现 {} 次", word, count);
//     }
// }

// ===== 测试版练习 =====
// 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
// 取消下面的注释，运行 cargo test --example lesson09

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_word_frequency() {
        let text = "hello world hello rust hello world programming rust is fun rust";
        let mut word_count: HashMap<&str, i32> = HashMap::new();
        for word in text.split_whitespace() {
            let count = word_count.entry(word).or_insert(0);
            *count += 1;
        }
        assert_eq!(word_count["hello"], 3);
        assert_eq!(word_count["world"], 2);
        assert_eq!(word_count["rust"], 3);
        assert_eq!(word_count["programming"], 1);
        assert_eq!(word_count["is"], 1);
        assert_eq!(word_count["fun"], 1);
        assert_eq!(word_count.len(), 6); // 共 6 个不同单词
    }
}
*/

/*
 * ============================================================
 * 练习 2（应用，15-20 分钟）：集合运算
 * ============================================================
 *
 * 给定两个 Vec<i32>，用 HashSet 实现并集、交集、差集、对称差集。
 * 提示：遍历 Vec 逐个 insert 到 HashSet，用 union/intersection/difference 方法
 */

// fn exercise_2_set_operations() {
//     use std::collections::HashSet;
//     let vec_a = vec![1, 2, 3, 4, 5, 6];
//     let vec_b = vec![4, 5, 6, 7, 8, 9];
//     let mut set_a: HashSet<&i32> = HashSet::new();
//     for n in &vec_a { set_a.insert(n); }
//     let mut set_b: HashSet<&i32> = HashSet::new();
//     for n in &vec_b { set_b.insert(n); }
//     println!("A={:?}, B={:?}", set_a, set_b);
//     println!("并集：{:?}", set_a.union(&set_b).cloned().collect::<HashSet<_>>());
//     println!("交集：{:?}", set_a.intersection(&set_b).cloned().collect::<HashSet<_>>());
//     println!("差集(A-B)：{:?}", set_a.difference(&set_b).cloned().collect::<HashSet<_>>());
//     println!("对称差集：{:?}", set_a.symmetric_difference(&set_b).cloned().collect::<HashSet<_>>());
// }

/*
 * ============================================================
 * 练习 3（进阶，选做）：成绩排名
 * ============================================================
 *
 * 给定学生成绩列表，用 for 循环：
 *   1. 过滤及格（>=60）学生
 *   2. 成绩提高 10%（上限100）
 *   3. 冒泡排序按成绩降序
 *   4. 输出排名
 */

// fn exercise_3_grade_ranking() {
//     let students = vec![
//         ("Alice", 85.0), ("Bob", 52.0), ("Charlie", 73.0),
//         ("Diana", 46.0), ("Eve", 91.0), ("Frank", 67.0),
//     ];
//     let mut passed: Vec<(&str, f64)> = Vec::new();
//     for &(name, score) in &students {
//         if score >= 60.0 {
//             passed.push((name, (score * 1.1).min(100.0)));
//         }
//     }
//     let len = passed.len();
//     for i in 0..len {
//         for j in 0..len - 1 - i {
//             if passed[j].1 < passed[j + 1].1 { passed.swap(j, j + 1); }
//         }
//     }
//     println!("提分后排名：");
//     let mut rank = 0;
//     for (name, score) in &passed {
//         rank += 1;
//         println!("  第{}名：{} - {:.1}分", rank, name, score);
//     }
// }

// 取消注释运行练习：
// fn main() {
//     exercise_1_word_frequency();
//     exercise_2_set_operations();
//     exercise_3_grade_ranking();
// }

// ============================================================================
// 结尾
// ============================================================================

/*
 * ============================================================
 * 核心收获：
 * ============================================================
 *
 * 1. 集合是"装一堆东西"的容器：
 *    - Vec<T>：可变长度列表，最常用，push/pop/insert/remove
 *    - HashMap<K,V>：键值对字典，查找极快，entry API 智能插入
 *    - HashSet<T>：不重复集合，支持并集/交集/差集
 *    - BTreeMap/BTreeSet：有序集合，按键排序遍历
 *
 * 2. String 的要点：
 *    - String 拥有数据，&str 借用数据
 *    - UTF-8 编码：len() 返回字节数，chars().count() 返回字符数
 *    - 切片必须在字符边界处，否则 panic
 *
 * 3. for 循环是遍历集合的基础工具：
 *    - 遍历 Vec：for item in &vec { ... }
 *    - 遍历 HashMap：for (k, v) in &map { ... }
 *    - 带编号：手动维护一个计数器变量
 *
 * ============================================================
 * 常见陷阱：
 * ============================================================
 *
 * 1. Vec 越界访问会 panic！用 get(index) 返回 Option 更安全
 * 2. HashMap 遍历顺序不确定！需要有序用 BTreeMap
 * 3. String 的 len() 是字节数不是字符数！中文字符=3字节
 *
 * ============================================================
 * 下节课预告：迭代器与闭包
 * ============================================================
 *
 * 本节课我们用 for 循环遍历集合、手动累加、冒泡排序……
 * 这些操作虽然直观，但代码有点冗长。
 *
 * 下节课学习迭代器与闭包——函数式编程的双剑合璧！
 * 迭代器像"传送带"，闭包像"匿名小函数"，两者结合能把
 * 十几行的 for 循环浓缩成一行优雅的链式调用。
 *
 * 敬请期待！
 *
 */
