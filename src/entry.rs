use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    Start,
    Stop,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub project: String,
    pub action: Action,
    pub created_at: String,
    pub updated_at: String,
}

impl Entry {
    pub fn start(title: String, project: String) -> Self {
        let now = chrono::Local::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            project,
            action: Action::Start,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn stop(&mut self) {
        self.action = Action::Stop;
        self.updated_at = chrono::Local::now().to_rfc3339();
    }
}
