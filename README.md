## todo-rs

[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/liruifengv/todo-rs)

简体中文 | [English](./README.en-US.md)

Rust 开发的简易命令行 TODO LIST。

## 教程文章

- [Rust 实战教程之用 Rust 写一个命令行 TODO List（一）](https://liruifengv.com/posts/write-todo-by-rust/)
- [Rust 实战教程之用 Rust 写一个命令行 TODO List（二）](https://liruifengv.com/posts/write-todo-by-rust-2/)
- 未完待续

你可以在我的博客中找到更多关于 Rust 的文章：[https://liruifengv.com/posts/tags/rust/](https://liruifengv.com/posts/tags/rust/)

也可以关注我的公众号 SayHub，第一时间获取更新：

![](https://bucket.liruifengv.com/qrcode.png)

## 参考资料
- [dvm](https://github.com/justjavac/dvm)
- [Rust 命令行手册](https://rust-cli.github.io/book/index.html)
- [zodo](https://github.com/unixzii/zodo)
## 安装

```bash
cargo install --git https://github.com/liruifengv/todo-rs
```

## 用法

```console
➜  ~  rodo --help
rodo 0.1.0
Rodo is a simple todo list manager.

USAGE:
    rodo.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add       Add a todo item.
    help      Print this message or the help of the given subcommand(s)
    info      Show rodo info.
    list      List all the todo items. [aliases: ls, ll, la]
    remove    Remove a todo item. [aliases: rm]
```

## 开发

```bash
# 克隆这个仓库
git clone

# 本地运行
cargo run

# 打包
cargo build
```

## ISSUE

如果你遇到任何问题，欢迎[联系我](https://github.com/liruifengv/todo-rs/issues)。

## Contributing

你可以随时给这个项目提 PR。

## License

todo-rs 基于 MIT 协议。查看 [LICENSE](./LICENSE) 详情。
