## todo-rs

[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/liruifengv/todo-rs)

简体中文 | [English](./README.en-US.md)

Rust 开发的简易命令行 TODO LIST。

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
