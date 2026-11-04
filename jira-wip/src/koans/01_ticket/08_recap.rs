/// 我们已经走了很长一段路：从如何定义 struct 到 trait 和派生宏，
/// 还涉及了测试、模块系统、可见性、所有权和方法语法。
/// 深呼吸一下，伸展一下身体，回顾一下我们做过的事情。
///
/// 然后准备好进入下一节！

#[derive(PartialEq, Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

pub struct Ticket {
    title: String,
    description: String,
    status: Status,
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
    #[test]
    fn the_next_step_of_your_journey() {
        let i_am_ready_to_continue = __;

        assert!(i_am_ready_to_continue);
    }
}
