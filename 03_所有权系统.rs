// -*- coding: utf-8 -*-
// ============================================================================
// 第 03 课：所有权系统 —— Rust 的灵魂，内存安全的秘密武器
// ============================================================================
//
// 上节课我们学了控制流和函数。
// 这节课我们要学习 Rust 最核心、最独特的特性——"所有权"。
//
// 所有权系统是 Rust 区别于所有其他语言的根本原因。
// 它让 Rust 在编译时就解决内存问题，零运行时开销，无需垃圾回收。
//
// 想象你买了一套房子：每套房子有且只有一个主人（房产证上写名字）；
// 过户给别人后，你就不再是主人了；主人消失，房子被政府收回。
// Rust 的所有权规则跟房产证一模一样——每个值只有一个所有者，
// 赋值就是过户，离开作用域就自动释放。
//
// ============================================================================

fn main() {
    // =============================================================
    // 第一节：所有权三规则——房产证的故事
    // =============================================================
    //
    // 【生活类比：房产证】
    // 想象你买了一套房子：
    //   规则一：每套房子必须有一个主人（房产证上写着名字）
    //   规则二：一套房子同一时刻只能有一个主人（不能有两个房产证）
    //   规则三：如果主人消失了，房子会被政府收回（自动清理）
    //
    // Rust 的所有权规则跟房产证一模一样：
    //   规则一：Rust 中每个值都有一个"所有者"（变量名）
    //   规则二：同一时刻，一个值只能有一个所有者
    //   规则三：当所有者离开"作用域"时，值被自动丢弃（drop）
    //
    // 【什么是"作用域"？】
    // 作用域就是一个变量"活着"的范围。
    // 用大括号 {} 包起来的区域就是一个作用域。
    // 变量在声明时出生，在 } 结束时死亡。
    // 就像一个人在家（作用域内）可以活动，出门了（作用域外）就找不到了。

    println!("=== 第一节：所有权三规则 ===\n");

    // 【规则一演示：每个值有一个所有者】
    let house = String::from("湖景别墅"); // house 就是这个字符串的所有者
    println!("房子的主人是 house，房子是：{}", house);

    // 【规则三演示：离开作用域，自动 drop】
    {
        let temp_house = String::from("临时住所"); // temp_house 诞生
        println!("作用域内，临时住所是：{}", temp_house);
    } // <-- 这里 temp_house 离开作用域，String 被自动释放！
      // 就像主人消失了，房子被政府收回
      // println!("{}", temp_house); // 如果取消这行注释，编译器会报错：
      // error[E0425]: cannot find value `temp_house` in this scope

    // 【为什么需要所有权系统？】
    // 在 C/C++ 中，程序员必须手动 malloc/free（申请/释放内存）
    // 常见问题：
    //   1. 内存泄漏——忘记释放，内存越来越少
    //   2. 悬垂指针——释放了还在用，程序崩溃
    //   3. 重复释放——释放两次，程序崩溃
    //
    // Rust 的所有权系统在编译时就杜绝了这三种问题！
    // 只要代码能编译通过，内存就是安全的。
    // 这就是 Rust 的承诺。

    // =============================================================
    // 第二节：Move 语义——房产过户
    // =============================================================
    //
    // 【生活类比：房产过户】
    // 你把房子过户给朋友：
    //   - 过户完成后，房产证上写的是朋友的名字
    //   - 你不再是房子的主人了
    //   - 你不能再进那套房子了（你已经没有权利了）
    //
    // Rust 中，把一个堆上的值赋给另一个变量，就相当于"过户"：
    //   - 原变量失效
    //   - 新变量成为唯一的所有者

    println!("\n=== 第二节：Move 语义（过户） ===\n");

    let s1 = String::from("hello"); // s1 是 "hello" 的所有者
    let s2 = s1; // 所有权从 s1 "过户"给 s2，s1 从此失效！

    // println!("{}", s1); // 错误！编译器不让用了！
    // 报错信息：error[E0382]: borrow of moved value: `s1`
    // 翻译：s1 的值已经被移走了，不能再使用
    println!("s2 拿到了所有权：{}", s2);

    // 【为什么赋值会"过户"？——堆内存的秘密】
    // String 类型在内存中是这样的：
    //
    //   栈（Stack）              堆（Heap）
    //   ┌──────────────┐        ┌─────────────┐
    //   │ s1: ptr  ────────────>│ h e l l o   │
    //   │     len: 5  │        └─────────────┘
    //   │     cap: 5  │
    //   └──────────────┘
    //
    // 如果简单地让 s2 也指向同一块堆内存：
    //   栈（Stack）              堆（Heap）
    //   ┌──────────────┐        ┌─────────────┐
    //   │ s1: ptr  ───────┐    │ h e l l o   │
    //   ├──────────────┤  ├───>│             │
    //   │ s2: ptr  ─────┘    └─────────────┘
    //   └──────────────┘
    //
    // 当 s1 和 s2 都离开作用域时，同一块堆内存会被释放两次！
    // 这叫"双重释放"（double free），会导致程序崩溃。
    //
    // 所以 Rust 的做法是：赋值时让 s1 失效，只有 s2 拥有所有权。
    // 这样堆内存只被释放一次，问题解决！

    // =============================================================
    // 第三节：Copy 语义——复印文件
    // =============================================================
    //
    // 【生活类比：复印文件】
    // 你有一份文件，去复印了一份给朋友：
    //   - 朋友拿到的是复印件
    //   - 你的原件还在
    //   - 两份文件互不影响
    //
    // Rust 中，简单的栈上数据（整数、浮点数、布尔值等）赋值时，
    // 不是"过户"，而是"复印"——两边都有效。

    println!("\n=== 第三节：Copy 语义（复印） ===\n");

    let x = 5;   // x 是 i32 类型，存在栈上
    let y = x;   // x 被"复印"给 y，x 依然有效！

    println!("x = {}（原件）", x);   // 正确！x 还在
    println!("y = {}（复印件）", y); // 正确！y 是副本

    // 【哪些类型会"复印"（Copy）？】
    // 这些类型都存在栈上，复制成本很低，所以 Rust 允许自动复印：
    //   1. 所有整数类型：i32, u64, isize 等
    //   2. 浮点数类型：f32, f64
    //   3. 布尔类型：bool（true / false）
    //   4. 字符类型：char（单个字符）
    //   5. 元组——如果每个元素都是 Copy 的话：(i32, f64) 可以 Copy
    //   6. 数组——如果元素是 Copy 的话：[i32; 5] 可以 Copy
    //
    // String 不是 Copy 的！因为它在堆上有数据，复印太昂贵。
    // 如果你真的想"复印"一个 String，要显式调用 .clone()

    // 【显式克隆 String】
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝：堆上的数据也被复制了一份

    println!("s1 = {}（原件）", s1); // 正确！clone 后 s1 依然有效
    println!("s2 = {}（克隆体）", s2);

    // 【小结：Move vs Copy】
    //   堆上数据（String, Vec 等）→ 赋值 = Move（过户）
    //   栈上数据（i32, bool 等）  → 赋值 = Copy（复印）
    //   想要堆上数据的副本？  → 用 .clone() 显式克隆

    // =============================================================
    // 第四节：函数参数的所有权转移
    // =============================================================
    //
    // 【生活类比：送礼物】
    // 你把礼物送给朋友：
    //   - 朋友收到礼物，拥有了它
    //   - 你手上空了
    //   - 你不能再把这个礼物送给第二个人
    //
    // 把变量传给函数，就像"送礼物"：
    //   - 如果参数类型是"拥有所有权"的（如 String），所有权会转移进函数
    //   - 函数结束后，值被释放
    //   - 调用者不能再使用这个变量

    println!("\n=== 第四节：函数参数的所有权转移 ===\n");

    // --- Move 演示 ---
    let my_gift = String::from("一本好书");
    give_gift(my_gift); // my_gift 的所有权转移进函数
    // println!("{}", my_gift); // 错误！my_gift 已经被"送出去"了
    // 报错：error[E0382]: borrow of moved value: `my_gift`

    // --- Copy 演示 ---
    let my_number = 42;
    print_number(my_number); // my_number 被复印了一份传进去
    println!("函数调用后，my_number 还在：{}", my_number); // 正确！

    // --- 函数也可以返回所有权 ---
    let returned = create_and_return(); // 函数创建值并返回所有权
    println!("从函数拿回来的：{}", returned);

    // --- 如果你既想传进去又想继续用？---
    // 方法一：传进去再传出来（很麻烦）
    let s1 = String::from("hello");
    let s1 = take_and_return(s1); // 传进去，再传出来
    println!("拿回来的：{}", s1);

    // 方法二：使用引用（借用）——下一节的主角！
    let s2 = String::from("world");
    let len = calculate_length(&s2); // 传引用，不转移所有权
    println!("'{}' 的长度是 {}，s2 还能用！", s2, len);

    // =============================================================
    // 第五节：借用（引用）——借钥匙参观房子
    // =============================================================
    //
    // 【生活类比：借钥匙】
    // 你有一套房子，朋友想参观：
    //   - 你把钥匙借给朋友（引用 &T）
    //   - 朋友可以参观房子（读取数据）
    //   - 但朋友不能拆墙装修（不能修改）
    //   - 房子还是你的（所有权不变）
    //   - 参观完后，钥匙还给你（引用结束）
    //
    // 这就是"借用"（Borrowing）：
    //   - 用 & 符号创建引用
    //   - 引用可以读取数据，但不拥有数据
    //   - 原变量依然有效

    println!("\n=== 第五节：借用（引用） ===\n");

    let house = String::from("湖景别墅");
    let description = describe(&house); // &house 就是"借钥匙"
    println!("{}", description);
    println!("房子还是 {} 的：{}", "house", house); // house 还在！

    // =============================================================
    // 第六节：不可变引用（&T）和可变引用（&mut T）
    // =============================================================
    //
    // 【生活类比：参观钥匙 vs 装修钥匙】
    //
    // 不可变引用 &T（参观钥匙）：
    //   - 可以同时借给很多人（多个 &T 可以共存）
    //   - 大家只能看，不能动（只读）
    //   - 就像开放日参观，很多人同时参观没问题
    //
    // 可变引用 &mut T（装修钥匙）：
    //   - 同一时间只能借给一个人（只能有一个 &mut T）
    //   - 这个人可以改装房子（读写）
    //   - 如果有人在装修，其他人连参观都不行（不能有其他引用）
    //   - 就像装修时要封锁现场，防止安全事故

    println!("\n=== 第六节：不可变引用与可变引用 ===\n");

    // --- 不可变引用：可以同时有多个 ---
    let book = String::from("Rust编程之道");
    let reader1 = &book; // 第一个人借去看
    let reader2 = &book; // 第二个人也借去看
    let reader3 = &book; // 第三个人也行！
    println!("读者1看到：{}", reader1);
    println!("读者2看到：{}", reader2);
    println!("读者3看到：{}", reader3);
    // 三个人同时读，互不影响，没问题！

    // --- 可变引用：同一时间只能有一个 ---
    let mut room = String::from("毛坯房");
    println!("装修前：{}", room);

    renovate(&mut room); // 把"装修钥匙"借给装修工
    println!("装修后：{}", room);

    // --- 借用规则演示 ---
    borrowing_rules_demo();

    // --- 编译器报错演示（注释展示） ---
    compiler_error_demos();

    // =============================================================
    // 第七节：切片（Slice）——切蛋糕
    // =============================================================
    //
    // 【生活类比：切蛋糕】
    // 你有一个完整的蛋糕（String），但你只想分享一块：
    //   - 你切出一块（切片 &str）
    //   - 这块蛋糕还是蛋糕的一部分（不是新蛋糕）
    //   - 切片是对原始数据某一部分的"借用"
    //
    // 切片是一种特殊的引用，它引用数据中连续的一部分。

    println!("\n=== 第七节：切片（Slice） ===\n");

    // --- 字符串切片 &str ---
    let sentence = String::from("hello world rust");
    // 切出第一个单词
    let word1 = &sentence[0..5];   // "hello"（索引 0 到 4，不含 5）
    let word2 = &sentence[6..11];  // "world"（索引 6 到 10）
    let word3 = &sentence[12..16]; // "rust"（索引 12 到 15）
    println!("三个单词：{} {} {}", word1, word2, word3);

    // 简写形式
    let hello = &sentence[..5];     // 从开头到索引 5（不含）
    let world = &sentence[6..11];   // 从索引 6 到 11（不含）
    let all = &sentence[..];        // 整个字符串的切片
    println!("简写：{} {} {}", hello, world, all);

    // 字符串字面量 "hello" 本身就是 &str 类型！
    // 它是整个程序运行期间都存在的字符串的切片
    let literal: &str = "我是一个字符串字面量，其实我是切片";
    println!("{}", literal);

    // --- 数组切片 &[T] ---
    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4]; // [20, 30, 40]
    println!("数组切片：{:?}", middle);

    // 切片也受借用规则约束！
    let mut data = vec![1, 2, 3, 4, 5];
    let slice = &data[0..3]; // 不可变切片（借用）
    println!("切片：{:?}", slice);
    // data.push(6); // 错误！data 已经被不可变借用了，不能修改
    // 等切片不再使用后，才能修改 data
    println!("切片用完了");
    data.push(6); // 现在可以了！因为 slice 后面不再使用
    println!("修改后的 data：{:?}", data);

    // =============================================================
    // 【实战项目】字符串处理工具
    // =============================================================
    //
    // 我们来做一个小型的字符串处理工具箱，
    // 把今天学到的所有知识用起来！
    // 每个功能都封装成函数，在 main 中调用。

    println!("\n============================================");
    println!("实战项目：G-one 的字符串处理工具箱");
    println!("============================================\n");

    let test_string = String::from("hello world rust programming language");

    // 功能一：统计字数
    let word_count = count_words(&test_string);
    println!("原始字符串：{}", test_string);
    println!("单词数量：{}", word_count);

    // 功能二：反转字符串
    let reversed = reverse_string(&test_string);
    println!("反转结果：{}", reversed);

    // 功能三：首字母大写
    let capitalized = capitalize_first(&test_string);
    println!("首字母大写：{}", capitalized);

    // 功能四：截取前 N 个字符（切片）
    let prefix = first_n_chars(&test_string, 11);
    println!("前 11 个字符：{}", prefix);

    // 功能五：提取第一个单词（切片）
    let first = first_word_slice(&test_string);
    println!("第一个单词：{}", first);

    // =============================================================
    // 【练习题】
    // =============================================================

    println!("\n============================================");
    println!("练习参考答案");
    println!("============================================\n");

    // ===== 测试版练习 =====
    // 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
    // 取消下面的注释，运行 cargo test --example 03_所有权系统
    /*
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_exercise1_clone_fixes_move() {
            // 测试练习 1 的核心功能：clone() 修复 move 错误
            let original = String::from("重要数据");
            let backup = original.clone();
            // clone 后 original 依然有效
            assert_eq!(original, "重要数据");
            assert_eq!(backup, "重要数据");
            // 两份数据互不影响
            assert_eq!(original, backup);
        }
    }
    */

    // ----- 练习 1：基础巩固（5-10 分钟） -----
    // 题目：写一段代码让编译器报 move 错误，然后用 clone() 修复
    println!("--- 练习 1：Move 错误与 clone 修复 ---\n");

    // 错误版本（取消注释会报错）：
    // let original = String::from("重要数据");
    // let backup = original;
    // println!("原始数据：{}", original); // 报错！original 已经被 move 了

    // 正确版本：用 clone() 创建副本
    let original = String::from("重要数据");
    let backup = original.clone(); // 深拷贝，original 不会失效
    println!("原始数据：{}", original);
    println!("备份数据：{}", backup);
    println!("两份数据都有效！\n");

    // ----- 练习 2：应用练习（15-20 分钟） -----
    // 题目：实现 first_word 函数（返回字符串中第一个单词的切片）
    println!("--- 练习 2：first_word 函数 ---\n");

    let sentence = String::from("hello world");
    let word = first_word_slice(&sentence);
    println!("句子：{}", sentence);
    println!("第一个单词：{}", word);

    let single = String::from("onlyword");
    let word2 = first_word_slice(&single);
    println!("句子：{}", single);
    println!("第一个单词：{}", word2);

    let empty = String::from("   leading spaces");
    let word3 = first_word_slice(&empty);
    println!("句子：'{}'", empty);
    println!("第一个单词：'{}'", word3);
    println!();

    // ----- 练习 3：进阶挑战（选做） -----
    // 题目：演示可变引用和不可变引用不能同时存在，解释原因
    println!("--- 练习 3：借用冲突演示 ---\n");
    borrowing_conflict_demo();

    // =============================================================
    // 【补充知识】编译器报错信息解读
    // =============================================================
    //
    // Rust 编译器的报错信息非常友好，它会告诉你：
    //   1. 错误在哪里（文件名、行号、列号）
    //   2. 错误是什么（错误码 + 中文描述）
    //   3. 为什么会出错（解释原因）
    //   4. 怎么修复（建议代码）
    //
    // 常见的借用相关错误：
    //
    // error[E0382]: borrow of moved value: `s1`
    //   → 你试图使用一个已经被 move 走的变量
    //   → 修复：用 clone() 或改用引用
    //
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    //   → 你同时创建了两个可变引用
    //   → 修复：确保同一时间只有一个可变引用
    //
    // error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
    //   → 你同时有可变引用和不可变引用
    //   → 修复：让不可变引用在可变引用之前结束
    //
    // 遇到编译错误不要慌，认真读错误信息，它就是你最好的老师！

    println!("============================================");
    println!("编译器报错信息已整理在代码注释中");
    println!("取消注释相关代码可以亲自体验编译错误");
    println!("============================================");

    // =============================================================
    // 【教授的话】
    // =============================================================

    println!("\n==========================================");
    println!("第03课 所有权系统 - 完");
    println!("==========================================");
}

/*
 * 核心收获：
 * - 所有权三规则：每个值有且仅有一个所有者，所有者离开作用域时值被自动 drop，这是 Rust 内存安全的基石
 * - Move 与 Copy：堆上数据赋值是"过户"（原变量失效），栈上数据赋值是"复印"（两边有效），用 .clone() 可以显式深拷贝
 * - 借用规则：多个不可变引用 &T 可以共存，但同一时刻只能有一个可变引用 &mut T，两者不能同时存在——编译器替你守住数据安全的底线
 *
 * 常见陷阱：
 * - 把 String 赋值给另一个变量后，还试图使用原来的变量——编译器会报 borrow of moved value，解决方法是用 .clone() 或改用引用
 * - 在持有不可变引用 &T 的同时试图创建可变引用 &mut T——编译器会报 cannot borrow as mutable，解决方法是让不可变引用先结束生命周期
 *
 * 下节课预告：
 * - 下节课学结构体，把多个数据组织成一个有意义的"东西"，就像把姓名、年龄、身高打包成一份"个人档案"
 */

// =============================================================
// 辅助函数定义
// =============================================================

/// 【演示 Move】把礼物送给函数，所有权转移
/// 参数类型是 String（拥有所有权），调用时所有权会转移进来
fn give_gift(gift: String) {
    println!("收到礼物：{}", gift);
} // gift 在这里离开作用域，String 被自动释放

/// 【演示 Copy】打印一个数字，不影响原变量
/// 参数类型是 i32（Copy 类型），调用时会被复印一份
fn print_number(num: i32) {
    println!("函数里看到的数字：{}", num);
} // num 是副本，函数结束后副本被丢弃，原值不受影响

/// 创建一个 String 并返回所有权给调用者
fn create_and_return() -> String {
    let s = String::from("函数内部创建的字符串");
    s // 返回 s，所有权转移给调用者
}

/// 接收一个 String，再把它返回去（归还所有权）
fn take_and_return(s: String) -> String {
    println!("函数里暂时持有：{}", s);
    s // 归还所有权
}

/// 【演示借用】计算字符串长度，不转移所有权
/// 参数是 &String，表示"借用"，不拥有所有权
fn calculate_length(s: &String) -> usize {
    s.len()
} // s 是引用，离开作用域时只丢弃引用，底层数据不会被释放

/// 【演示借用】描述一个字符串，只读不改
fn describe(text: &String) -> String {
    format!("这是一段 {} 个字符的文本", text.len())
}

/// 【演示可变引用】装修房子——通过可变引用修改数据
fn renovate(room: &mut String) {
    room.push_str(" -> 精装修完成，拎包入住！");
}

/// 【借用规则演示】
/// 展示不可变引用和可变引用的共存规则
fn borrowing_rules_demo() {
    println!("--- 借用规则演示 ---\n");

    // 规则一：可以同时有多个不可变引用
    let data = String::from("共享数据");
    let r1 = &data; // 不可变引用 1
    let r2 = &data; // 不可变引用 2
    let r3 = &data; // 不可变引用 3
    println!("多个不可变引用共存：{} / {} / {}", r1, r2, r3);

    // 规则二：同一时间只能有一个可变引用
    let mut mutable_data = String::from("可变数据");
    {
        let r_mut = &mut mutable_data; // 可变引用
        r_mut.push_str("（已修改）");
        println!("可变引用修改后：{}", r_mut);
    } // r_mut 在这里结束，之后才能再借用 mutable_data

    // 规则三：不可变引用和可变引用不能同时存在
    // 下面的代码如果取消注释会报错：
    // let r_immut = &data;
    // let r_mut = &mut data;  // 错误！data 已经被不可变借用了
    // println!("{} {}", r_immut, r_mut);

    println!();
}

/// 【编译器报错演示】
/// 展示常见的借用冲突场景（代码在注释中，取消注释可体验编译错误）
fn compiler_error_demos() {
    println!("--- 编译器报错演示（见代码注释） ---\n");

    // ---- 错误 1：使用已 move 的变量 ----
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}", s1);
    //
    // 编译器会说：
    // error[E0382]: borrow of moved value: `s1`
    //  --> src/main.rs:X:Y
    //   |
    // 3 |     let s1 = String::from("hello");
    //   |         -- move occurs because `s1` has type `String`
    // 4 |     let s2 = s1;
    //   |              -- value moved here
    // 5 |     println!("{}", s1);
    //   |                    ^^ value borrowed here after move
    //
    // 修复方法：let s2 = s1.clone(); 或者改用 &s1

    // ---- 错误 2：同时有两个可变引用 ----
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;  // 报错！
    // println!("{} {}", r1, r2);
    //
    // 编译器会说：
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    //  --> src/main.rs:X:Y
    //   |
    // 4 |     let r1 = &mut s;
    //   |              ------ first mutable borrow occurs here
    // 5 |     let r2 = &mut s;
    //   |              ^^^^^^ second mutable borrow occurs here
    // 6 |     println!("{} {}", r1, r2);
    //   |                        -- first borrow later used here
    //
    // 修复方法：确保同一时间只有一个可变引用

    // ---- 错误 3：不可变引用和可变引用同时存在 ----
    // let mut s = String::from("hello");
    // let r1 = &s;        // 不可变借用
    // let r2 = &s;        // 不可变借用
    // let r3 = &mut s;    // 可变借用——报错！
    // println!("{} {} {}", r1, r2, r3);
    //
    // 编译器会说：
    // error[E0502]: cannot borrow `s` as mutable because it is
    //               also borrowed as immutable
    //  --> src/main.rs:X:Y
    //   |
    // 4 |     let r1 = &s;
    //   |              -- immutable borrow occurs here
    // ...
    // 6 |     let r3 = &mut s;
    //   |              ^^^^^^ mutable borrow occurs here
    // 7 |     println!("{} {} {}", r1, r2, r3);
    //   |                        -- immutable borrow later used here
    //
    // 修复方法：NLL（Non-Lexical Lifetimes）——如果 r1、r2 在 r3 之前不再使用，编译器会自动结束借用
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{} {}", r1, r2); // r1、r2 最后一次使用，借用到这里结束
    // let r3 = &mut s;           // 现在可以可变借用了！
    // println!("{}", r3);

    println!("以上三种错误是所有权系统最常见的编译错误。");
    println!("建议你取消注释亲自试一试，看看编译器怎么说！\n");
}

/// 【借用冲突演示——进阶挑战】
/// 演示为什么 Rust 不允许同时存在可变引用和不可变引用
fn borrowing_conflict_demo() {
    println!("场景：一个图书馆管理系统");
    println!("  - 读者想查书（不可变引用，只读）");
    println!("  - 管理员想修改库存（可变引用，读写）");
    println!("  - 如果同时进行，可能会出现数据不一致！\n");

    let mut inventory = String::from("库存：《Rust编程》5本");

    // 模拟读者查询
    let reader_view = &inventory;
    println!("读者看到：{}", reader_view);
    // 如果此时管理员修改库存：
    // let admin_edit = &mut inventory; // 报错！已经有不可变引用了
    // admin_edit.push_str("，已借出2本");
    // 因为 reader_view 还在后面被使用，编译器不允许同时可变借用

    // 正确做法：让读者的查询先完成
    println!("读者查询完成。\n");

    // 读者查询结束后，管理员才能修改
    let admin_edit = &mut inventory;
    admin_edit.push_str("，已借出2本");
    println!("管理员修改后：{}", admin_edit);

    println!("\n为什么 Rust 这样设计？");
    println!("  如果允许同时读和写：");
    println!("  - 读者读到的数据可能是"半修改"状态");
    println!("  - 多线程环境下会导致数据竞争（data race）");
    println!("  - 这是 C/C++ 中最难调试的 bug 之一");
    println!("  Rust 在编译时就杜绝了这种可能性！\n");
}

// =============================================================
// 【实战项目】字符串处理工具箱——所有函数在这里定义
// =============================================================

/// 功能一：统计字数
/// 使用 split_whitespace() 按空白字符分割，统计单词数量
/// split_whitespace() 返回一个迭代器，每次产出一个单词
fn count_words(text: &str) -> usize {
    // text.split_whitespace() 把字符串按空格/换行/制表符分割
    // .count() 统计迭代器中有多少个元素
    text.split_whitespace().count()
}

/// 功能二：反转字符串
/// 将字符串中的字符顺序颠倒
/// 注意：这按 Unicode 字符反转，对中文也能正常工作
fn reverse_string(text: &str) -> String {
    // text.chars() 把字符串拆成字符的迭代器
    // .rev() 反转迭代器
    // .collect() 把迭代器收集回一个新的 String
    text.chars().rev().collect()
}

/// 功能三：首字母大写
/// 把字符串第一个字符变成大写，其余保持不变
fn capitalize_first(text: &str) -> String {
    // 取出第一个字符
    let mut chars = text.chars();
    match chars.next() {
        // 如果有第一个字符
        Some(first) => {
            // to_uppercase() 返回大写形式（可能不止一个字符，比如德语 ß）
            let upper: String = first.to_uppercase().collect();
            // 拼接：大写首字母 + 剩余字符
            upper + chars.as_str()
        }
        // 如果是空字符串，原样返回
        None => String::new(),
    }
}

/// 功能四：截取前 N 个字符（切片应用）
/// 注意：这里按字节截取，对纯 ASCII（英文）没问题
/// 对于中文等多字节字符，需要用 .chars().take(n) 方式
/// 这里演示字节切片，展示切片的基本用法
fn first_n_chars(text: &str, n: usize) -> &str {
    // 检查 n 是否超过字节长度
    if n >= text.len() {
        return text;
    }
    // 使用字节索引切片
    // 注意：如果 n 落在多字节字符中间，这里会 panic
    // 更安全的方式见下面的注释
    &text[..n]
    // 更安全的实现（按 Unicode 字符数截取）：
    // let end = text.char_indices().nth(n).map(|(i, _)| i).unwrap_or(text.len());
    // &text[..end]
}

/// 功能五：提取第一个单词（切片应用）
/// 返回字符串中第一个单词的切片
/// 这个函数展示了切片作为返回值的用法
fn first_word_slice(text: &str) -> &str {
    // 把字符串转成字节数组
    let bytes = text.as_bytes();
    // 遍历每个字节，找到第一个空格
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // 找到空格，返回从开头到空格前的切片
            return &text[..i];
        }
    }
    // 没有空格，整个字符串就是一个单词
    text
}
