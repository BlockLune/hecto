# Hecto

一个使用 Rust 编写的简易文本编辑器，详见[教程](https://www.flenker.blog/hecto/)。

## 学习笔记

### Cargo

Cargo 是 Rust 的包管理器，常用命令如下：

```shell
# 初始化一个禁用了版本控制的项目 hecto
cargo init hecto --vcs none

# Debug 构建：构建至 `./target/debug/` 目录
cargo build

# Release 构建：构建至 `./target/release/` 目录
cargo build --release

# 基于 `Cargo.lock` 文件中的内容进行构建
cargo build --locked

# Debug 运行
cargo run

# 清理
cargo clean

# 生成 rustdoc（需要在代码中使用 `///` 创建注释）
cargo doc --open
```

### Canonical Mode & Raw Mode

默认情况下，我们进入的是“规范模式 (Canonical Mode)” / “熟模式 (Cooked Mode)”，但对于我们的文本编辑器，我们需要的是“原始模式 (Raw Mode)”。我们将借助 [crossterm](https://docs.rs/crossterm/latest/crossterm/) 来实现这个目标。

> By default your terminal starts in **canonical mode**, also called **cooked mode**. In this mode, keyboard input is only sent to your program when the user presses `Enter`.

### `Result` 和 `match` 语句

在其他编程语言中，函数实际上有两种方法将控制权返回给调用它的代码：

- 返回值
- 异常

在 Rust 中，可以借由 `Result` 这个特殊的类型来直接将可能的异常包裹在返回值中。它有两个变体，`Ok()` 和 `Err()`。

下面的例子展示了 `Result` 的用法，同时，展示了其与 `match` 语句的配合使用：

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    // 成功的情况
    match divide(10.0, 2.0) {
        Ok(result) => println!("结果: {}", result),  // 输出: 结果: 5
        Err(e) => println!("错误: {}", e),
    }

    // 失败的情况
    match divide(10.0, 0.0) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e),  // 输出: 错误: 除数不能为零
    }
}
```

如果不想进行结构，希望直接获取有效值，或在**出现错误时直接终止程序**，可以使用 `Result` 类型的 `unwrap()` 方法：

```rust
fn main() {
    // 成功情况 - 正常获取值
    let result = divide(10.0, 2.0).unwrap();
    println!("结果: {}", result);  // 输出: 结果: 5

    // 失败情况 - 程序会 panic 并终止
    // let result = divide(10.0, 0.0).unwrap();  // 这行会崩溃程序
}
```

可以使用 `?` 来向外层自动传播错误（注意下面的例子中，外层函数的返回值类型也变为了 `Result`）：

```rust
fn calculate() -> Result<f64, String> {
    let a = divide(10.0, 2.0)?;  // 自动传播错误
    let b = divide(a, 3.0)?;     // 自动传播错误
    Ok(b)
}
```

### Clippy

Clippy 是 Rust 的一个 Linter，来源于 MS Office 97 的那个回形针功能。常用命令如下：

```shell
# 运行 clippy
cargo clippy

# 让 clippy 进行更广泛的检查
# 其中的 pedentic 可以让 clippy 建议所有可能的惯用改进
cargo clippy -- -W clippy::all  -W clippy::pedantic

# 让 clippy 尝试帮我们修复某些问题
cargo clippy --fix -- -W clippy::all  -W clippy::pedantic
```

通过在 `main.rs` 的顶部添加下面的代码，可以告诉 clippy 我们希望默认启用的标志：

```rust
#![warnall, clippy::pedantic]
```

事实上 `all` 并不包含所有内容，它包含的是 `correctness`、`suspicious`、`style`、`complexity`、`perf` 等类别的内容。

具体可见 [Clippy 文档](https://doc.rust-lang.org/clippy/index.html)。

### VT100

被现代终端广泛支持的转义序列，参见 [VT100 用户指南](http://vt100.net/docs/vt100-ug/chapter3.html)。

`crossterm` 使我们不需要手动输入这些序列以实现清屏等操作。

### 函数签名

```rust
pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error>
```

- `pub`：指明这个函数可在本文件外被访问
- `fn`：指明这是一个函数
- `->`：后跟这个函数的返回类型（注意如果这个函数有返回值，那这就是必须写的，无法自动推导）

### 整型类型

```rust
fn main() {

    let small_u: u8 = std::u8::MAX;
    let medium_u: u16 = std::u16::MAX;
    let large_u: u32 = std::u32::MAX;
    let extra_large_u: u64 = std::u64::MAX;

    let small_i_min: i8 = std::i8::MIN;
    let small_i_max: i8 = std::i8::MAX;
    let medium_i_min: i16 = std::i16::MIN;
    let medium_i_max: i16 = std::i16::MAX;
    let large_i_min: i32 = std::i32::MIN;
    let large_i_max: i32 = std::i32::MAX;
    let extra_large_i_min: i64 = std::i64::MIN;
    let extra_large_i_max: i64 = std::i64::MAX;

    println!("Unsigned integers:");
    println!("u8 max: {}", small_u);
    println!("u16 max: {}", medium_u);
    println!("u32 max: {}", large_u);
    println!("u64 max: {}", extra_large_u);

    println!("Signed integers:");
    println!("i8 min: {}, max: {}", small_i_min, small_i_max);
    println!("i16 min: {}, max: {}", medium_i_min, medium_i_max);
    println!("i32 min: {}, max: {}", large_i_min, large_i_max);
    println!("i64 min: {}, max: {}", extra_large_i_min, extra_large_i_max);
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d9ab1c4613e62957ed2a759c48c2d892)

### 限定参数必须实现某个特性

下面的两个签名是等价的，都限定了一个实现了 `Command` 特性的参数：

```rust
fn queue_command(command: impl Command) -> Result<(), Error>
```

```rust
fn queue_command<T:Command>(command: T) -> Result<(), Error>
```

### rustdoc

通过 `///` 可以创建 rustdoc（类似 JSDoc）：

```rust
/// 把两个数相加
/// # Examples
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 整型溢出

在 Rust 里，整型溢出（arithmetic overflow / underflow）在 Debug 和 Release 两种构建模式下的行为是 有明确区别的，这属于编译器的一种“双态策略”。

- Debug 模式：在每一次可能溢出的算数操作后插入运行时检查，如果发生溢出，立刻 `panic`；
- Release 模式：不包含任何溢出检查，如果发生溢出，直接按二进制补码环绕（wrap-around）处理，例如 `i32:MAX + 1` 得到 `i32:MIN`。

为了防止溢出，可以使用 `saturating_sub`、`saturating_add` 等方法。

### `if let` 和 `while let` 语句

如果只关心一种匹配情况，使用 `match` 可能过重。此时可以使用 `if let` 语句：

```rust
if let Some(v) = maybe {
    // 只有当 maybe 是 Some(v) 时才进此分支
    println!("got {}", v);
} else {
    // 其余情况（这里是 None）
    println!("nothing");
}
```

上面的代码等价于：

```rust
match maybe {
    Some(v) => println!("got {}", v),
    _ => println!("nothing"),
}
```

`if let` 只进行一次匹配，使用 `while let` 以进行循环匹配直至终止：

```rust
fn main() {
    let mut stack = vec![1, 2, 3];

    // 只要 pop() 返回 Some(elem) 就一直循环
    while let Some(top) = stack.pop() {
        println!("popped {}", top);
    }

    println!("stack is now empty");
}
```

### 字符串（字面量、切片和 `String`）

Rust 中有两种文本表示方式：[`str`](https://doc.rust-lang.org/std/primitive.str.html) 和 `String`。

一个 `str` 指的是内存中的一个字节序列，我们更多地与 `&str` 交互，即指向内存中字节序列的指针。我们称 `str` 为 _字面量字符串（literal string）_，而 `&str` 为 _字符串切片（String Slice）_。

```rust
fn main() {
    let slice: &str = "Hello, World";
    dbg!(slice.as_ptr());
    dbg!(slice.len());
    dbg!(slice.as_bytes());
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9b33b1870f543d26da81125b436c5d40)

`str` 非常高效，但如果你想修改它，它也非常难操作 —— 本质上，你需要重新创建一个 `str`。这就是 `String` 出马的地方了。

`String` 是一个 `struct`，修改它非常简单。并且，由于它实现了 `Deref<target=str>` 特性，所以我们可以很方便地使用 `&` 将一个 `String` 转换为 `&str`。

```rust
fn prints_str(str: &str) {
    println!("I only print &strs, and the &str I got is: {str}");
}


fn main() {
    let slice: &str = "Hello!";
    prints_str(slice);

    let mut string: String = String::from("Hello!");
    string.pop();
    string.push_str(", World!");
    prints_str(&string);
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6f5382263c70502f924fbfc6c117bb84)

在上面的例子中：

- `"Hello!"`：一个字面量（`'static str`）
- `String::from("Hello!")`：将字符串字面量拷贝到堆上，返回一个新的 `String`，也可用 `"Hello!".to_string()`、`"Hello!".to_owned()` 等写法

| 名称     | 类型             | 存储位置 | 是否拥有 | 可否修改 | 典型创建方式                              |
| ------ | -------------- | ---- | ---- | ---- | ----------------------------------- |
| 字符串字面量 | `&'static str` | 只读段  | 否    | 否    | `"hello"`                           |
| 字符串切片  | `&str`         | 任意内存 | 否    | 否    | `&s[..]`, `&String`, `&'static str` |
| 堆字符串   | `String`       | 堆    | 是    | 是    | `String::from`, `"x".to_string()`   |

### 向量

在 `String` 底层，使用了向量（`Vec<u8>`）。`u8` 代表无符号 8 位数字，即字节。

```rust
fn main() {
    let mut vec: Vec<usize> = Vec::new();
    println!("The vec: {:?}",vec);
    for num in 0..100 {
        println!();
        println!("Step {}", num);
        vec.push(num);
        println!("The vec: {:?}", vec);
        println!("Its capacity: {}", vec.capacity());
        println!("Its length: {}", vec.len());
        println!("Item number {}: {}", num, vec.get(num).unwrap());
    }
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=29a07bd5c0844be3516c2e5d2c241dab)

- `Vec::new()`：创建一个空向量
- `.capacity()`：展示当前容量，从上面的运行结果可以看到容量从 4 开始，并在容量不足时倍增（4->8->16->32->...）
- `.len()`：当前存储的元素个数
- `.get(index)`：获取索引位置的元素，返回一个 `Option`

了解更多：[Vec in std::vec - Rust](https://doc.rust-lang.org/std/vec/struct.Vec.html)

### 模块

目前的文件结构如下：

```text
hecto/
├── Cargo.toml
└── src/
    ├── main.rs          <- 根 (crate 根)
    ├── editor.rs        <- `editor` 模块的入口
    └── editor/
        └── terminal.rs  <- editor 的子模块
```

> 以前还有一个 `mod.rs` 的写法，就是把 `editor.rs` 重命名为 `mod.rs` 并放入 `editor/` 目录，现已不再推荐。

在 `main.rs` 中，通过 `mod editor;` 导入我们的自定义的 `editor` 模块，然后就可以使用 `use editor::Editor;` 来使用模块中定义的内容。

了解更多：

- [Clear explanation of Rust’s module system](https://www.sheshbabu.com/posts/rust-module-system/)
- [【翻译】关于Rust模块系统的清晰解释 - 知乎](https://zhuanlan.zhihu.com/p/164556350)

### `Option`

这个概念类似于 `Result`，但与 `Result` 的要么正确（`Ok`）、要么错误（`Err`）不同，`Option` 表示的状态都是正常的，不过是存在（`Some`）或者不存在（`None`）。

### `default` 和 `new`

作用上类似于构造器。

- `new` 是约定的工厂函数，编译器不会自动调用。
- `default` 是标准 trait `Default` 的唯一方法，编译器也不会自动调用，只有你显式写 `Buffer::default()` 或 `#[derive(Default)]` 时才会生效。常用于返回当前结构体的合理空值的场景。

```rust
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            lines: vec![String::from("Hello, World!")],
        }
    }
}
```

### 迭代器

`.collect()` 是迭代器上的一个便捷方法，用它可以消费迭代器的所有结果并将结果存储在 `Vec` 中。一个 `Vec` 自身也实现了迭代器相关的特性，它也是可迭代的。

```rust
pub fn main() {
    let range = 1..10;

    println!("First range");
    println!("{:?}", range);
    for n in range {
        println!("{:?}", n); // 输出 1-9
    }

    let second_range = 1..10;
     println!("Second range");
      println!("{:?}", second_range);
    for n in second_range.take(3) {
        println!("{:?}", n); // 输出 1-3（取 3 个）
    }

    let third_range = 1..10;
    println!("Third range");
    println!("{:?}", third_range);
    for n in third_range.skip(2) {
        println!("{:?}", n); // 输出 3-9（跳过开始的 1、2）
    }

    let fourth_range = 1..10;
    let vec: Vec<u8> = fourth_range.skip(2).take(3).collect();
    println!("Vector");
    println!("{:?}", vec);
     for n in vec {
        println!("{:?}", n); // 输出 3、4、5
    }
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1eb6d8b6d6d6e297648434aec58e96c4)

- `.take(n)`：限定只取开头的 n 个迭代项
- `.skip(n)`：跳过开头的 n 个迭代项
- `.collect()`：将所有迭代项存入一个 `Vec` 中

### `panic!`

Rust 使用 `panic!` 来表示它不知道如何处理的错误，默认操作是**展开栈（unwinding the stack）**，这包括一些清理和打印调用栈信息（表现为堆栈追踪 (Stack trace)）。

可以在 `Cargo.toml` 中配置在发生 `panic!` 时立即中止：

```toml
panic = 'abort'
```

这会使最终程序更小，但它可能会留下一些未清理的资源，例如已打开文件或挂起的网络连接。

处理 `panic!` 有以下几种方法：

- **捕获展开（Catching Unwinds）**：你可以在展开操作时通过 `std::panic::catch_unwind` 捕获一个恐慌，从而使程序有可能继续运行
- **创建自定义恐慌处理程序（Panic Handler）**：如果不希望简单地将控制权交还操作系统，那么就需要自定义其行为。下面是一个自定义恐慌处理程序的例子，其中的 `-> !` 表示这个函数永远不会返回：

  ```rust
  #[panic_handler]
  fn panic(info: &PanicInfo) -> ! {
      loop {}
  }
  ```
- **恐慌钩子（Panic Hooks）**：在恐慌展开前，你可以定义一些函数，以实现清理或设置更受控的崩溃等目的

借助 `#[cfg(debug_assertions)]` 和 `debug_assert!`，我们可以让部分检查仅在 Debug 构建中进行，而从生产发布中完全移除。

```rust
fn expensive_check() -> bool {
    println!("Performing expensive check!");
    return true;
}

#[cfg(debug_assertions)]
fn other_expensive_check() -> bool {
    println!("Thoroughly performing some other expensive check!");
    return true;
}


#[cfg(not(debug_assertions))]
fn other_expensive_check() -> bool {
    println!("Only superficially performing some other expensive  check!");
    return true;
}


fn main() {
    println!("Release Checks:");
    assert!(expensive_check());
    assert!(expensive_check(), "Expensive check failed in Release Build!");
    assert_eq!(expensive_check(), true);
    assert_ne!(expensive_check(), false);

    #[cfg(debug_assertions)]
    {
        println!("Debug Checks:");
    }

    debug_assert!(expensive_check());
    debug_assert!(expensive_check(), "Expensive check failed in Debug Build!");
    debug_assert_eq!(expensive_check(), true);
    debug_assert_ne!(expensive_check(), false);

    println!("Checks from conditional functions:");
    assert!(other_expensive_check());

    println!("All checks passed");
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7469cd789a9d25760ce67f10b40c6639)

### 生命周期

下面这段代码无法通过编译，因为 Rust 编译器无法在编译时确定从 `longest` 返回的 `&str` 将存活多久：

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

为解决这个问题，需要显式地定义生命周期：

```rust

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- `'a`：一个通用的生命周期标记
- `'static`：表示某数据在整个程序运行期间都存在
- `'_`：匿名生命周期

在生命周期结束时，Rust 通过 `Drop` 特性中的 `drop()` 函数确定如何清理资源。

需要注意的是，`drop()` 在恐慌时也会被调用，并且如果其中再次引发了恐慌，会形成“双重恐慌”的局面。

### 备用屏幕

终端提供两个缓冲区：主屏幕和备用屏幕。备用屏幕不破坏主屏幕内容，可以临时提供一个干净的“工作区”。

1. 临时全屏应用界面

vim、less、tmux、man、htop 等全屏 CLI 程序启动时切换到 alternate screen，退出时再切回主屏幕，于是用户之前看到的历史输出仍保持原样，不会被程序本身的输出“冲掉”。

2. 避免滚动条污染

主屏幕（Normal Buffer）支持回滚，而 alternate screen 通常禁用了回滚，程序可以随心所欲地重绘整个窗口，不必担心用户滚动到“奇怪”的区域。

3. 简化光标定位

程序可以假设自己从左上角(0,0)开始绘制，省掉对已有内容的计算，实现起来更直观。

我们可以用 crossterm 的 `EnterAlternateScreen` 和 `LeaveAlternateScreen` 来切入和切出备用屏幕。

### 闭包

Rust 中的闭包语法来自于 Ruby：

```rust
fn main() {
    let some_closure = |x| x+1;
    let some_value = some_closure(5);
    dbg!(some_value);
    let some_more_sophisticated_closure = |name| {
        println!("Hey, {}, how are you?", name);
    };
    some_more_sophisticated_closure("Philipp");
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=be2c5802fee314941a2d0d3ee67b74fc)

闭包可以访问闭包外的变量。如果使用的变量实现了 `Copy` 特性，闭包将有一个可用的副本来处理。如果不是，需要使用 `move` 移动语句将相关外部变量移入闭包中。在下面的例子中， `message` 在 `greet` 定义后对 `main` 不可访问：

```rust
fn main() {
    let message = String::from("Hello");
    let greet = move |name| println!("{}, {}!", message, name);
    greet("Philipp"); // that's my name, in case you forgot
}
```

`Box` 类似于 C++ 中的 `unique_ptr`。

代码中的例子：

```rust
// 获取当前的 Panic Hook，默认情况下打印恐慌
let current_hook = std::panic::take_hook();

// 定义一个新的闭包，包含对 PanicInfo 的引用
// 使用 `move` 将任何需要的外部变量移入闭包
// 将闭包放在一个 `Box` 中并将其设置为新的 Panic Hook
std::panic::set_hook(Box::new(move |panic_info| {
    // 这里是我们自定义的的 Panic Hook
    // 执行原来的 Hook 以保留原有的 Panic 输出行为
    current_hook(panic_info);
}));
```

## 其他 Rust 学习资源

### 博客文章

- [2024 年的 Rust 学习指南](https://github.com/pretzelhammer/rust-blog/blob/master/posts/translations/zh-hans/learning-rust-in-2024.md)

### 书籍

- [Rust 程序设计语言](https://rustwiki.org/zh-CN/book/)
- [Rust 语言圣经](https://course.rs/about-book.html)
- [通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/)
- [100 个练习题学习 Rust](https://colobu.com/rust100/)
- [Rust 宏小册](https://zjp-cn.github.io/tlborm/)
- [Rust 秘典（死灵书）](https://nomicon.purewhite.io/vec/vec.html)
- [Comprehensive Rust](https://google.github.io/comprehensive-rust/zh-CN/)
- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)

### 课程

- [CS110L: Safety in Systems Programming (Spring 2020)](https://reberhardt.com/cs110l/spring-2020/)

### 练习平台

- [Rust on Exercism](https://exercism.org/tracks/rust)
