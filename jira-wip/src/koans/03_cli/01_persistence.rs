//! 使用 `cargo run --bin jira-wip` 把玩 CLI 时，你可能已经注意到
//! 要真正演练我们实现的全部功能其实相当棘手：
//! 每次执行都会重新创建存储，什么都没有被持久化！
//!
//! 是时候解决这个问题了：我们希望在多次 CLI 调用之间把存储持久化到磁盘，
//! 并在执行下一条命令之前重新加载它。
//!
//! 我们将依赖 `serde` crate（`Ser`ialisation/`De`serialisation，即序列化/反序列化）：
//! 只要你的结构体或枚举实现了 serde 的 `Serialize` trait，
//! 它就可以把数据序列化成许多不同的文件格式。
//! 而反方向的旅程则需要 `Deserialize`。
//!
//! 你不需要手动实现这些：只需在 `store_recap` 中需要的地方加上
//! `#[derive(Serialize, Deserialize)]`——之后 `load` 和 `save` 函数就应该能直接工作！
//!
//! 随后适当地更新 `src/main.rs`，用上你的劳动成果！
use super::store_recap::TicketStore;
use std::fs::read_to_string;
use std::path::Path;

/// 如果存在配置文件，则从中获取认证参数。
pub fn load(path: &Path) -> TicketStore {
    println!("Reading data from {:?}", path);
    // 将数据读入内存，并把值存到一个字符串中
    match read_to_string(path) {
        Ok(data) => {
            // 从 YAML 格式反序列化配置
            serde_yaml::from_str(&data).expect("Failed to parse serialised data.")
        }
        Err(e) => match e.kind() {
            // 文件不存在——这是你第一次使用 IronJira！
            std::io::ErrorKind::NotFound => {
                // 返回默认配置
                TicketStore::new()
            }
            // 出了点问题——带着错误信息让 CLI 崩溃。
            _ => panic!("Failed to read data."),
        },
    }
}

/// 将工单保存到磁盘上的正确文件中。
pub fn save(ticket_store: &TicketStore, path: &Path) {
    // 将数据序列化为 YAML 格式
    let content = serde_yaml::to_string(ticket_store).expect("Failed to serialize tickets");
    println!("Saving tickets to {:?}", path);
    // 保存到磁盘
    std::fs::write(path, content).expect("Failed to write tickets to disk.")
}

#[cfg(test)]
mod tests {
    use super::super::store_recap::{
        Status, TicketDescription, TicketDraft, TicketStore, TicketTitle,
    };
    use super::*;
    use fake::Fake;
    use tempfile::NamedTempFile;

    #[test]
    fn load_what_you_save() {
        let mut store = TicketStore::new();
        let draft = generate_ticket_draft();
        store.save(draft);

        // 我们使用 `tempfile` crate 即时生成一个临时路径，
        // 它会在测试结束时被清理。
        // 更多细节见 https://docs.rs/tempfile/3.1.0/tempfile/。
        let temp_path = NamedTempFile::new().unwrap().into_temp_path();

        save(&store, temp_path.as_ref());
        let loaded_store = load(temp_path.as_ref());

        assert_eq!(store, loaded_store);
    }

    fn generate_ticket_draft() -> TicketDraft {
        let description = TicketDescription::new((0..3000).fake()).unwrap();
        let title = TicketTitle::new((1..50).fake()).unwrap();

        TicketDraft { title, description }
    }
}
