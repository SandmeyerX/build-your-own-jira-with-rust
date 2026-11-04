/// 太好了，我们了解了什么是 trait，以及如何实现一个 trait。
/// 不过我相信你也会同意：手动实现 PartialEq 相当繁琐、
/// 重复性很高，计算机肯定能做得更好，而不必麻烦我们！
///
/// Rust 团队也理解你的痛苦，因此提供了一个很方便的特性：派生宏（derive macros）。
/// 派生宏是一种代码生成工具：在代码编译开始之前，派生宏会接收
/// 它所应用到的代码作为输入（以 token 流的形式！），
/// 并有机会按需生成其他代码。
///
/// 例如，这个 `#[derive(PartialEq)]` 会接收我们 enum 的定义作为输入，
/// 并生成一个 PartialEq 实现，它和我们上一个 koan 中手动写出的实现完全等价。
/// 你可以使用 `cargo expand` 查看派生宏生成的代码：
/// https://github.com/dtolnay/cargo-expand
///
/// ```sh
/// cargo install cargo-expand
/// cargo expand -p jira-wip path_to_enlightenment::derive
/// ```
///
/// PartialEq 并不是唯一可以自动派生实现的 trait！
#[derive(PartialEq, Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assertions() {
        // 你的目标是让这个测试编译通过。
        assert_eq!(Status::ToDo, Status::ToDo);
        assert_ne!(Status::Done, Status::ToDo);
        assert_ne!(Status::InProgress, Status::ToDo);
        assert_eq!(Status::InProgress, Status::InProgress);
    }
}
