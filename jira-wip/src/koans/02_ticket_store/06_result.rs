use super::id_generation::TicketId;
use super::recap::Status;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::error::Error;

/// 我们代码的结构进展得相当不错：它看起来和感觉上都像地道的 Rust，
/// 并且恰当地建模了我们正在处理的领域——JIRA。
///
/// 不过，仍有一些地方可以改进：创建新草稿时的验证逻辑。
/// 我们之前的函数 `create_ticket_draft` 在标题或描述未通过验证检查时会 panic。
/// 调用者完全不知道这可能会发生——函数签名看起来相当无害：
/// ```
/// pub fn create_ticket_draft(title: String, description: String, status: Status) -> TicketDraft
/// ```
/// 调用者通常不会“捕获” panic：它们旨在用于程序无法恢复的状态。
///
/// 对于预期内的错误场景，我们可以使用 `Result` 做得更好：
/// ```
/// pub fn create_ticket_draft(title: String, description: String, status: Status) -> Result<TicketDraft, ValidationError>
/// ```
/// `Result` 是标准库中定义的枚举，就像 `Option` 一样。
/// `Option` 编码的是某些数据可能缺失的可能性，而 `Result`
/// 编码的是操作可能失败这一想法。
///
/// 它的定义大致如下：
/// ```
/// pub enum Result<T, E> {
///     Ok(T),
///     Err(E)
/// }
/// ```
/// `Ok` 变体用于在函数执行成功时返回其结果。
/// `Err` 变体用于返回描述出错原因的错误。
///
/// 错误类型 `E` 必须实现标准库中的 `Error` trait。
/// 让我们归档旧的 `create_ticket_draft` 函数，并定义一个新的
/// `TicketDraft::new` 方法返回 `Result`，以便更好地向调用者设定预期。
#[derive(Debug, Clone, PartialEq)]
pub struct TicketDraft {
    title: String,
    description: String,
}

impl TicketDraft {
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn new(title: String, description: String) -> Result<TicketDraft, ValidationError> {
        if title.is_empty() {
            return Err(ValidationError("Title cannot be empty!".to_string()));
        }
        if title.len() > 50 {
            todo!()
        }
        if description.len() > 3000 {
            todo!()
        }

        let draft = TicketDraft { title, description };
        Ok(draft)
    }
}

/// 我们的错误结构体，在验证失败时返回。
/// 它是对字符串的包装，即验证错误消息。
/// 没有字段名的结构体称为元组结构体，你可以在 Rust 书中阅读更多内容：
/// https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
#[derive(PartialEq, Debug, Clone)]
pub struct ValidationError(String);

/// 要在 `Result` 中把 `ValidationError` 用作 `Err` 变体，我们需要实现
/// `Error` trait。
///
/// `Error` trait 要求我们的结构体实现 `Debug` 和 `Display` trait，
/// 因为错误可能会一路向上传播，直到展示给最终用户。
/// 我们可以派生 `Debug`，但 `Display` 必须显式实现：
/// `Display` 规定了你的结构体在面向用户的输入中如何打印，因此它不能
/// 自动派生。
impl Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    id: TicketId,
    title: String,
    description: String,
    status: Status,
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
        let ticket = Ticket {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
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

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn title_cannot_be_empty() {
        let description = (0..3000).fake();

        let result = TicketDraft::new("".into(), description);
        assert!(result.is_err())
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let description = (0..3000).fake();
        // 让我们生成一个长度超过 51 个字符的标题。
        let title = (51..10_000).fake();

        let result = TicketDraft::new(title, description);
        assert!(result.is_err())
    }

    #[test]
    fn description_cannot_be_longer_than_3000_chars() {
        let description = (3001..10_000).fake();
        let title = (1..50).fake();

        let result = TicketDraft::new(title, description);
        assert!(result.is_err())
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

        TicketDraft::new(title, description).expect("Failed to create ticket")
    }
}
