use super::id_generation::TicketId;
use super::recap::Status;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::error::Error;

/// 现在只剩下两块拼图了：删除工单，以及在 `TicketStore` 中更新工单。
/// 更新功能将让我们能够修改工单的 `status`，这是我们 JIRA 克隆版的圣杯。
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
        let timestamp = Utc::now();
        let ticket = Ticket {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
            created_at: timestamp.clone(),
            // 一个新字段，用于跟踪工单最后一次被触碰的时间。
            // 它初始时与 `created_at` 同步，在工单被更新时更新。
            updated_at: timestamp,
        };
        self.data.insert(id, ticket);
        id
    }

    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }

    pub fn list(&self) -> Vec<&Ticket> {
        self.data.values().collect()
    }

    /// 我们接收一个 `id` 和一个 `patch` 结构体：这让我们能够约束 `Ticket` 中
    /// 哪些字段实际上可以被更新。
    /// 例如，我们不希望用户能够更新 `id` 或
    /// `created_at` 字段。
    ///
    /// 如果我们选择了不同的策略，例如实现一个 `get_mut` 方法
    /// 来取回工单的可变引用，并让调用者可以随意编辑它，
    /// 我们就无法维持同样的保证。
    ///
    /// 如果传入的 `id` 与存储中的某个工单匹配，我们返回编辑后的工单。
    /// 如果不匹配，我们返回 `None`。
    pub fn update(&mut self, id: &TicketId, patch: TicketPatch) -> Option<&Ticket> {
        todo!()
    }

    /// 如果传入的 `id` 与存储中的某个工单匹配，我们返回被删除的工单
    /// 以及一些额外的元数据。
    /// 如果不匹配，我们返回 `None`。
    pub fn delete(&mut self, id: &TicketId) -> Option<DeletedTicket> {
        todo!()
    }

    fn generate_id(&mut self) -> TicketId {
        self.current_id += 1;
        self.current_id
    }
}

/// 我们不想放宽对工单可接受的标题或可接受的
/// 描述的约束。
/// 这意味着我们需要使用与 `TicketDraft` 相同的规则
/// 来验证 `TicketPatch` 中的 `title` 和 `description`。
///
/// 为了保持 DRY（不重复自己），我们引入两个新类型，其构造函数保证我们
/// 关心的不变量。
#[derive(Debug, Clone, PartialEq)]
pub struct TicketTitle(String);

impl TicketTitle {
    pub fn new(title: String) -> Result<Self, ValidationError> {
        if title.is_empty() {
            return Err(ValidationError("Title cannot be empty!".to_string()));
        }
        if title.len() > 50 {
            return Err(ValidationError(
                "A title cannot be longer than 50 characters!".to_string(),
            ));
        }
        Ok(Self(title))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketDescription(String);

impl TicketDescription {
    pub fn new(description: String) -> Result<Self, ValidationError> {
        if description.len() > 3000 {
            Err(ValidationError(
                "A description cannot be longer than 3000 characters!".to_string(),
            ))
        } else {
            Ok(Self(description))
        }
    }
}

/// `TicketPatch` 约束了我们认为是可编辑的字段。
///
/// 如果某个字段被设置为 `Some`，它的值将被更新为指定的值。
/// 如果某个字段被设置为 `None`，该字段保持不变。
#[derive(Debug, Clone, PartialEq)]
pub struct TicketPatch {
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>,
}

/// 由于验证已经内建在我们的类型中，我们不再需要担心这些字段的可见性。
/// 我们的 `TicketPatch` 和 `TicketDraft` 没有身份标识，没有 id，不像保存在存储中的
/// `Ticket`。
/// 借用领域驱动设计（Domain Driven Design）的术语来说，它们是值对象，而不是实体。
///
/// 只要我们已知我们的不变量得到维持，我们就可以让用户随意修改它们。
/// 因此我们可以去掉构造函数和所有访问器方法。相当不错吧？
#[derive(Debug, Clone, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

/// 一个围绕被删除工单的轻量包装，用于存储一些元数据（删除时间戳）。
/// 如果我们有用户系统，我们还会存储
/// 执行删除操作的用户身份。
#[derive(Debug, Clone, PartialEq)]
pub struct DeletedTicket {
    ticket: Ticket,
    deleted_at: DateTime<Utc>,
}

impl DeletedTicket {
    pub fn ticket(&self) -> &Ticket {
        &self.ticket
    }
    pub fn deleted_at(&self) -> &DateTime<Utc> {
        &self.deleted_at
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ValidationError(String);

impl Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    id: TicketId,
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Ticket {
    pub fn title(&self) -> &TicketTitle {
        &self.title
    }
    pub fn description(&self) -> &TicketDescription {
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
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};
    use std::time::Duration;

    #[test]
    fn updating_nothing_leaves_the_updatable_fields_unchanged() {
        let mut store = TicketStore::new();
        let draft = generate_ticket_draft();
        let ticket_id = store.save(draft.clone());

        let patch = TicketPatch {
            title: None,
            description: None,
            status: None,
        };
        let updated_ticket = store.update(&ticket_id, patch).unwrap();

        assert_eq!(draft.title, updated_ticket.title);
        assert_eq!(draft.description, updated_ticket.description);
        assert_eq!(Status::ToDo, updated_ticket.status);
    }

    #[test]
    fn trying_to_update_a_missing_ticket_returns_none() {
        let mut store = TicketStore::new();
        let ticket_id = Faker.fake();
        let patch = generate_ticket_patch(Status::Done);

        assert_eq!(store.update(&ticket_id, patch), None);
    }

    #[test]
    fn update_works() {
        let mut store = TicketStore::new();
        let draft = generate_ticket_draft();
        let patch = generate_ticket_patch(Status::Done);
        let ticket_id = store.save(draft.clone());

        // 让我们等一会儿，否则 `created_at` 和 `updated_at`
        // 可能会变得相同 (ᴗ˳ᴗ)
        std::thread::sleep(Duration::from_millis(100));
        let updated_ticket = store.update(&ticket_id, patch.clone()).unwrap();

        assert_eq!(patch.title.unwrap(), updated_ticket.title);
        assert_eq!(patch.description.unwrap(), updated_ticket.description);
        assert_eq!(patch.status.unwrap(), updated_ticket.status);
        assert_ne!(updated_ticket.created_at(), updated_ticket.updated_at());
    }

    #[test]
    fn delete_works() {
        let mut store = TicketStore::new();
        let draft = generate_ticket_draft();
        let ticket_id = store.save(draft.clone());
        let ticket = store.get(&ticket_id).unwrap().to_owned();

        let deleted_ticket = store.delete(&ticket_id).unwrap();

        assert_eq!(deleted_ticket.ticket(), &ticket);
        assert_eq!(store.get(&ticket_id), None);
    }

    #[test]
    fn deleting_a_missing_ticket_returns_none() {
        let mut store = TicketStore::new();
        let ticket_id = Faker.fake();

        assert_eq!(store.delete(&ticket_id), None);
    }

    #[test]
    fn list_returns_all_tickets() {
        let n_tickets = 100;
        let mut store = TicketStore::new();

        for _ in 0..n_tickets {
            let draft = generate_ticket_draft();
            store.save(draft);
        }

        assert_eq!(n_tickets, store.list().len());
    }

    #[test]
    fn on_a_single_ticket_list_and_get_agree() {
        let mut store = TicketStore::new();

        let draft = generate_ticket_draft();
        let id = store.save(draft);

        assert_eq!(vec![store.get(&id).unwrap()], store.list());
    }

    #[test]
    fn list_returns_an_empty_vec_on_an_empty_store() {
        let store = TicketStore::new();

        assert!(store.list().is_empty());
    }

    #[test]
    fn title_cannot_be_empty() {
        assert!(TicketTitle::new("".into()).is_err())
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        // 让我们生成一个长度超过 51 个字符的标题。
        let title = (51..10_000).fake();

        assert!(TicketTitle::new(title).is_err())
    }

    #[test]
    fn description_cannot_be_longer_than_3000_chars() {
        let description = (3001..10_000).fake();

        assert!(TicketDescription::new(description).is_err())
    }

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
        assert_eq!(retrieved_ticket.created_at(), retrieved_ticket.updated_at());
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

    #[test]
    fn ids_are_not_reused() {
        let n_tickets = 100;
        let mut store = TicketStore::new();

        for expected_id in 1..n_tickets {
            let draft = generate_ticket_draft();
            let ticket_id = store.save(draft);
            assert_eq!(expected_id, ticket_id);
            assert!(store.delete(&ticket_id).is_some());
        }
    }

    fn generate_ticket_draft() -> TicketDraft {
        let description = TicketDescription::new((0..3000).fake()).unwrap();
        let title = TicketTitle::new((1..50).fake()).unwrap();

        TicketDraft { title, description }
    }

    fn generate_ticket_patch(status: Status) -> TicketPatch {
        let patch = generate_ticket_draft();

        TicketPatch {
            title: Some(patch.title),
            description: Some(patch.description),
            status: Some(status),
        }
    }
}
