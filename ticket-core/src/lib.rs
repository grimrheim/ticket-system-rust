use std::{path::PathBuf};
use chrono::{DateTime, Duration, Utc};    // cargo add chrono
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]   // необходим для сериализации/десериализации в json
pub enum Priority {
    Low,
    Medium,
    High
}

#[derive(Serialize, Deserialize)]
enum Status {
    Open,
    InProgress,
    Done
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum TicketError {
    AlreadyTaken,
    AlreadyDone
}

#[derive(Serialize, Deserialize)]
pub struct TicketId(u32);

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    id: TicketId,
    header: String,
    description: String,
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
    use super::*;   // подтягивает все из родительского модуля

    fn make_ticket() -> Ticket {
        Ticket::new(
            TicketId(1), String::from("name"),
            String::from("text"), Vec::new(),
            None, Priority::Low, None
        )
    }
 
    #[test]
    fn take_then_finish() {
        let mut t = make_ticket();
        assert!(t.take().is_ok());
        assert!(t.finish().is_ok());
    }

    #[test]
    fn finish_directly() {
        let mut t = make_ticket();
        assert!(t.finish().is_ok());
        assert!(t.finish().is_err());
    }

    #[test]
    fn cannot_take_down() {
        let mut t = make_ticket();
        assert!(t.finish().is_ok());
        assert!(t.take().is_err());
    }

    #[test]
    fn serde_test() {
        let t = make_ticket();

        let json = serde_json::to_string_pretty(&t).unwrap();
        println!("{}", json);

        let back: Ticket = serde_json::from_str(&json).unwrap();
        assert_eq!(back.header, t.header);
    }
}
