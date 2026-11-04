use std::collections::HashMap;
use super::recap::Ticket;

/// 让我们为工单 id 定义一个类型别名。
/// 这是一种轻量级技术，用于给底层数据类型添加一层语义。
///
/// 底层类型仍然是 `u32`。
/// 下面这段代码仍然有效：
/// ```
/// let number: u32 = 1;
/// let ticket_id: TicketId = number;
/// ```
/// 如果我们想确保不会把工单 id 和具有不同语义的 `u32` 变量混用，
/// 就需要创建一个新类型，例如 `struct TicketId(u32)`。
/// 目前这似乎没有必要——我们身边没有那么多 `u32` 在飞来飞去。
pub type TicketId = u32;

// 欢迎给 `TicketStore` 添加更多字段来解决这个练习！
struct TicketStore {
    data: HashMap<TicketId, Ticket>,
}

impl TicketStore {
    pub fn new() -> TicketStore
    {
        TicketStore {
            data: HashMap::new(),
        }
    }

    /// 到目前为止，我们一直把 `id` 作为 `save` 方法的一个参数。
    ///
    /// 当你传入两个具有相同 id 的不同工单并调用 save 时，会发生什么？
    /// 我们已经用一个测试固定了期望：第二个工单会覆盖第一个工单。
    /// 另一种选择本应是报错。
    ///
    /// 但 JIRA 并不是这样工作的：你不能选择工单的 id，
    /// 它是由系统为你生成的，并且其唯一性有保证。
    /// 还有另一个特点：id 是整数，并且是单调递增的
    /// （看板上的第一个工单会是 `BOARDNAME-1`，第二个是
    /// `BOARDNAME-2`，依此类推）。
    ///
    /// 我们希望我们的克隆版 IronJira 也具有相同的行为。
    /// `TicketStore` 会负责为我们的工单生成 id，并且该 id
    /// 会在插入后由 `save` 返回。
    pub fn save(&mut self, ticket: Ticket) -> TicketId
    {
        let id = self.generate_id();
        self.data.insert(id, ticket);
        id
    }

    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }

    fn generate_id(__) -> TicketId {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::recap::{create_ticket, Status};
    use fake::{Faker, Fake};

    #[test]
    fn a_ticket_with_a_home()
    {
        let ticket = generate_ticket(Status::ToDo);
        let mut store = TicketStore::new();

        let ticket_id = store.save(ticket.clone());

        assert_eq!(store.get(&ticket_id), Some(&ticket));
        assert_eq!(ticket_id, 1);
    }

    #[test]
    fn a_missing_ticket()
    {
        let ticket_store = TicketStore::new();
        let ticket_id = Faker.fake();

        assert_eq!(ticket_store.get(&ticket_id), None);
    }

    #[test]
    fn id_generation_is_monotonic()
    {
        let n_tickets = 100;
        let mut store = TicketStore::new();

        for expected_id in 1..n_tickets {
            let ticket = generate_ticket(Status::ToDo);
            let ticket_id = store.save(ticket);
            assert_eq!(expected_id, ticket_id);
        }
    }

    fn generate_ticket(status: Status) -> Ticket {
        let description = (0..3000).fake();
        let title = (1..50).fake();

        create_ticket(title, description, status)
    }
}
