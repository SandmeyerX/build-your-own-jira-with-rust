//! 向用户暴露我们所构建功能的方式有很多：API、GUI 等等。
//! 我们会选择一种更简单、但足以用来检验我们的实现，
//! 并亲手触摸劳动成果的方式：一个命令行应用，也就是 CLI。
//!
//! Rust 很适合编写 CLI：我们将使用 `structopt`，这个 crate
//! 提供了一个 derive 宏，可以以声明式的方式定义 CLI 接口。
//!
//! 我们定义命令的结构，并适当地标注每个字段，
//! `#[derive(structopt::StructOpt)]` 会负责生成解析用户输入所需的全部代码，
//! 同时为 CLI 本身及其每个子命令生成详细的 `--help` 页面。
//!
//! 每个字段以及每个 `Command` 变体上的注释都会显示在对应命令的帮助页面中！
//!
//! 你可以在它们的文档中进一步了解 `structopt`：
//! https://docs.rs/structopt/0.3.12/structopt/
//!
//! 你可以使用 `cargo expand` 查看 `structopt` 生成的代码：
//! https://github.com/dtolnay/cargo-expand
//!
//! 补全缺失的字段！
//!
//! 当你准备好后，取消 src/main.rs 中相应行的注释，并在终端中
//! 运行 `cargo run --bin jira-wip`！
use super::store_recap::{TicketStore, Status, TicketDraft, TicketPatch, TicketTitle, TicketDescription};
use super::id_generation::TicketId;
use std::error::Error;
use std::str::FromStr;
use std::fmt::Formatter;

#[derive(structopt::StructOpt, Clone)]
/// 一个用于与玩具版 Jira 克隆 IronJira 交互的小型命令行接口。
pub enum Command {
    /// 在你的看板上创建一个工单。
    Create {
        __
    },
    /// 编辑现有工单的详细信息。
    Edit {
        /// 你想编辑的工单 id。
        #[structopt(long)]
        id: TicketId,
        /// 工单的新状态。
        #[structopt(long)]
        status: Option<Status>,
        /// 工单的新描述。
        #[structopt(long)]
        description: Option<TicketDescription>,
        /// 你工单的新标题。
        #[structopt(long)]
        title: Option<TicketTitle>,
    },
    /// 通过工单 id 从存储中删除一个工单。
    Delete {
        __
    },
    /// 列出所有现有工单。
    List,
}

/// `structopt` 依赖 `FromStr` 来了解如何从用户传入的输入字符串中
/// 解析我们的自定义结构体和枚举。
///
/// 解析可能会失败：我们需要声明出错时返回的错误类型，
/// 并实现 `from_str` 函数。
impl FromStr for Status {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        __
    }
}

impl FromStr for TicketTitle {
    __
}

impl FromStr for TicketDescription {
    __
}

/// 用于解析失败的错误结构体。
#[derive(Debug)]
pub struct ParsingError(String);

impl Error for ParsingError { }

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

/// 核心函数：给定一个 `TicketStore` 的可变引用和一个 `Command`，
/// 执行用户指定的操作。
/// 我们使用 `Box<dyn Error>` 来避免必须指定顶层处理函数的具体失败模式。
///
/// `dyn Error` 是 trait 对象的语法，这是一个更高级的主题，本工作坊不会涉及。
/// 如果你好奇，可以查看 Rust 书中的相关章节：
/// https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
pub fn handle_command(ticket_store: &mut TicketStore, command: Command) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Create { description, title } => {
            todo!()
        }
        Command::Edit {
            id,
            title,
            description,
            status,
        } => {
            todo!()
        }
        Command::Delete { ticket_id } => match ticket_store.delete(&ticket_id) {
            Some(deleted_ticket) => println!(
                "The following ticket has been deleted:\n{:?}",
                deleted_ticket
            ),
            None => println!(
                "There was no ticket associated to the ticket id {:?}",
                ticket_id
            ),
        },
        Command::List => {
            todo!()
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_status_fails_to_be_parsed()
    {
        let invalid_status = "Not a good status";
        assert!(Status::from_str(invalid_status).is_err());
    }
}
