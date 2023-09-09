// These constants will eventually be stored in a config file

// Determines how long a jwt should remain valid
pub const DAYS_VALID: i64 = 3;
pub const JWT_SECRET: &[u8] = "secret".as_bytes();

pub const USER_TREE_NAME: &str = "users";
pub const USER_ID_NAME: &str = "user_id";
pub const REG_CODE_NAME: &str = "reg_code";
