use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Auth {
    allowed_users: Arc<RwLock<Vec<i64>>>,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            allowed_users: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_users(&self, user_ids: &[i64]) {
        let mut users = self.allowed_users.write().await;
        for &id in user_ids {
            if !users.contains(&id) {
                users.push(id);
            }
        }
    }

    pub async fn is_allowed(&self, user_id: i64) -> bool {
        let users = self.allowed_users.read().await;
        users.is_empty() || users.contains(&user_id)
    }
}

impl Default for Auth {
    fn default() -> Self {
        Self::new()
    }
}
