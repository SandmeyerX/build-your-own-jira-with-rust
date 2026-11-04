/// 是时候转移重点了：我们的工单运行得不错，但它们需要一个归宿。
/// 一个可以存储它们、搜索它们、取回它们的地方。
///
/// 我们可以使用许多不同的数据结构来存储和管理工单。
/// JIRA 用户非常依赖工单标识符，例如 RUST-2018 或 COVID-19。
/// 它是一个唯一的标签，可以明确标识单个工单，
/// 通常形式为 `<board name>-<ticket number>`。
/// 我们还没有看板（board）的概念，所以先使用简单的数字 id。
///
/// 允许我们根据 id 获取工单的最简单数据结构是什么？
/// 对我们来说，使用 HashMap 是合理的，它在其他语言中也称为字典。
/// 你可以在这里阅读 Rust 中 HashMap 的更多内容：
/// https://doc.rust-lang.org/std/collections/struct.HashMap.html
use std::collections::HashMap;
/// 让我们导入上一组练习中完成的内容。
use super::recap::Ticket;

/// 首先，我们将创建一个 TicketStore 结构体，其中有一个 `data` 字段，类型为 HashMap。
///
/// HashMap 是一个*泛型*结构体：我们需要指定两个类型，一个用于键，一个用于
/// 存储的值——HashMap<K, V>。
///
/// 让我们把值类型设置成我们的 Ticket，并使用无符号整数作为 id。
struct TicketStore {
    /// 已存储工单的集合。
    data: HashMap<u32, Ticket>,
}

impl TicketStore {
    /// 方法不必把 self 作为参数。
    /// 这等价于其他编程语言中的类方法/静态方法。
    /// 可以通过 `TicketStore::new()` 调用它。
    pub fn new() -> TicketStore {
        TicketStore {
            // 注意，编译器可以推断出我们 HashMap 键值对的类型。
            data: HashMap::new(),
        }
    }

    /// 我们接收 `&mut self`，因为我们将要修改 HashMap，以插入一个新的
    /// 键值对。
    pub fn save(&mut self, ticket: Ticket, id: u32) {
        todo!()
    }

    pub fn get(&self, id: &u32) -> &Ticket {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::super::recap::{create_ticket, Status};
    use super::*;
    use fake::{Fake, Faker};

    /// 现在让我们把 TicketStore 用起来。
    ///
    /// 我们将创建一个工单，把它保存到 TicketStore 中，最后验证
    /// 我们保存到存储中的工单确实就是我们创建的那个工单。
    #[test]
    fn a_ticket_with_a_home() {
        let ticket = generate_ticket(Status::ToDo);

        // 特别注意这里的 'mut' 关键字：在 Rust 中，变量默认是不可变的。
        // `mut` 关键字用于表明你必须特别注意这个变量，因为它在
        // 函数体后面很可能会发生变化。
        let mut store = TicketStore::new();
        let ticket_id = Faker.fake();

        // 这里我们需要创建一个 `ticket` 的克隆，因为 `save` 按值接收 `ticket`
        // 参数，从而把它的值的所有权从调用者函数转移到方法中。
        // 但在这个方法调用之后，我们仍然需要 `ticket` 的值，来验证它是否
        // 与我们取回的内容匹配。
        // 因此需要克隆它，创建值的一个副本，并把该副本传给 `save` 方法。
        //
        // （你可能需要回到 `recap` 练习中，为 Ticket 和 Status 派生更多 trait……）
        store.save(ticket.clone(), ticket_id);

        assert_eq!(store.get(&ticket_id), &ticket);
    }

    /// 我们希望 `get` 方法在查找一个没有关联工单的 id 时 panic
    /// （目前如此）。
    ///
    /// Rust 有一种更优雅地处理这种失败模式的方式，我们稍后会看一看。
    #[test]
    #[should_panic]
    fn a_missing_ticket() {
        let ticket_store = TicketStore::new();
        let ticket_id = Faker.fake();

        ticket_store.get(&ticket_id);
    }

    /// 这不是我们最终版本工单存储所期望的行为，
    /// 但目前这样就可以了。
    #[test]
    fn inserting_a_ticket_with_an_existing_id_overwrites_previous_ticket() {
        let first_ticket = generate_ticket(Status::ToDo);
        let second_ticket = generate_ticket(Status::ToDo);
        let ticket_id = Faker.fake();
        let mut store = TicketStore::new();

        store.save(first_ticket.clone(), ticket_id);
        assert_eq!(store.get(&ticket_id), &first_ticket);

        store.save(second_ticket.clone(), ticket_id);
        assert_eq!(store.get(&ticket_id), &second_ticket);
    }

    fn generate_ticket(status: Status) -> Ticket {
        let description = (0..3000).fake();
        let title = (1..50).fake();

        create_ticket(title, description, status)
    }
}
