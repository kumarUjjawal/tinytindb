pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

#[derive(Debug, Clone, Copy)]
pub struct Row {
    pub id: u32,
    pub username: [u8; COLUMN_USERNAME_SIZE],
    pub email: [u8; COLUMN_EMAIL_SIZE],
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RowError {
    #[error("username is too long")]
    UserNameTooLong,
    #[error("email is too long")]
    EmailTooLong,
}

impl Row {
    pub fn empty() -> Self {
        Self {
            id: 0,
            username: [0; COLUMN_USERNAME_SIZE],
            email: [0; COLUMN_EMAIL_SIZE],
        }
    }

    /// Create a Row with validation
    pub fn from_values(id: u32, username: &str, email: &str) -> Result<Self, RowError> {
        let username_bytes = username.as_bytes();
        let email_bytes = email.as_bytes();

        if username_bytes.len() > COLUMN_USERNAME_SIZE {
            return Err(RowError::UserNameTooLong);
        }

        if email_bytes.len() > COLUMN_EMAIL_SIZE {
            return Err(RowError::EmailTooLong);
        }

        let mut row = Row::empty();
        row.id = id;

        row.username[..username_bytes.len()].copy_from_slice(username_bytes);
        row.email[..email_bytes.len()].copy_from_slice(email_bytes);

        Ok(row)
    }

    /// Interpret username as str
    ///
    pub fn username_as_str(&self) -> &str {
        let end = self
            .username
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(COLUMN_USERNAME_SIZE);
        std::str::from_utf8(&self.username[..end]).unwrap_or("")
    }

    pub fn email_as_str(&self) -> &str {
        let end = self
            .email
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(COLUMN_EMAIL_SIZE);
        std::str::from_utf8(&self.email[..end]).unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_values_success() {
        let row = Row::from_values(1, "alice", "alice@example.com").unwrap();
        assert_eq!(row.id, 1);
        assert_eq!(row.username_as_str(), "alice");
        assert_eq!(row.email_as_str(), "alice@example.com");
    }

    #[test]
    fn from_values_username_too_long() {
        let long_username = "x".repeat(COLUMN_USERNAME_SIZE + 1);
        let res = Row::from_values(1, &long_username, "a@example.com");
        assert_eq!(res.unwrap_err(), RowError::UserNameTooLong);
    }

    #[test]
    fn from_values_email_too_long() {
        let long_email = "y".repeat(COLUMN_EMAIL_SIZE + 1);
        let res = Row::from_values(1, "alice", &long_email);
        assert_eq!(res.unwrap_err(), RowError::EmailTooLong);
    }

    #[test]
    fn round_trip_username_and_email() {
        let row = Row::from_values(42, "bob", "bob@example.com").unwrap();
        assert_eq!(row.username_as_str(), "bob");
        assert_eq!(row.email_as_str(), "bob@example.com");
    }
}
