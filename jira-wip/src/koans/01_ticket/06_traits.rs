use crate::path_to_enlightenment::visibility::ticket::Status;

/// 你可能已经注意到，在上一个 koan 的测试中，我们没有检查
/// `.status()` 返回的状态是否与传给 `create_ticket` 的状态一致。
///
/// 这是因为 `assert_eq!(ticket.status(), Status::ToDo)` 会编译失败：
///
/// error[E0369]: binary operation `==` cannot be applied to type `&path_to_enlightenment::visibility::ticket::Status`
///    --> jira-wip/src/koans/01_ticket/05_ownership.rs:128:13
///     |
/// 128 |             assert_eq!(ticket.status(), Status::ToDo);
///     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///     |             |
///     |             &path_to_enlightenment::visibility::ticket::Status
///     |             path_to_enlightenment::visibility::ticket::Status
///     |
///     = note: an implementation of `std::cmp::PartialEq` might be missing for `&path_to_enlightenment::visibility::ticket::Status`
///
/// `assert_eq` 要求它的参数实现 `PartialEq` trait。
/// 什么是 trait？
/// Rust 中的 trait 与其他编程语言中的接口非常相似：
/// trait 描述一种行为/能力。
/// 例如：
///
/// ```
/// pub trait Pay {
///     fn pay(self, amount: u64, currency: String) -> u64
/// }
/// ```
///
/// 实际上，trait 定义了一组方法的签名。
/// 要实现一个 trait，struct 或 enum 必须在 `impl Trait` 块中实现这些方法：
///
/// ```
/// impl Pay for TaxPayer {
///     fn pay(self, amount: u64, currency: String) -> u64 {
///         todo!()
///     }
/// }
/// ```
///
/// `PartialEq` 就是支撑 == 运算符的 trait。
/// 它的定义大致如下（简化版）：
/// ```
/// pub trait PartialEq {
///     fn eq(&self, other: &Self) -> bool
/// }
/// ```
/// 实际定义会稍微复杂一些，带有泛型参数，以允许比较不同的类型。
/// 不过现在我们先使用这个简化版本。
///
/// 让我们为 Status 实现它！
impl PartialEq for Status {
    fn eq(&self, other: &Status) -> bool {
        // 如果你需要复习 `match` 语法，请查看
        // https://doc.rust-lang.org/book/ch06-02-match.html
        match (self, other) {
            __
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        // 你的目标是让这个测试编译通过。
        assert_eq!(Status::ToDo == Status::ToDo, true);
        assert_eq!(Status::Done == Status::ToDo, false);
        assert_eq!(Status::InProgress == Status::ToDo, false);
        assert_eq!(Status::InProgress == Status::InProgress, true);
    }
}
