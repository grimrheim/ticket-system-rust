use std::path::PathBuf;
use chrono::{DateTime, Duration, Utc};    // cargo add chrono
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]   // необходим для сериализации/десериализации в json
pub enum Priority {
    Low,
    Medium,
    High
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low    => "Low",
            Priority::Medium => "Medium",
            Priority::High   => "High"
        }
    }

    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "Low"       => Some(Priority::Low),
            "Medium"    => Some(Priority::Medium),
            "High"      => Some(Priority::High),
            _           => None
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Status {
    Open,
    InProgress,
    Done
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Open        => "Open",
            Status::InProgress  => "InProgress",
            Status::Done        => "Done"
        }
    }

    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "Open"          => Some(Status::Open),
            "InProgress"    => Some(Status::InProgress),
            "Done"          => Some(Status::Done),
            _               => None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TicketError {
    AlreadyTaken,
    AlreadyDone
}

// Отдельный тип нужен, чтобы не перепутать его с простым числом u32
// Чтобы HashMap мог исп. тип как ключ, ключ должен стать хэшем
// для этого добавляем PartialEq & Eq & Hash
//
// Debug, Clone, Copy
// Debug позволяет печатать значение для отладки через {:?}, он же нужен для assert_eq
// Copy и Clone позволяют использовать id неск. раз без &.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
// Структура-кортеж, содержащая внутри себя один i32
// доступ к полю происходит по id.0
pub struct TicketId(pub(crate) u32);

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    pub(crate) id: TicketId,
    pub(crate) header: String,
    pub description: String,
    tags: Vec<String>,
    created: DateTime<Utc>,     // форматирование .format("%Y-%m-%d %H:%M")
    expires: DateTime<Utc>,
    path: Option<PathBuf>,
    status: Status,
    priority: Priority,
    parent: Option<TicketId>,
    children: Vec<TicketId>
}

impl Ticket {
    pub fn new(
        id: TicketId,
        header: String,
        description: String,
        tags: Vec<String>,
        path: Option<PathBuf>,
        priority: Priority,
        parent: Option<TicketId>,
    ) -> Self {

        let created = Utc::now();
        let status = Status::Open;
        let children = Vec::new();
        let expires = created + Duration::days(7);

        Self {
            id,
            header,
            description,
            tags,
            created,
            expires,
            path,
            status,
            priority,
            parent,
            children
        }
    }

    // Функция возвращает либо успех, либо ошибку
    // Result<T, E> это enum с двумя вариантами: Ok(T) и Err(E)
    pub fn take(&mut self) -> Result<(), TicketError> {
        match self.status {
            Status::Done        => Err(TicketError::AlreadyDone),
            Status::InProgress  => Err(TicketError::AlreadyTaken),
            Status::Open        => {
                self.status = Status::InProgress;
                Ok(())
            }
        }
    }

    pub fn finish(&mut self) -> Result<(), TicketError> {
        match self.status {
            Status::Done      => Err(TicketError::AlreadyDone),
            Status::Open | Status::InProgress      => {
                self.status = Status::Done;
                Ok(())
            }
        }
    }

    pub fn print_status(&self) {
        match self.status {
            Status::Open        => println!("Open!"),
            Status::InProgress  => println!("InProgress!"),
            Status::Done        => println!("Done!")
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_ticket() -> Ticket {
        Ticket::new(
            TicketId(1), String::from("name"),
            String::from("text"), Vec::new(),
            None, Priority::Low, None
        )
    } 
}

