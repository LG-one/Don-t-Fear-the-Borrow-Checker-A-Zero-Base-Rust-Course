// ============================================================================
//  Rust 异步编程入门 —— async/await 与 tokio
// ============================================================================
//
//  本课需要 tokio 依赖。请确保 Cargo.toml 中有以下配置：
//
//  ```toml
//  [dependencies]
//  tokio = { version = "1", features = ["full"] }
//  ```
//
//  运行方式：cargo run --example lesson_async
//
//  作者：G-one
//  定位：课外进阶资料，面向已有 Rust 基础的同学
//
// ============================================================================
//  目录
//  ----
//  第一部分：概念篇
//    1. 什么是异步编程（同步 vs 异步）
//    2. 为什么需要异步（IO 密集型场景）
//
//  第二部分：语法篇
//    3. async/await 基础语法
//    4. tokio 运行时简介（#[tokio::main]）
//    5. 异步函数和异步块
//
//  第三部分：并发篇
//    6. tokio::spawn 并发任务
//    7. tokio::join! 同时等待多个任务
//    8. tokio::select! 选择第一个完成的任务
//
//  第四部分：通信与IO篇
//    9. 异步 Channel（tokio::sync::mpsc）
//   10. 异步文件IO（tokio::fs）
//
//  第五部分：实战项目
//    异步 HTTP 请求器（模拟版）
//
// ============================================================================

// 注意：这段代码本身不是可直接编译的 main.rs，而是按章节拆分的示例集合。
// 实际使用时请将每个章节的示例放入独立文件或单独的函数中运行。
// 下面我们用一个 main 函数把所有示例串起来。

// ============================================================================
// 第一部分：概念篇
// ============================================================================

// ---------------------------------------------------------------------------
// 1. 什么是异步编程（同步 vs 异步）
// ---------------------------------------------------------------------------
//
// 【同步模型】
//   同步代码是一行一行顺序执行的。当遇到一个耗时操作（比如读文件、发网络请求），
//   程序会"阻塞"在那里，等到操作完成后才继续执行下一行。
//
//   伪代码示意（同步）：
//     let a = 从网络下载文件A();   // 花 3 秒，程序在此等待 3 秒
//     let b = 从网络下载文件B();   // 花 3 秒，程序在此再等 3 秒
//     // 总共花了 6 秒
//
// 【异步模型】
//   异步代码在遇到耗时操作时，不会傻等，而是"让出"执行权，去做别的事情。
//   等耗时操作完成后，再回来继续处理结果。
//
//   伪代码示意（异步）：
//     let a = 从网络下载文件A();   // 发起请求，不等结果，立刻往下走
//     let b = 从网络下载文件B();   // 发起请求，不等结果
//     let a = a.await;             // 现在才真正等待 A 完成
//     let b = b.await;             // 等待 B 完成（可能 A 完成时 B 也快完成了）
//     // 总共花了大约 3 秒（两个下载几乎并行进行）
//
// 【关键区别总结】
//   同步：调用 -> 阻塞等待 -> 得到结果 -> 继续
//   异步：调用 -> 立即返回一个"未来值"（Future） -> 需要结果时再 await -> 继续
//
// 【类比】
//   同步就像你在餐厅点了一道菜，站在厨房门口等厨师做完，才去点下一道菜。
//   异步就像你点完一道菜，拿到取餐号，然后继续去点别的菜，或者刷手机。
//   菜做好了服务员会叫你。

// ---------------------------------------------------------------------------
// 2. 为什么需要异步（IO 密集型场景）
// ---------------------------------------------------------------------------
//
// 计算机程序的瓶颈通常分为两类：
//
//   (1) CPU 密集型：大量计算，CPU 一直满负荷工作
//       例如：视频编码、图像处理、密码破解
//       这类任务用多线程更合适，因为需要多个 CPU 核心同时计算。
//
//   (2) IO 密集型：大量时间花在等待 IO 操作上（网络、磁盘、数据库）
//       例如：Web 服务器同时处理 10000 个请求、爬虫同时抓取 100 个网页
//       这类任务大部分时间 CPU 都在"等"，用异步最合适。
//
// 【为什么不用多线程解决 IO 密集型问题？】
//   可以，但有成本：
//   - 每个线程需要分配独立的栈空间（通常 2-8 MB）
//   - 10000 个并发连接需要 10000 个线程 → 数十 GB 内存，不现实
//   - 线程切换（上下文切换）也有开销
//
// 【异步的优势】
//   - 异步任务比线程轻量得多（一个 Future 通常只需要几百字节）
//   - 一个线程可以运行成千上万个异步任务
//   - 切换成本极低（协作式调度，而非抢占式）
//
// 【什么语言有异步？】
//   - JavaScript/TypeScript：天生异步（Promise, async/await）
//   - Python：asyncio 库
//   - C#：async/await 关键字
//   - Rust：async/await + 运行时（tokio, async-std）
//
// Rust 的异步模型是"零成本抽象"——你写的 async 代码，编译后生成的状态机
// 效率和手写状态机几乎一样，但代码可读性天差地别。

// ============================================================================
// 第二部分：语法篇
// ============================================================================

// ---------------------------------------------------------------------------
// 3. async/await 基础语法
// ---------------------------------------------------------------------------
//
// 【async fn】
//   在函数签名前加 `async` 关键字，函数就变成异步函数。
//   异步函数的返回值类型会被编译器自动包装成 `impl Future<Output = T>`。
//
// 【.await】
//   在异步函数内部，调用另一个异步函数时，加上 `.await` 表示"等待它完成"。
//   注意：`.await` 不会阻塞线程！它只是告诉运行时"我要等这个 Future 完成，
//   你先去执行别的任务吧"。

// 示例 3：最简单的 async 函数
async fn say_hello() {
    println!("[示例3] 你好，我是异步函数！");
}

async fn add_async(a: i32, b: i32) -> i32 {
    // 这个函数虽然没有真正的异步操作，但 async fn 依然返回一个 Future
    a + b
}

async fn demo_basic_async() {
    // 调用异步函数不会立即执行，必须 .await 才会真正执行
    say_hello().await;

    let result = add_async(3, 5).await;
    println!("[示例3] 3 + 5 = {}", result);
}

// ---------------------------------------------------------------------------
// 4. tokio 运行时简介（#[tokio::main]）
// ---------------------------------------------------------------------------
//
// 【为什么需要运行时？】
//   async fn 返回的是一个 Future（未来值），它本身不会自动执行。
//   就像你写了一张"支票"（Future），但需要去银行"兑现"（executor 执行）。
//   tokio 就是那个"银行"——它提供了一个异步运行时（runtime），负责：
//     - 调度和执行 Future
//     - 提供定时器、网络、文件 IO 等异步基础设施
//
// 【#[tokio::main]】
//   这是一个宏，它把你的 async fn main() 转换成同步的 fn main()，
//   内部创建一个 tokio 运行时，然后在这个运行时上执行你的 async main。
//
//   你写的代码：
//     #[tokio::main]
//     async fn main() {
//         println!("Hello from async!");
//     }
//
//   编译器展开后大约等价于：
//     fn main() {
//         let rt = tokio::runtime::Runtime::new().unwrap();
//         rt.block_on(async {
//             println!("Hello from async!");
//         });
//     }
//
// 【tokio::runtime::Runtime】
//   如果你想更灵活地控制运行时，可以手动创建：
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(some_async_function());
//   这种方式适合需要在同步代码中调用异步代码的场景。

// 示例 4：tokio 运行时（见 main 函数）

// ---------------------------------------------------------------------------
// 5. 异步函数和异步块
// ---------------------------------------------------------------------------
//
// 【异步函数】
//   用 `async fn` 定义，返回 impl Future<Output = T>
//
// 【异步块】
//   用 `async { ... }` 创建一个匿名的 Future，不需要定义函数。
//   异步块可以捕获外部变量（类似闭包）。
//
// 【两者的关系】
//   async fn foo() { body }
//   大约等价于：
//   fn foo() -> impl Future<Output = ()> { async { body } }

// 示例 5：异步块
async fn demo_async_block() {
    let name = String::from("G-one");

    // 异步块：像闭包一样捕获外部变量
    let greeting = async {
        format!("你好，{}！这是来自异步块的消息。", name)
    };

    // 异步块返回一个 Future，需要 .await 才能拿到结果
    let msg = greeting.await;
    println!("[示例5] {}", msg);

    // 异步块也可以包含多个 await
    let computation = async {
        let a = add_async(10, 20).await;
        let b = add_async(a, 30).await;
        b
    };
    println!("[示例5] 计算结果: {}", computation.await);
}

// ============================================================================
// 第三部分：并发篇
// ============================================================================

// ---------------------------------------------------------------------------
// 6. tokio::spawn 并发任务
// ---------------------------------------------------------------------------
//
// 【tokio::spawn 的作用】
//   把一个 Future 提交给 tokio 运行时，让它在后台并发执行。
//   spawn 会返回一个 JoinHandle，你可以稍后 .await 它来获取结果。
//
// 【重要约束】
//   被 spawn 的 Future 必须满足 `Send + 'static`：
//   - Send：可以安全地在线程间传递（大多数类型都满足）
//   - 'static：不能持有非 'static 的引用（因为任务可能比创建它的作用域活得更久）
//
//   如果需要传递数据给 spawn 的任务，应该转移所有权（move），
//   而不是借用引用。
//
// 【与直接 .await 的区别】
//   - 直接 .await：当前任务暂停，等待子 Future 完成（串行）
//   - tokio::spawn：创建一个独立的并发任务，当前任务继续执行

// 模拟一个耗时的异步操作（比如网络请求）
async fn fetch_data(id: u32) -> String {
    // 模拟网络延迟：每个请求耗时不同
    let delay_ms = 100 * (id as u64);
    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
    format!("任务{}的数据(耗时{}ms)", id, delay_ms)
}

async fn demo_spawn() {
    println!("\n[示例6] === tokio::spawn 并发任务 ===");

    // 用 spawn 创建 3 个并发任务
    let handle1 = tokio::spawn(async {
        let result = fetch_data(1).await;
        println!("  任务1完成");
        result
    });

    let handle2 = tokio::spawn(async {
        let result = fetch_data(2).await;
        println!("  任务2完成");
        result
    });

    let handle3 = tokio::spawn(async {
        let result = fetch_data(3).await;
        println!("  任务3完成");
        result
    });

    // 等待所有任务完成，收集结果
    // .await JoinHandle 会返回 Result<T, JoinError>
    // JoinError 表示任务 panic 了
    let r1 = handle1.await.unwrap();
    let r2 = handle2.await.unwrap();
    let r3 = handle3.await.unwrap();

    println!("  收集到的结果: [{}, {}, {}]", r1, r2, r3);
    // 注意：虽然每个任务分别耗时 100ms、200ms、300ms，
    // 但由于是并发执行的，总耗时大约只有 300ms（取决于最慢的那个）
}

// ---------------------------------------------------------------------------
// 7. tokio::join! 同时等待多个任务
// ---------------------------------------------------------------------------
//
// 【tokio::join! 的作用】
//   同时等待多个 Future 全部完成，返回一个元组包含所有结果。
//   比手动 spawn + await 更简洁，且所有任务在同一个任务内并发执行。
//
// 【与 spawn 的区别】
//   - join!：所有 Future 在当前任务内并发执行（不会创建新任务）
//   - spawn：每个 Future 是独立的任务，可以在不同线程上执行
//
// 【适用场景】
//   当你需要同时做多件事，但不想创建独立任务时，用 join! 最方便。

async fn demo_join() {
    println!("\n[示例7] === tokio::join! 同时等待 ===");

    let start = std::time::Instant::now();

    // join! 会同时轮询所有 Future，直到全部完成
    let (r1, r2, r3) = tokio::join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3),
    );

    let elapsed = start.elapsed();
    println!("  结果: [{}, {}, {}]", r1, r2, r3);
    println!("  总耗时: {:?}", elapsed);
    // 预期耗时约 300ms（三个任务并行，取最慢的）
    // 如果是串行执行，应该需要 100+200+300 = 600ms
}

// ---------------------------------------------------------------------------
// 8. tokio::select! 选择第一个完成的任务
// ---------------------------------------------------------------------------
//
// 【tokio::select! 的作用】
//   同时等待多个 Future，但只取第一个完成的结果，其他的被丢弃（取消）。
//
// 【语法】
//   tokio::select! {
//       result_a = future_a => { 处理 A 的结果 }
//       result_b = future_b => { 处理 B 的结果 }
//   }
//
// 【注意事项】
//   - select! 会取消（drop）未完成的分支
//   - 如果多个分支同时完成，select! 会随机选一个（公平选择）
//   - 常见用途：超时控制、竞争请求、优雅退出

async fn demo_select() {
    println!("\n[示例8] === tokio::select! 选择最快 ===");

    // 场景：同时请求两个服务器，谁先返回用谁的
    let server_a = async {
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        "服务器A的响应"
    };

    let server_b = async {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        "服务器B的响应"
    };

    // select! 会等待第一个完成的分支
    tokio::select! {
        result = server_a => {
            println!("  先收到: {}（来自分支A）", result);
        }
        result = server_b => {
            println!("  先收到: {}（来自分支B）", result);
        }
    }

    // 实用场景：超时控制
    println!("\n[示例8] === 超时控制 ===");

    let slow_task = async {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        "慢任务完成了"
    };

    let timeout = tokio::time::sleep(std::time::Duration::from_millis(200));

    tokio::select! {
        result = slow_task => {
            println!("  任务完成: {}", result);
        }
        _ = timeout => {
            println!("  超时！任务未在 200ms 内完成");
        }
    }
}

// ============================================================================
// 第四部分：通信与IO篇
// ============================================================================

// ---------------------------------------------------------------------------
// 9. 异步 Channel（tokio::sync::mpsc）
// ---------------------------------------------------------------------------
//
// 【Channel 是什么？】
//   Channel（通道）是异步任务之间传递消息的方式，类似于一条传送带：
//   - 发送端（tx）：把消息放上传送带
//   - 接收端（rx）：从传送带上取消息
//
// 【mpsc = Multiple Producer, Single Consumer】
//   - 多个生产者：可以有多个 tx 克隆，从不同任务发送消息
//   - 单个消费者：只有一个 rx，按顺序接收消息
//
// 【为什么不用标准库的 std::sync::mpsc？】
//   标准库的 channel 是同步的（recv 会阻塞线程）。
//   在异步代码中使用同步 channel 会阻塞整个运行时线程，
//   导致其他异步任务无法执行。tokio 的 channel 是异步的，
//   recv().await 会让出执行权，不阻塞线程。

async fn demo_channel() {
    println!("\n[示例9] === 异步 Channel ===");

    // 创建一个容量为 32 的 mpsc channel
    // 参数是缓冲区大小：如果 tx 发送时 rx 还没来得及消费，
    // 最多可以缓存 32 条消息，之后发送端会等待
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    // 创建 3 个生产者任务
    for i in 1..=3 {
        let tx_clone = tx.clone(); // 克隆发送端（多生产者）
        tokio::spawn(async move {
            for j in 1..=3 {
                let msg = format!("任务{}-消息{}", i, j);
                // send 是异步的，如果缓冲区满了会等待
                tx_clone.send(msg).await.unwrap();
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
        });
    }

    // 重要：必须 drop 原始的 tx，否则 rx 会一直等待（因为还有发送者存在）
    drop(tx);

    // 消费端：逐条接收消息
    let mut count = 0;
    // recv() 返回 Option<T>，None 表示所有发送端都已关闭
    while let Some(msg) = rx.recv().await {
        count += 1;
        println!("  收到: {}", msg);
    }
    println!("  共收到 {} 条消息", count);
    // 预期收到 9 条消息（3个任务 x 3条消息）
}

// ---------------------------------------------------------------------------
// 10. 异步文件IO（tokio::fs）
// ---------------------------------------------------------------------------
//
// 【tokio::fs 的作用】
//   提供异步版本的文件读写操作。和 std::fs 功能类似，但不会阻塞线程。
//
// 【底层原理】
//   tokio::fs 内部其实还是用了线程池来执行真正的磁盘IO操作，
//   但对调用者来说是异步的——await 时不会阻塞当前任务。
//
// 【常用函数】
//   - tokio::fs::read(path)        -> 异步读取文件内容为字节
//   - tokio::fs::read_to_string(path) -> 异步读取文件内容为字符串
//   - tokio::fs::write(path, data)  -> 异步写入文件
//   - tokio::fs::create_dir(path)   -> 异步创建目录
//   - tokio::fs::remove_file(path)  -> 异步删除文件
//   - tokio::fs::File::open(path)   -> 异步打开文件（支持流式读写）

async fn demo_async_file_io() {
    use tokio::io::AsyncWriteExt; // 提供 write_all 等异步方法

    println!("\n[示例10] === 异步文件IO ===");

    let file_path = "test_async_demo.txt";

    // --- 写入文件 ---
    let content = "这是异步写入的内容\n第二行\n第三行\n";
    // write 是 tokio::fs 的便捷函数，一步完成写入
    match tokio::fs::write(file_path, content).await {
        Ok(_) => println!("  文件写入成功: {}", file_path),
        Err(e) => {
            println!("  写入失败: {}", e);
            return;
        }
    }

    // --- 读取文件 ---
    match tokio::fs::read_to_string(file_path).await {
        Ok(text) => println!("  文件内容:\n{}", text),
        Err(e) => println!("  读取失败: {}", e),
    }

    // --- 流式写入（适合大文件） ---
    let stream_path = "test_async_stream.txt";
    match tokio::fs::File::create(stream_path).await {
        Ok(mut file) => {
            // 分多次写入
            file.write_all(b"Stream write: line 1\n").await.unwrap();
            file.write_all(b"Stream write: line 2\n").await.unwrap();
            file.write_all(b"Stream write: line 3\n").await.unwrap();
            // flush 确保数据写入磁盘
            file.flush().await.unwrap();
            println!("  流式写入完成: {}", stream_path);
        }
        Err(e) => println!("  创建文件失败: {}", e),
    }

    // --- 流式读取（使用 BufReader） ---
    use tokio::io::AsyncBufReadExt; // 提供 lines() 等方法
    match tokio::fs::File::open(stream_path).await {
        Ok(file) => {
            let reader = tokio::io::BufReader::new(file);
            let mut lines = reader.lines();
            println!("  流式读取:");
            while let Some(line) = lines.next_line().await.unwrap() {
                println!("    {}", line);
            }
        }
        Err(e) => println!("  打开文件失败: {}", e),
    }

    // 清理临时文件
    let _ = tokio::fs::remove_file(file_path).await;
    let _ = tokio::fs::remove_file(stream_path).await;
    println!("  临时文件已清理");
}

// ============================================================================
// 第五部分：实战项目
// ============================================================================
//
// 异步 HTTP 请求器（模拟版）
//
// 【项目目标】
//   模拟一个异步 HTTP 请求器，演示异步编程的核心模式：
//   - 用 tokio::spawn 创建多个并发任务
//   - 每个任务模拟一个耗时操作（用 sleep 模拟网络延迟）
//   - 用 channel 把结果发回主线程
//   - 主线程汇总所有结果
//
// 【真实场景对应】
//   在真实项目中，网络请求会用 reqwest 等库替换 sleep。
//   这里用 sleep 模拟，重点展示异步架构模式。
//
// 【数据流】
//   main
//     |
//     +-- spawn task 1 --[模拟请求]-- tx.send(result1) --+
//     +-- spawn task 2 --[模拟请求]-- tx.send(result2) --+--> rx.recv() --> 汇总
//     +-- spawn task 3 --[模拟请求]-- tx.send(result3) --+
//     +-- ...                                              |
//     +-- spawn task N --[模拟请求]-- tx.send(resultN) --+

// 模拟的 URL 和响应数据
struct MockRequest {
    url: String,
    delay_ms: u64,  // 模拟的网络延迟
}

// 模拟的响应
struct MockResponse {
    url: String,
    status: u16,       // HTTP 状态码
    body_len: usize,   // 响应体长度（模拟）
    elapsed_ms: u64,   // 实际耗时
}

// 模拟发送 HTTP GET 请求
// 真实场景中这里会用 reqwest::get(url).await
async fn mock_http_get(request: MockRequest) -> MockResponse {
    let start = std::time::Instant::now();

    // 模拟网络延迟
    tokio::time::sleep(std::time::Duration::from_millis(request.delay_ms)).await;

    // 模拟偶尔失败的请求
    let (status, body_len) = if request.delay_ms > 800 {
        // 延迟太大的模拟超时
        (408, 0) // Request Timeout
    } else {
        (200, request.url.len() * 100) // 假数据
    };

    let elapsed_ms = start.elapsed().as_millis() as u64;

    MockResponse {
        url: request.url,
        status,
        body_len,
        elapsed_ms,
    }
}

async fn demo_project() {
    println!("\n========================================");
    println!("  实战项目：异步 HTTP 请求器（模拟版）");
    println!("========================================\n");

    // 准备一批"请求"
    let requests = vec![
        MockRequest { url: "https://api.example.com/users".into(),     delay_ms: 200 },
        MockRequest { url: "https://api.example.com/posts".into(),      delay_ms: 350 },
        MockRequest { url: "https://api.example.com/comments".into(),   delay_ms: 150 },
        MockRequest { url: "https://api.example.com/albums".into(),     delay_ms: 500 },
        MockRequest { url: "https://api.example.com/photos".into(),     delay_ms: 100 },
        MockRequest { url: "https://api.example.com/todos".into(),      delay_ms: 900 }, // 这个会"超时"
    ];

    println!("准备发送 {} 个请求...\n", requests.len());

    // 创建 channel，用于任务把结果发回主线程
    let (tx, mut rx) = tokio::sync::mpsc::channel::<MockResponse>(32);

    let total_start = std::time::Instant::now();

    // 为每个请求 spawn 一个异步任务
    let mut handles = Vec::new();
    for request in requests {
        let tx_clone = tx.clone();
        let url = request.url.clone();

        let handle = tokio::spawn(async move {
            // 发送请求（模拟）
            let response = mock_http_get(request).await;

            // 把结果通过 channel 发回主线程
            // 注意：这里不直接返回结果，而是通过 channel 发送
            // 这样主线程可以在任务进行中就逐步接收结果
            let _ = tx_clone.send(response).await;
        });

        handles.push((url, handle));
    }

    // 必须 drop 原始 tx，否则 rx.recv() 永远不会返回 None
    drop(tx);

    // 主线程在另一个任务中收集结果
    // 这里演示两种方式：1. 通过 channel 收集  2. 等待所有 join handle

    // 方式1：通过 channel 逐步接收结果
    println!("--- 通过 Channel 接收结果（按完成顺序）---");
    let mut success_count = 0u32;
    let mut fail_count = 0u32;
    let mut total_bytes = 0usize;

    while let Some(response) = rx.recv().await {
        if response.status == 200 {
            success_count += 1;
            total_bytes += response.body_len;
            println!(
                "  [OK]   {} -> {} bytes ({}ms)",
                response.url, response.body_len, response.elapsed_ms
            );
        } else {
            fail_count += 1;
            println!(
                "  [FAIL] {} -> 状态码 {} ({}ms)",
                response.url, response.status, response.elapsed_ms
            );
        }
    }

    // 方式2：也可以等待所有 spawn 的任务彻底结束
    // （确保 channel 收完后，任务也执行完毕了）
    for (url, handle) in handles {
        if let Err(e) = handle.await {
            println!("  任务异常 {}: {}", url, e);
        }
    }

    let total_elapsed = total_start.elapsed();

    // 汇总报告
    println!("\n--- 汇总报告 ---");
    println!("  成功: {} 个请求", success_count);
    println!("  失败: {} 个请求", fail_count);
    println!("  总数据量: {} bytes", total_bytes);
    println!("  总耗时: {:?}", total_elapsed);
    println!("  (如果是串行执行，预计耗时: {}ms)",
        200 + 350 + 150 + 500 + 100 + 900);
    println!("\n结论：异步并发将 2200ms 的串行工作压缩到了约 {:?}！", total_elapsed);
}

// ============================================================================
// main 函数：按顺序运行所有示例
// ============================================================================

#[tokio::main]
async fn main() {
    println!("============================================================================");
    println!("  Rust 异步编程入门 —— async/await 与 tokio");
    println!("  by G-one");
    println!("============================================================================\n");

    // 第二部分：语法篇
    println!("========== 第二部分：语法篇 ==========");
    demo_basic_async().await;
    demo_async_block().await;

    // 第三部分：并发篇
    println!("\n========== 第三部分：并发篇 ==========");
    demo_spawn().await;
    demo_join().await;
    demo_select().await;

    // 第四部分：通信与IO篇
    println!("\n========== 第四部分：通信与IO篇 ==========");
    demo_channel().await;
    demo_async_file_io().await;

    // 第五部分：实战项目
    demo_project().await;

    println!("\n============================================================================");
    println!("  全部示例运行完毕！");
    println!("============================================================================");
}

// ============================================================================
// 总结与进阶方向
// ============================================================================
//
// 【本课小结】
//   1. 异步编程 = 遇到等待时让出执行权，而非阻塞线程
//   2. async fn 返回 Future，.await 驱动执行
//   3. tokio 是 Rust 最流行的异步运行时
//   4. tokio::spawn 创建并发任务
//   5. join! 等待多个任务全部完成
//   6. select! 选择第一个完成的任务（常用于超时和竞争）
//   7. mpsc channel 用于异步任务间通信
//   8. tokio::fs 提供异步文件操作
//
// 【进阶方向】
//   - Stream（异步迭代器）：处理流式数据
//   - Mutex / RwLock：tokio 的异步锁（不要用 std 的同步锁！）
//   - JoinSet：动态管理一组任务
//   - Pin 和 Future trait 的底层原理
//   - async trait 方法（Rust 1.75+ 已原生支持）
//   - 错误处理模式：anyhow + thiserror 在异步代码中的使用
//
// 【常见陷阱】
//   1. 在异步代码中使用 std::thread::sleep → 会阻塞整个运行时！
//      正确做法：用 tokio::time::sleep
//   2. 在异步代码中使用 std::sync::Mutex → 可能导致死锁！
//      正确做法：用 tokio::sync::Mutex（或者在不跨 .await 时用 std 的也行）
//   3. 忘记 .await → Future 不会执行，编译器会警告
//   4. 在 spawn 的任务中借用局部变量 → 生命周期不够长
//      正确做法：用 move 闭包转移所有权
//
// ============================================================================
