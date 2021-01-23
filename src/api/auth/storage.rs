use super::dto::UserLoginInfo;

pub struct UserTable(std::collections::HashSet<UserLoginInfo>);

impl UserTable {
    pub fn new(table: Vec<UserLoginInfo>) -> Self {
        Self(table.into_iter().collect())
    }

    pub fn contains(&self, info: &UserLoginInfo) -> bool {
        self.0.contains(&info)
    }
}
