use super::id_generation::TicketId;
use super::recap::Status;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 我们知道，在工单被保存之前，id 和创建时间永远不会存在；
/// 而在调用 `save` 之后，它们则总会被填充。
///
/// 我们在上一个练习中采用的方法有其局限性：每次访问 `id` 和 `created_at` 时，
/// 我们都需要跟踪工单所处的“生命周期阶段”。
/// 它已经被保存了吗？对这些 `Option` 进行 unwrap 安全吗？
/// 这是不必要的认知负担，并且会在后续编写新代码或重构现有功能时导致错误。
///
/// 我们可以做得更好。
/// 我们可以使用类型来更好地建模我们的领域，并约束代码的行为。
///
/// 在调用 `TicketStore::save` 之前，我们处理的是 `TicketDraft`。
/// 没有 `created_at`，没有 `id`，没有 `status`。
/// 另一方面，`TicketStore::get` 会返回一个 `Ticket`，其中包含 `created_at` 和
/// `id`。
///
/// 不经过存储就无法创建 `Ticket`：
/// 我们将强制 `save` 成为从 `TicketDraft` 生成 `Ticket` 的唯一方式。
/// 这也会确保所有工单都从 `ToDo` 状态开始。
///
/// 更少出错空间，更少歧义，你可以通过查看代码中函数的签名
/// 来理解领域约束。
///
/// 关于类型驱动开发这一主题，可以查看：
/// - https://fsharpforfunandprofit.com/series/designing-with-types.html
/// - https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/
/// - https://www.youtube.com/watch?v=PLFl95c-IiU
///
#[derive(Debug, Clone, PartialEq)]
pub struct TicketDraft {
    title: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
    id: TicketId,
    created_at: DateTime<Utc>,
}

struct TicketStore {
    data: HashMap<TicketId, Ticket>,
    current_id: TicketId,
}

impl TicketStore {
    pub fn new() -> TicketStore {
        TicketStore {
            data: HashMap::new(),
            current_id: 0,
        }
    }

    pub fn save(&mut self, draft: TicketDraft) -> TicketId {
        let id = self.generate_id();

        // 我们可以在这里使用 `Ticket` 的“原始”构造函数，因为
        // 存储与 `Ticket` 定义在同一个模块中。
        // 如果你从另一个模块导入 `Ticket`，
        // `TicketStore::get` 确实会是拿到
        // `Ticket` 实例的唯一方式。
        // 这强化了我们期望的不变量：把草稿保存到存储中
        // 是“创建” `Ticket` 的唯一方式。
        let ticket = Ticket {
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
            id,
            created_at: Utc::now(),
        };
        self.data.insert(id, ticket);
        id
    }

    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }

    fn generate_id(&mut self) -> TicketId {
        self.current_id += 1;
        self.current_id
    }
}

impl TicketDraft {
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn description(&self) -> &String {
        &self.description
    }
}

impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn id(&self) -> &TicketId {
        &self.id
    }
}

pub fn create_ticket_draft(title: String, description: String) -> TicketDraft {
    if title.is_empty() {
        panic!("Title cannot be empty!");
    }
    if title.len() > 50 {
        panic!("A title cannot be longer than 50 characters!");
    }
    if description.len() > 3000 {
        panic!("A description cannot be longer than 3000 characters!");
    }

    TicketDraft { title, description }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn a_ticket_with_a_home() {
        let draft = generate_ticket_draft();
        let mut store = TicketStore::new();

        let ticket_id = store.save(draft.clone());
        let retrieved_ticket = store.get(&ticket_id).unwrap();

        assert_eq!(&ticket_id, retrieved_ticket.id());
        assert_eq!(&draft.title, retrieved_ticket.title());
        assert_eq!(&draft.description, retrieved_ticket.description());
        assert_eq!(&Status::ToDo, retrieved_ticket.status());
    }

    #[test]
    fn a_missing_ticket() {
        let ticket_store = TicketStore::new();
        let ticket_id = Faker.fake();

        assert_eq!(ticket_store.get(&ticket_id), None);
    }

    #[test]
    fn id_generation_is_monotonic() {
        let n_tickets = 100;
        let mut store = TicketStore::new();

        for expected_id in 1..n_tickets {
            let draft = generate_ticket_draft();
            let ticket_id = store.save(draft);
            assert_eq!(expected_id, ticket_id);
        }
    }

    fn generate_ticket_draft() -> TicketDraft {
        let description = (0..3000).fake();
        let title = (1..50).fake();

        create_ticket_draft(title, description)
    }
}
