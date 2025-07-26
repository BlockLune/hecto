# Hecto

一个使用 Rust 编写的简易文本编辑器，相见[教程](https://www.flenker.blog/hecto/)。

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

>  By default your terminal starts in **canonical mode**, also called **cooked mode**. In this mode, keyboard input is only sent to your program when the user presses `Enter`.

### `Result` 和 `match` 语句

在其他编程语言中，函数实际上有两种方法将控制权返回给调用它的代码：

- 返回值
- 异常

在 Rust 中，可以借由 `Result` 这个特殊地类型来直接将可能的异常包裹在返回值中。它有两个变体，`Ok()` 和 `Err()`。

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
- `->`：后跟这个函数的返回类型

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

### Debug 和 Release 模式下对于整型溢出的不同行为

在 Rust 里，整型溢出（arithmetic overflow / underflow）在 Debug 和 Release 两种构建模式下的行为是 有明确区别的，这属于编译器的一种“双态策略”。

- Debug 模式：在每一次可能溢出的算数操作后插入运行时检查，如果发生溢出，立刻 `panic`；
- Release 模式：不包含任何溢出检查，如果发生溢出，直接按二进制补码环绕（wrap-around）处理，例如 `i32:MAX + 1` 得到 `i32:MIN`。

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
