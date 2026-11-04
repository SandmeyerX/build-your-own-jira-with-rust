/// 你可能已经注意到，我们 koan 测试开头的 `mod XX`。
/// `mod` 代表 module（模块）：它是 Rust 提供给你组织代码的工具之一。
/// 特别地，模块会影响你的 struct、enum 和函数的可见性。
///
/// 我们想用这个 koan 来探索模块会对项目结构产生什么影响，以及如何利用它们来强制实现封装。
///
/// 你可以在 Rust 书中了解更多关于模块和可见性的内容：
/// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
pub mod ticket {
    /// 在模块中定义的 struct、enum 和函数，对该模块内所有其他 struct、
    /// enum 和函数都是可见的——例如，我们可以在 `create_ticket` 的签名中
    /// 使用 `Ticket` 作为返回类型。
    ///
    /// 但一旦离开它们所定义的模块，情况就不再如此：Rust 中所有实体
    /// 默认都是私有的，除非它们带有 `pub` 前缀。
    ///
    /// 这一点同样适用于 struct 的字段。
    /// 与某个 struct 定义在同一模块中的函数，可以访问该 struct 的所有字段
    /// （例如 `create_ticket` 可以通过指定字段来创建 `Ticket`）。
    /// 在模块之外，这些字段不可访问，因为默认情况下它们被视为私有，
    /// 除非带有 `pub` 前缀。
    enum Status {
        ToDo,
        InProgress,
        Blocked,
        Done,
    }

    struct Ticket {
        title: String,
        description: String,
        status: Status,
    }

    fn create_ticket(title: String, description: String, status: Status) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty!");
        }
        if title.len() > 50 {
            panic!("A title cannot be longer than 50 characters!");
        }
        if description.len() > 3000 {
            panic!("A description cannot be longer than 3000 characters!");
        }

        // 函数会隐式返回其最后一个表达式的结果，所以这里可以省略
        // `return` 关键字。
        Ticket {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    /// 在上面的代码中添加必要的 `pub` 修饰符，以避免编译器
    /// 对这个 use 语句报错。
    use super::ticket::{create_ticket, Status, Ticket};

    /// 不过要小心！我们不希望这个函数在你修改可见性、让 use 语句能够编译之后仍然能编译通过！
    /// 一旦你确认它确实无法编译，就把它注释掉。
    fn should_not_be_possible() {
        let ticket: Ticket =
            create_ticket("A title".into(), "A description".into(), Status::ToDo);

        // 尝试运行这个 koan 时，你应该会看到这个错误：
        //
        // error[E0616]: field `description` of struct `path_to_enlightenment::visibility::ticket::Ticket` is private
        // --> jira-wip/src/koans/01_ticket/04_visibility.rs:81:24
        //    |
        // 81 |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // 一旦你确认下面这行无法编译，
        // 就把它注释掉，以便继续下一个 koan！
        assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // 这同样应该是不可能的，并且会遇到与上面类似的错误。
        // （只有在你把前一个测试中有问题的行注释掉之后，它才会在下一个编译阶段
        // 抛出编译错误！）
        //
        // 这证明 `create_ticket` 现在是获得 `Ticket` 实例的唯一方式。
        // 不可能创建一个标题或描述非法的 ticket！
        //
        // 一旦你确认下面这些代码无法编译，
        // 就把它们注释掉，以便继续下一个 koan！
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: Status::ToDo,
        };
    }
}
