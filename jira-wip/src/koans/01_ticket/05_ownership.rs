/// 通过模块和可见性修饰符，我们现在已经完全封装了 Ticket 的字段。
/// 已经无法绕过我们的校验来创建 Ticket 实例。
/// 但与此同时，我们也让 struct 的字段变得无法访问，
/// 因为它们是私有的！
///
/// 让我们通过引入一组访问器方法来修复这个问题，这些方法为 ticket 中的字段提供**只读**访问。

/// 让我们导入上一个练习中定义的 Status 枚举，我们不需要修改它。
use super::visibility::ticket::Status;

/// 在这里重新定义 Ticket，因为需要访问私有字段的方法
/// 必须定义在 struct 本身所在的同一模块中，正如我们在上一个练习中看到的那样。
pub struct Ticket {
    title: String,
    description: String,
    status: Status
}

/// struct 上的方法定义在 `impl` 块中。
impl Ticket {
    /// 语法看起来和定义函数非常相似。
    /// 只有一个特殊之处：如果你想在方法中访问该 struct，
    /// 需要在方法签名的第一个参数位置接收 `self`。
    ///
    /// 根据你想达成的目标，有三种选择：
    /// - self
    /// - &self
    /// - &mut self
    ///
    /// 我们现在第一次接触所有权这个话题，它由编译器通过（大名鼎鼎的）
    /// 借用检查器来强制执行。
    ///
    /// 在 Rust 中，每个值都有一个所有者，并在编译期静态确定。
    /// 任意时刻，每个值都只有一个所有者。
    /// 在编译期跟踪所有权，正是 Rust 能够在大多数情况下
    /// 既不需要垃圾回收、也不需要开发者显式管理内存的原因。
    ///
    /// 所有者可以对一个值 `a` 做什么？
    /// 它可以修改它。
    /// 它可以把所有权移动给另一个函数或变量。
    /// 它可以向其他函数或变量借出许多不可变引用（`&a`）。
    /// 它可以向另一个函数或变量借出**唯一一个**可变引用（`&mut a`）。
    ///
    /// 对于一个值的共享不可变引用（`&a`），你能做什么？
    /// 你可以读取该值，并创建更多不可变引用。
    ///
    /// 对于一个值的唯一可变引用（`&mut a`），你能做什么？
    /// 你可以修改底层值。
    ///
    /// 所有权嵌入在类型系统中：每个函数都必须在其签名中声明
    /// 它对其所有参数要求的所有权层级。
    /// 如果调用者无法满足这些要求，就不能调用该函数。
    ///
    /// 在我们的例子中，我们只需要读取 Ticket struct 的一个字段：
    /// 请求对 struct 的不可变引用就足够了。
    ///
    /// 如果这听起来有点复杂/模糊，先别急：随着你继续做练习，
    /// 并在解决一堆编译器错误的过程中前进，它会变得越来越清晰：
    /// 编译器是帮助你熟悉所有权及其规则的最佳结对编程伙伴。
    /// 想了解更多关于所有权的内容，请查看：
    /// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
    pub fn title(&self) -> &String {
        /// 我们返回的是对 title 字段的不可变引用（&）。
        /// 这将允许我们访问该字段，但无法修改它：
        /// 封装得到了保证，我们可以放心，不变式不会被破坏。
        &self.title
    }

    /// 把 __ 替换为合适的类型，为另外两个字段获得访问器方法。
    /// 如果你在想为什么我们返回 &str 而不是 &String，请查看：
    /// https://blog.thoughtram.io/string-vs-str-in-rust/
    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &Status {
       &self.status
    }
}

pub fn create_ticket(title: String, description: String, status: Status) -> Ticket {
    if title.is_empty() {
        panic!("Title cannot be empty!");
    }
    if title.len() > 50 {
        panic!("A title cannot be longer than 50 characters!");
    }
    if description.len() > 3000 {
        panic!("A description cannot be longer than 3000 characters!");
    }

    Ticket {
        title,
        description,
        status,
    }
}

#[cfg(test)]
mod tests {
    use super::{create_ticket, Ticket};
    use super::super::visibility::ticket::Status;

    fn verify_without_tampering() {
        let ticket: Ticket = create_ticket("A title".into(), "A description".into(), Status::ToDo);

        /// 我们不再访问字段 `ticket.description`，而是调用访问器方法
        /// `ticket.description()`，它返回字段值的引用，
        /// 让我们能够验证其值，同时没有机会修改它。
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.title(), "A title");
    }
}
