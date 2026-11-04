use super::recap::Ticket;
use std::collections::HashMap;

struct TicketStore {
    data: HashMap<u32, Ticket>,
}

impl TicketStore {
    pub fn new() -> TicketStore {
        TicketStore {
            data: HashMap::new(),
        }
    }

    pub fn save(&mut self, ticket: Ticket, id: u32) {
        self.data.insert(id, ticket);
    }

    /// 在上一练习中尝试实现 `get` 时，你可能会遇到一些问题，
    /// 原因是签名不匹配：HashMap 上的 `get` 返回的是 `Option<&Ticket>`，
    /// 而不是 `&Ticket`。
    ///
    /// 什么是 Option？
    ///
    /// 简而言之，Rust 没有 `null`：如果一个函数返回 `Ticket`，
    /// 那么这个 `Ticket` 就不可能不存在。
    /// 如果函数确实有可能无法返回 `Ticket`，
    /// 我们就需要在返回类型中表达这一点。
    /// 这正是 `Option` 的用武之地（`Option` 取的是“可选”之意，至少我是这么理解的）。
    /// `Option` 是一个枚举：
    ///
    /// ```
    /// enum Option<T> {
    ///     Some(T),
    ///     None
    /// }
    /// ```
    /// 这里的 `T` 是一个泛型类型参数：正如我们在 HashMap 中看到的那样，Rust 允许你
    /// 对容器中的类型进行泛型化。
    /// `None` 变体表示值缺失。
    /// 而 `Some` 变体则告诉你，你有一个值。
    ///
    /// 如果不先检查变体，你就无法使用 `Option` 中的值，
    /// 因此写代码时不可能“忘记”处理 `None`。
    /// 编译器会强制你同时处理顺利和不顺利的情况。
    ///
    /// 关于 `Option` 的更多细节，Rust 书中有一篇详尽的介绍：
    /// https://doc.rust-lang.org/1.29.0/book/2018-edition/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values
    pub fn get(&self, id: &u32) -> Option<&Ticket> {
        self.data.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::super::recap::{create_ticket, Status};
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn a_ticket_with_a_home() {
        let ticket = generate_ticket(Status::ToDo);
        let mut store = TicketStore::new();
        let ticket_id = Faker.fake();

        store.save(ticket.clone(), ticket_id);

        // 注意，即使存储中确实存在具有指定 id 的工单，
        // 它也会作为 `Option<&Ticket>` 的 `Some` 变体返回。
        assert_eq!(store.get(&ticket_id), Some(&ticket));
    }

    /// 现在，我们希望 `get` 方法在查找没有关联工单的 id 时返回 `None`，
    /// 而不是 panic。
    #[test]
    fn a_missing_ticket() {
        let ticket_store = TicketStore::new();
        let ticket_id = Faker.fake();

        assert_eq!(ticket_store.get(&ticket_id), None);
    }

    #[test]
    fn inserting_a_ticket_with_an_existing_id_overwrites_previous_ticket() {
        let first_ticket = generate_ticket(Status::ToDo);
        let second_ticket = generate_ticket(Status::ToDo);
        let mut store = TicketStore::new();
        let ticket_id = Faker.fake();

        store.save(first_ticket.clone(), ticket_id);
        assert_eq!(store.get(&ticket_id), Some(&first_ticket));

        store.save(second_ticket.clone(), ticket_id);
        assert_eq!(store.get(&ticket_id), Some(&second_ticket));
    }

    fn generate_ticket(status: Status) -> Ticket {
        let description = (0..3000).fake();
        let title = (1..50).fake();

        create_ticket(title, description, status)
    }
}
