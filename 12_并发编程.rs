// ============================================================================
//  第 12 课：并发编程 —— 多个厨师同时做菜
// ============================================================================
//
//  想象你开了一家餐厅：
//  - 只有一个厨师，客人点了10道菜，只能一道一道做，非常慢。
//  - 如果有5个厨师，每人做2道菜，速度就快多了！
//  - 但多个厨师共用一个厨房，需要协调：刀只有一把，烤箱只有一个……
//
//  这就是"并发编程"要解决的问题：
//  - 让多个任务"同时"进行（提高效率）
//  - 协调它们对共享资源的访问（避免混乱）
//
//  本节课你将学到：
//  1. 线程（Thread）—— 多个厨师同时工作
//  2. 通道（Channel）—— 传菜窗口，厨师把菜放到窗口，服务员端走
//  3. 互斥锁（Mutex）—— 一次只能一个人用的厨房
//  4. Arc —— 共享钥匙，多个人都能开同一扇门
//  5. Send / Sync —— Rust 如何保证并发安全
//
// ============================================================================

// 引入标准库中的同步原语模块
use std::sync::{Arc, Mutex, mpsc};
// thread 模块提供了线程相关功能
use std::thread;
// Duration 用于设置线程休眠时间
use std::time::Duration;

fn main() {
    println!("============================================================");
    println!("  第 12 课：并发编程 —— 多个厨师同时做菜");
    println!("============================================================\n");

    // ================================================================
    //  第一节：并发 vs 并行 —— 有什么区别？
    // ================================================================
    //
    //  【生活类比】
    //  - 并发（Concurrency）：一个人交替做多件事。
    //    比如你一边煮饭一边炒菜：先煮上饭，等饭的时候去炒菜，
    //    炒完菜饭也好了。你并没有"同时"做两件事，而是在交替切换。
    //
    //  - 并行（Parallelism）：多个人同时做不同的事。
    //    比如两个厨师，一个煮饭一个炒菜，真正"同时"进行。
    //
    //  【关键区别】
    //  - 并发是"逻辑上同时"：任务交替执行，看起来同时在进行
    //  - 并行是"物理上同时"：需要多个 CPU 核心，真正同时执行
    //
    //  在 Rust 中，我们通过"线程"来实现并发和并行。
    //  如果你的电脑有多个 CPU 核心，Rust 会自动把不同线程分配到不同核心上，
    //  实现真正的并行。
    //
    println!("--- 第一节：并发 vs 并行 ---");
    println!("并发 = 一个人交替做多件事（逻辑上同时）");
    println!("并行 = 多个人同时做不同的事（物理上同时）");
    println!("Rust 通过线程实现并发，多核 CPU 上自动并行执行\n");

    demo_thread_basics();
    demo_join_handle();
    demo_move_closure();
    demo_channel();
    demo_multiple_producers();
    demo_mutex();
    demo_arc();
    demo_arc_mutex();
    demo_send_sync();
    demo_rwlock_hint();
    demo_async_hint();

    // 实战项目
    project_concurrent_file_processor();

    // 练习题
    exercises();

    // 课程总结
    summary();
}

// ================================================================
//  第二节：thread::spawn 创建线程 —— 招聘厨师
// ================================================================
//
//  【生活类比】
//  线程就像餐厅里的厨师。主线程（main 函数）是餐厅经理，
//  经理可以用 thread::spawn "招聘"一个新厨师，让他去做另一道菜。
//
//  thread::spawn 接收一个闭包（closure），闭包里的代码会在新线程中运行。
//
fn demo_thread_basics() {
    println!("--- 第二节：创建线程 ---");

    // 使用 thread::spawn 创建一个新线程
    // spawn 的意思是"产生、孵化"，就像孵化一个新厨师
    // 返回值是一个 JoinHandle（后面会讲怎么用它）
    let handle = thread::spawn(|| {
        // 这段代码在新线程中执行
        // 就像新厨师开始做菜
        for i in 1..=3 {
            println!("  [新线程] 做菜步骤 {}/3...", i);
            // 让线程休眠一小会儿，模拟做菜需要时间
            // sleep 会让当前线程暂停指定的时间
            thread::sleep(Duration::from_millis(100));
        }
        println!("  [新线程] 菜做好了！");
    });

    // 主线程同时也在做自己的事
    println!("[主线程] 经理在安排其他工作...");
    for i in 1..=3 {
        println!("  [主线程] 工作步骤 {}/3...", i);
        thread::sleep(Duration::from_millis(100));
    }
    println!("[主线程] 经理工作完成！\n");

    // 注意：如果不调用 join()，主线程可能在新线程完成之前就结束了！
    // 我们在下一节讲 join()
    // 这里先调用一下，确保新线程完成
    handle.join().unwrap();
}

// ================================================================
//  第三节：JoinHandle 和 join() —— 等厨师做完菜
// ================================================================
//
//  【生活类比】
//  经理派厨师去做菜后，不能直接关门走人——得等厨师做完、收拾好厨房。
//  join() 就是"等待"的意思：等新线程执行完毕。
//
//  JoinHandle 是 spawn 返回的"工作凭证"，拿着它可以：
//  - 调用 join() 等待线程完成
//  - join() 会阻塞当前线程，直到目标线程结束
//
fn demo_join_handle() {
    println!("--- 第三节：JoinHandle 和 join() ---");

    // 创建一个新线程，它会返回一个数字
    let handle = thread::spawn(|| {
        // 做一些计算
        let mut sum = 0;
        for i in 1..=100 {
            sum += i;
        }
        // 闭包的最后一个表达式就是返回值
        sum
    });

    // join() 会等待线程完成，并返回线程闭包的返回值
    // join() 返回的是 Result<T, E>，所以用 unwrap() 获取值
    // - Ok(value)：线程正常完成，返回值
    // - Err(e)：线程发生了 panic
    let result = handle.join().unwrap();
    println!("  1 到 100 的和 = {}", result);

    // 多个线程的情况：先启动所有线程，再依次 join
    let mut handles = vec![];
    for id in 0..5 {
        // 注意：这里用了 move |...| 语法，下一节会详细解释
        let handle = thread::spawn(move || {
            println!("  [厨师{}] 开始做菜...", id);
            thread::sleep(Duration::from_millis(50 * (id as u64 + 1)));
            println!("  [厨师{}] 做完了！", id);
            // 返回厨师编号和结果
            (id, format!("菜{}", id))
        });
        handles.push(handle);
    }

    // 依次等待所有线程完成
    // join() 会消耗 handle（因为用了 &mut self 或者说所有权转移）
    // 所以我们用 into_iter() 来消耗 Vec
    for handle in handles {
        let (id, dish) = handle.join().unwrap();
        println!("  经理收到：厨师{} 做的{}", id, dish);
    }
    println!();
}

// ================================================================
//  第四节：move 闭包 —— 把食材交给厨师
// ================================================================
//
//  【生活类比】
//  你买了一袋面粉（一个变量），想让厨师拿去做面条。
//  问题来了：面粉只有一袋，要么你留着，要么给厨师。
//  move 闭包就是"把所有权交给新线程"。
//
//  【为什么需要 move？】
//  线程可能比主线程活得更久。如果线程只是"借用"主线程的数据，
//  主线程结束时数据被销毁，线程就访问到了无效内存——这不安全！
//  所以 Rust 要求：把数据 move（移动）到新线程中，确保线程拥有数据的所有权。
//
fn demo_move_closure() {
    println!("--- 第四节：move 闭包 ---");

    let chef_name = String::from("厨师小明");

    // 如果不加 move，编译器会报错：
    // "closure may outlive the current function, but it borrows `chef_name`"
    // 因为闭包默认是借用外部变量的，但新线程可能比 main 活得更久
    //
    // 加了 move 后，chef_name 的所有权被移动到闭包中
    // 主线程就不能再使用 chef_name 了
    let handle = thread::spawn(move || {
        // 现在 chef_name 属于这个线程了
        println!("  {} 开始工作了！", chef_name);
        thread::sleep(Duration::from_millis(50));
        println!("  {} 工作完成！", chef_name);
    });

    // 如果取消下面这行的注释，编译会报错：
    // 因为 chef_name 已经被 move 到新线程了，主线程不能再用它
    // println!("{}", chef_name);  // 编译错误！

    handle.join().unwrap();

    // 如果主线程之后还需要用这个值怎么办？
    // 方法1：在 move 之前先 clone 一份
    let name = String::from("厨师小红");
    let name_clone = name.clone(); // 克隆一份给主线程

    let handle = thread::spawn(move || {
        println!("  {} 在新线程中工作", name); // 用原始值
    });
    handle.join().unwrap();
    println!("  主线程还在用：{}", name_clone); // 用克隆值
    println!();
}

// ================================================================
//  第五节：mpsc::channel 消息传递 —— 传菜窗口
// ================================================================
//
//  【生活类比】
//  餐厅里有一种"传菜窗口"：
//  - 厨师做好菜后，放到窗口上（发送消息）
//  - 服务员从窗口端走菜（接收消息）
//  - 这样厨师和服务员不需要直接交流，通过窗口传递即可
//
//  在 Rust 中，这叫做"通道"（Channel），分为两部分：
//  - tx（transmitter，发送端）：厨师用来放菜
//  - rx（receiver，接收端）：服务员用来端菜
//
//  mpsc 的全称是 "multiple producer, single consumer"
//  意思是：可以有多个厨师（多个发送端），但只有一个服务员（一个接收端）
//
fn demo_channel() {
    println!("--- 第五节：消息传递 Channel ---");

    // 创建一个通道，返回 (发送端, 接收端)
    // 类型标注：发送 i32 类型的消息
    let (tx, rx) = mpsc::channel::<i32>();

    // 创建一个新线程，让它发送消息
    thread::spawn(move || {
        // 厨师做菜，一道一道放到窗口
        let dishes = vec!["红烧肉", "糖醋鱼", "宫保鸡丁"];
        for (i, dish) in dishes.iter().enumerate() {
            println!("  [厨师] 做好了：{}", dish);
            // tx.send() 发送消息
            // send() 接收一个值，通过通道发给接收端
            // 注意：send 会消耗掉值的所有权（值被"发送"出去了）
            tx.send((i + 1) as i32).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        // tx 在这里被 drop（销毁），通道就关闭了
        // 接收端的迭代器就会结束
        println!("  [厨师] 今天所有菜都做完了！");
    });

    // 主线程作为服务员，从窗口端菜
    // rx.recv() 会阻塞当前线程，直到收到消息
    // 当发送端关闭（所有 tx 都被 drop）时，recv() 返回 Err
    println!("  [服务员] 开始端菜...");
    // rx 也可以当作迭代器使用！
    // 当通道关闭时，迭代自动结束
    for msg in rx {
        println!("  [服务员] 收到了第 {} 道菜的通知", msg);
    }
    println!("  [服务员] 所有菜都端完了！\n");
}

// ================================================================
//  第六节：多个发送者 —— tx.clone() 多个传菜窗口
// ================================================================
//
//  【生活类比】
//  如果有3个厨师，都需要往同一个传菜窗口放菜，怎么办？
//  我们可以 clone（克隆）发送端，每个厨师拿一份。
//  接收端只有一份，所有厨师的菜都会汇集到这里。
//
fn demo_multiple_producers() {
    println!("--- 第六节：多个发送者 ---");

    // 创建通道
    let (tx, rx) = mpsc::channel::<String>();

    // 创建3个厨师（3个线程），每个厨师有自己的发送端
    let chef_names = ["厨师A", "厨师B", "厨师C"];
    for (i, name) in chef_names.iter().enumerate() {
        // 克隆发送端，每个线程拿一份
        // 原始的 tx 也需要被 move 到某个线程，或者在主线程中 drop
        let tx_clone = tx.clone();
        thread::spawn(move || {
            // 每个厨师做一道菜
            let dish = format!("{}做的第{}道菜", name, i + 1);
            println!("  [{}] 正在做菜...", name);
            thread::sleep(Duration::from_millis(50 * (i as u64 + 1)));
            println!("  [{}] 做好了：{}", name, dish);
            // 用克隆的发送端发送消息
            tx_clone.send(dish).unwrap();
        });
    }

    // 重要：必须 drop 掉主线程中的原始 tx！
    // 否则接收端会一直等待（因为还有一个发送端没关闭）
    // 所有 clone 的 tx 在各自线程结束时会被 drop
    // 但原始的 tx 还在主线程中，需要手动 drop
    drop(tx);

    // 接收所有消息
    println!("  [经理] 等待所有菜上齐...");
    for dish in rx {
        println!("  [经理] 收到：{}", dish);
    }
    println!("  [经理] 所有菜都齐了！\n");
}

// ================================================================
//  第七节：Mutex<T> 互斥锁 —— 一次只能一个人用的厨房
// ================================================================
//
//  【生活类比】
//  餐厅只有一个厨房，但有多个厨师都想用。
//  怎么办？装一把锁（Mutex = Mutual Exclusion，互斥）：
//  - 一个厨师进去后，把门锁上（lock）
//  - 其他厨师在门口等着
//  - 用完厨房后，把锁打开（unlock），下一个厨师进去
//
//  【使用方法】
//  Mutex::new(value) —— 创建一个被锁保护的值
//  mutex.lock() —— 获取锁，返回 MutexGuard（智能指针）
//  MutexGuard 被 drop 时自动释放锁（离开作用域自动开锁）
//
fn demo_mutex() {
    println!("--- 第七节：Mutex 互斥锁 ---");

    // 创建一个被 Mutex 保护的数字
    // 类型是 Mutex<i32>，意思是"这个 i32 被锁保护着"
    let counter = Mutex::new(0);

    // 在单线程中使用 Mutex
    {
        // lock() 会阻塞当前线程直到获取到锁
        // 返回的是 MutexGuard，它实现了 Deref trait
        // 所以可以像普通引用一样使用
        let mut num = counter.lock().unwrap();
        // num 的类型实际上是 MutexGuard<i32>
        // 但因为实现了 Deref，我们可以直接用 *num 来访问内部值
        *num += 1;
        println!("  单线程：计数器 = {}", *num);
        // num（MutexGuard）在这里离开作用域，锁自动释放
    }

    {
        let mut num = counter.lock().unwrap();
        *num += 10;
        println!("  单线程：计数器 = {}", *num);
    }

    // lock() 返回的是 Result<MutexGuard, PoisonError>
    // 如果持有锁的线程 panic 了，锁会被"污染"（poisoned）
    // 此时 lock() 会返回 Err，用 unwrap() 就会 panic
    // 实际项目中应该处理这种情况

    println!("  注意：单线程中用 Mutex 是大材小用，");
    println!("  Mutex 的真正价值在于多线程场景！\n");
}

// ================================================================
//  第八节：Arc<T> 原子引用计数 —— 共享钥匙
// ================================================================
//
//  【生活类比】
//  想象餐厅的调料柜有一把钥匙，多个厨师都需要用。
//  我们给每个厨师配一把钥匙（Arc = Atomic Reference Counting）。
//  - 每多一个人拿到钥匙，计数 +1
//  - 每有一个人还回钥匙，计数 -1
//  - 计数归零时，调料柜可以拆除了
//
//  【为什么需要 Arc 而不是 Rc？】
//  Rc（引用计数）不是线程安全的——多个线程同时修改引用计数会出错。
//  Arc 使用"原子操作"来修改引用计数，保证线程安全。
//  atomic（原子）= 不可分割的操作，不会被线程切换打断。
//
//  【Arc 和 Rc 的对比】
//  Rc<T>  —— 单线程用，轻量，不能跨线程
//  Arc<T> —— 多线程用，稍微慢一点，但线程安全
//
fn demo_arc() {
    println!("--- 第八节：Arc 原子引用计数 ---");

    // 创建一个 Arc 包裹的值
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("  创建 Arc，引用计数 = {}", Arc::strong_count(&data));

    let mut handles = vec![];

    for id in 0..3 {
        // 克隆 Arc，引用计数 +1
        // 注意：Arc::clone() 只是增加引用计数，不会深拷贝数据
        let data_clone = Arc::clone(&data);
        println!("  克隆给线程{}，引用计数 = {}", id, Arc::strong_count(&data_clone));

        let handle = thread::spawn(move || {
            // data_clone 的所有权被 move 到新线程
            // 但底层数据是共享的，不会被复制
            println!("  [线程{}] 数据：{:?}", id, *data_clone);
            // 线程结束时，data_clone 被 drop，引用计数 -1
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 所有克隆都被 drop 了，现在只有原始的 data 还在
    println!("  所有线程结束后，引用计数 = {}", Arc::strong_count(&data));
    println!();
}

// ================================================================
//  第九节：Arc<Mutex<T>> 组合 —— 多人共用厨房
// ================================================================
//
//  【生活类比】
//  现在我们有了：
//  - Arc（共享钥匙）：让多个厨师都能进入调料柜所在的房间
//  - Mutex（厨房锁）：保证同一时间只有一个人在用厨房
//
//  组合起来 Arc<Mutex<T>>：
//  "多个线程共享同一个被锁保护的值"
//  - 每个线程拿着钥匙（Arc 克隆）进房间
//  - 用之前先锁门（lock），用完自动开门
//
//  这是 Rust 多线程共享可变状态的标准模式！
//
fn demo_arc_mutex() {
    println!("--- 第九节：Arc<Mutex<T>> 组合使用 ---");

    // 创建一个线程安全的共享计数器
    // Arc：允许多个线程共享所有权
    // Mutex：保证同一时间只有一个线程能修改
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for id in 0..5 {
        // 克隆 Arc，每个线程拿一份"钥匙"
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // lock() 获取锁
                // 如果锁被别的线程持有，当前线程会阻塞等待
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                // MutexGuard 在这里离开作用域，锁自动释放
                // 注意：如果在循环内保持锁，效率会很低
                // 因为其他线程都在等着，没法并行
            }
            println!("  [线程{}] 完成了100次加法", id);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 获取最终结果
    let final_count = *counter.lock().unwrap();
    println!("  最终计数器 = {}", final_count);
    println!("  5个线程 × 100次 = 500，正确！");
    println!();
}

// ================================================================
//  第十节：Send 和 Sync trait —— Rust 的并发安全守护者
// ================================================================
//
//  【生活类比】
//  Rust 编译器就像一个严格的安全检查员：
//  - Send：这个东西可以安全地从一个厨师传给另一个厨师
//  - Sync：这个东西可以安全地被多个厨师同时看着（引用）
//
//  几乎所有的 Rust 类型都自动实现了 Send 和 Sync，
//  但有几个"不安全"的类型不实现：
//
//  Rc<T>（引用计数）：
//  - 不是 Send：不能安全地发送到另一个线程
//  - 不是 Sync：不能安全地被多个线程共享引用
//  - 原因：Rc 的引用计数不是原子操作，多线程同时修改会出错
//  - 解决方案：用 Arc 代替 Rc
//
//  Cell<T> / RefCell<T>：
//  - 不是 Sync：内部可变性不是线程安全的
//  - 原因：它们的借用检查是在运行时做的，多线程环境不可靠
//  - 解决方案：用 Mutex 或 RwLock 代替
//
//  【总结】
//  - 需要跨线程传递？类型必须是 Send
//  - 需要多线程共享引用？类型必须是 Sync
//  - 大部分类型都自动满足，不满足的编译器会告诉你
//
fn demo_send_sync() {
    println!("--- 第十节：Send 和 Sync trait ---");

    // 以下类型都是 Send + Sync 的，可以安全地在多线程中使用：
    // - i32, f64, bool 等基本类型
    // - String, Vec<T>, HashMap 等集合类型
    // - Arc<T>（当 T 是 Send + Sync 时）
    // - Mutex<T>（当 T 是 Send 时）

    // 以下类型不是 Send 或不是 Sync：
    // - Rc<T>：不是 Send，也不是 Sync
    // - Cell<T>, RefCell<T>：不是 Sync
    // - 裸指针 *const T, *mut T：都不是

    // 演示：用 Arc（线程安全）而非 Rc（非线程安全）
    let shared = Arc::new(String::from("我是线程安全的！"));
    let shared_clone = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        println!("  新线程：{}", shared_clone);
    });
    handle.join().unwrap();
    println!("  主线程：{}", shared);

    // 如果把 Arc 换成 Rc，编译器会报错：
    // "the trait `Send` is not implemented for `Rc<String>`"
    // 这就是 Rust 的"无畏并发"——编译器帮你检查线程安全！
    println!("  Rust 编译器在编译时就能发现大部分并发 bug！");
    println!();
}

// ================================================================
//  补充：RwLock —— 读写锁
// ================================================================
//
//  【生活类比】
//  Mutex 就像一把简单的锁：一个人进去，其他人全等着。
//  RwLock 更聪明：
//  - 多个读者可以同时进去（大家只是看看，不冲突）
//  - 写者必须独占（改东西时别人不能看）
//
//  适用场景：读多写少的情况，比 Mutex 效率更高。
//
fn demo_rwlock_hint() {
    println!("--- 补充：RwLock（读写锁）---");
    println!("  RwLock 允许多个读者同时访问，但写者必须独占");
    println!("  适用于读多写少的场景，比 Mutex 更高效");
    println!("  用法：std::sync::RwLock::new(value)");
    println!("  读操作：rwlock.read() —— 获取读锁（多个线程可同时持有）");
    println!("  写操作：rwlock.write() —— 获取写锁（独占）");
    println!("  本课程不深入，记住它存在就好！\n");
}

// ================================================================
//  补充：async/await —— 异步编程
// ================================================================
//
//  【生活类比】
//  线程就像雇一个新厨师：每个厨师都要占一个工位（系统资源）。
//  如果有10000个任务，雇10000个厨师太贵了！
//
//  async/await 是另一种思路：
//  - 只雇几个厨师（线程）
//  - 每个厨师在等待（比如等水烧开）时，去做别的事
//  - 等水开了再回来继续
//  - 这样几个厨师就能处理大量任务
//
//  【注意】
//  Rust 的 async/await 需要运行时（如 tokio）来驱动。
//  标准库只提供了 async/await 语法，没有提供运行时。
//  本课程只用标准库，所以不深入 async。
//  知道它存在就好，以后项目中会用到！
//
fn demo_async_hint() {
    println!("--- 补充：async/await 异步编程 ---");
    println!("  async/await 是另一种并发模型，适合 I/O 密集型任务");
    println!("  Rust 标准库只提供语法，需要第三方运行时（如 tokio）");
    println!("  本课程不深入，后续项目课程会专门讲解！\n");
}

// ================================================================
//  实战项目：并发文件处理器
// ================================================================
//
//  项目目标：
//  1. 创建多个临时文本文件
//  2. 多线程分别读取每个文件，统计词频
//  3. 用 channel 把结果发回主线程
//  4. 主线程合并所有结果
//  5. 打印最终词频统计
//
//  【为什么用并发？】
//  如果有100个文件，一个一个读太慢。
//  用多线程同时读取，速度可以快很多！
//
fn project_concurrent_file_processor() {
    println!("============================================================");
    println!("  实战项目：并发文件处理器");
    println!("============================================================\n");

    // ---- 第一步：创建临时文件 ----
    // 定义一些测试文本文件的内容
    let files = vec![
        ("file1.txt", "hello world hello rust hello programming"),
        ("file2.txt", "rust is great rust is fast rust is safe"),
        ("file3.txt", "hello from rust world of concurrency"),
        ("file4.txt", "programming in rust is fun and safe"),
        ("file5.txt", "world of rust concurrency programming"),
    ];

    // 在临时目录创建文件
    let temp_dir = std::env::temp_dir().join("rust_concurrent_demo");
    // 创建临时目录（如果不存在）
    std::fs::create_dir_all(&temp_dir).expect("无法创建临时目录");

    for (name, content) in &files {
        let path = temp_dir.join(name);
        // std::fs::write 直接写入文件内容
        std::fs::write(&path, content).expect("无法写入文件");
        println!("  创建文件：{}", path.display());
    }
    println!();

    // ---- 第二步：创建通道用于收集结果 ----
    // 发送端给线程用，接收端给主线程用
    let (tx, rx) = mpsc::channel::<Vec<(String, usize)>>();

    // ---- 第三步：为每个文件创建一个处理线程 ----
    let mut handles = vec![];

    for (name, _content) in &files {
        let file_path = temp_dir.join(name);
        // 克隆发送端（每个线程需要自己的发送端）
        let tx_clone = tx.clone();

        let handle = thread::spawn(move || {
            // 读取文件内容
            // std::fs::read_to_string 读取整个文件为字符串
            let content = std::fs::read_to_string(&file_path)
                .expect(&format!("无法读取文件：{}", file_path.display()));

            // 统计词频
            // split_whitespace() 按空白字符分割字符串
            let mut word_count: Vec<(String, usize)> = Vec::new();

            for word in content.split_whitespace() {
                // to_lowercase() 统一转小写，避免 "Hello" 和 "hello" 算两个词
                let word_lower = word.to_lowercase();

                // 查找这个词是否已经在结果中
                let mut found = false;
                for entry in word_count.iter_mut() {
                    if entry.0 == word_lower {
                        entry.1 += 1; // 计数 +1
                        found = true;
                        break;
                    }
                }
                if !found {
                    // 第一次出现，添加到结果中
                    word_count.push((word_lower, 1));
                }
            }

            // 通过通道发送结果给主线程
            tx_clone.send(word_count).unwrap();
            println!("  [线程] 处理完成：{}", file_path.display());
        });
        handles.push(handle);
    }

    // 重要：drop 掉主线程的原始发送端
    // 否则接收端不会结束（因为还有一个发送端没关闭）
    drop(tx);

    // ---- 第四步：主线程收集并合并结果 ----
    // 用 HashMap 来合并所有线程的词频结果
    // 但我们只用标准库的方式，手动实现合并
    let mut final_word_count: Vec<(String, usize)> = Vec::new();

    // 从通道接收所有线程的结果
    for word_count in rx {
        for (word, count) in word_count {
            // 在最终结果中查找这个词
            let mut found = false;
            for entry in final_word_count.iter_mut() {
                if entry.0 == word {
                    entry.1 += count; // 合并计数
                    found = true;
                    break;
                }
            }
            if !found {
                final_word_count.push((word, count));
            }
        }
    }

    // 等待所有线程完成（确保所有临时文件操作完成后再清理）
    for handle in handles {
        handle.join().unwrap();
    }

    // ---- 第五步：按词频排序并打印结果 ----
    // 冒泡排序（从大到小），简单易懂
    let len = final_word_count.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if final_word_count[j].1 < final_word_count[j + 1].1 {
                final_word_count.swap(j, j + 1);
            }
        }
    }

    println!("  ---- 最终词频统计 ----");
    println!("  {:<20} | 出现次数", "单词");
    println!("  {:->20}-+-{:-<8}", "", "");
    for (word, count) in &final_word_count {
        println!("  {:<20} | {}", word, count);
    }
    println!();

    // ---- 清理临时文件 ----
    // 使用 std::fs::remove_dir_all 递归删除目录
    match std::fs::remove_dir_all(&temp_dir) {
        Ok(_) => println!("  临时文件已清理：{}", temp_dir.display()),
        Err(e) => println!("  清理临时文件失败：{}", e),
    }
    println!();
}

// ================================================================
//  练习题
// ================================================================

fn exercises() {
    println!("============================================================");
    println!("  练习题");
    println!("============================================================\n");

    // --------------------------------------------------------
    //  练习一（基础巩固，5-10 分钟）：
    //  创建 5 个线程，每个线程打印自己的编号，用 join 等待全部完成
    //
    //  提示：
    //  - 用 thread::spawn 创建线程
    //  - 用 move 把编号移入闭包
    //  - 把 JoinHandle 存到 Vec 中
    //  - 遍历 Vec，对每个 handle 调用 join()
    // --------------------------------------------------------
    println!("--- 练习一：5个线程打印编号 ---");
    println!("  （请先自己完成，再看参考答案）\n");

    // 参考答案
    exercise1_answer();

    // --------------------------------------------------------
    //  练习二（应用练习，15-20 分钟）：
    //  用 channel 实现生产者-消费者模式
    //  - 3 个生产者（线程），每个生产者生产 5 个产品
    //  - 1 个消费者（主线程），接收并处理所有产品
    //  - 产品格式："生产者X-产品Y"
    //
    //  提示：
    //  - 创建 channel
    //  - clone tx 给每个生产者
    //  - 每个生产者用 for 循环生产产品，通过 tx.send() 发送
    //  - 主线程用 rx 接收所有产品
    //  - 别忘了 drop 原始的 tx！
    // --------------------------------------------------------
    println!("--- 练习二：生产者-消费者模式 ---");
    println!("  （请先自己完成，再看参考答案）\n");

    // 参考答案
    exercise2_answer();

    // --------------------------------------------------------
    //  练习三（进阶挑战，选做）：
    //  用 Arc<Mutex<T>> 实现并发计数器
    //  - 10 个线程
    //  - 每个线程对计数器加 1000 次
    //  - 最终结果应该是 10000
    //
    //  提示：
    //  - Arc<Mutex<i32>> 作为共享计数器
    //  - 每个线程 clone Arc
    //  - 循环 1000 次，每次 lock() 后 +1
    //  - 注意：lock() 的作用域要尽量短（锁住的时间越短越好）
    // --------------------------------------------------------
    println!("--- 练习三：并发计数器（进阶挑战）---");
    println!("  （请先自己完成，再看参考答案）\n");

    // 参考答案
    exercise3_answer();
}

// 练习一参考答案
fn exercise1_answer() {
    let mut handles = vec![];

    for id in 1..=5 {
        // move 把 id 的所有权移入闭包
        // 注意：i32 实现了 Copy trait，所以 move 的是副本
        let handle = thread::spawn(move || {
            println!("  [线程{}] 我是第 {} 号线程！", id, id);
        });
        handles.push(handle);
    }

    // 依次 join 所有线程
    for (i, handle) in handles.into_iter().enumerate() {
        handle.join().unwrap();
        println!("  [主线程] 线程{} 已完成", i + 1);
    }
    println!();
}

// ===== 测试版练习 =====
// 如果你想体验 TDD（测试驱动开发），可以先写测试，再写实现：
// 取消下面的注释，运行 cargo test --example lesson12

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_thread_counter_with_arc_mutex() {
        // 练习一的核心延伸：用 Arc<Mutex<T>> 实现并发计数器
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..5 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let mut num = counter_clone.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let result = *counter.lock().unwrap();
        assert_eq!(result, 500); // 5 个线程 x 100 次 = 500
    }
}
*/

// 练习二参考答案
fn exercise2_answer() {
    // 创建通道
    let (tx, rx) = mpsc::channel::<String>();

    // 创建 3 个生产者
    for producer_id in 1..=3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for product_id in 1..=5 {
                let product = format!("生产者{}-产品{}", producer_id, product_id);
                tx_clone.send(product).unwrap();
                // 模拟生产需要时间
                thread::sleep(Duration::from_millis(10));
            }
        });
        // 注意：tx_clone 在这里超出作用域会被 drop
        // 但 spawn 的闭包已经 move 了它，所以这里没问题
    }

    // drop 原始的 tx，这样当所有生产者完成后，rx 会结束
    drop(tx);

    // 消费者（主线程）接收并处理产品
    let mut count = 0;
    for product in rx {
        println!("  [消费者] 收到：{}", product);
        count += 1;
    }
    println!("  [消费者] 共收到 {} 个产品（3个生产者 × 5个产品 = 15）", count);
    println!();
}

// 练习三参考答案
fn exercise3_answer() {
    // 创建共享计数器
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for id in 1..=10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // 锁住计数器，加1，然后立即释放锁
                // 这里 lock() 返回 MutexGuard
                // *counter_clone 解引用得到内部的 i32
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                // num（MutexGuard）在这里离开作用域，锁自动释放
            }
            println!("  [线程{}] 完成了 1000 次加法", id);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 获取最终结果
    let result = *counter.lock().unwrap();
    println!("  最终计数器 = {}", result);
    println!("  10个线程 × 1000次 = 10000，正确！");
    println!();
}

// ================================================================
//  课程总结
// ================================================================

fn summary() {
    println!("============================================================");
    println!("  课程总结");
    println!("============================================================");
    println!();
    /*
     * 核心收获：
     * - thread::spawn 创建新线程，move 闭包把数据所有权交给线程
     * - mpsc::channel 实现线程间消息传递，tx.clone() 实现多发送者
     * - Arc<Mutex<T>> 是 Rust 多线程共享可变状态的标准模式
     *
     * 常见陷阱：
     * - 忘记 drop 原始 tx：导致接收端永远等待，程序不会结束
     * - MutexGuard 持有时间过长：在循环内保持锁会导致其他线程饿死，应尽快释放
     *
     * 下节课预告：
     * - 下节课学 Cargo 项目管理，正式进入项目开发！
     */
}
