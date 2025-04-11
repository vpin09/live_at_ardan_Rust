use std::{ collections::HashMap, path::Path };
use serde::{ Serialize, Deserialize };

pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

#[derive(Debug, PartialEq, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
    role: LoginRole,
}
impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> Self {
        User {
            username: username.to_lowercase(),
            password: hash_passowrd(password),
            role,
        }
    }
}

pub fn hash_passowrd(password: &str) -> String {
    use sha2::Digest;

    let mut hasher=sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}",hasher.finalize())
}
/*pub fn get_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User)
    ]
}*/

pub fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));

    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    users
}

pub fn get_users() -> HashMap<String, User> {
    let user_path = Path::new("user.json");
    if user_path.exists() {
        let user_json = std::fs::read_to_string(user_path).unwrap();
        let user: HashMap<String, User> = serde_json::from_str(&user_json).unwrap();
        user
    } else {
        let user = get_default_users();
        let user_json = serde_json::to_string(&user).unwrap();
        std::fs::write(user_path, user_json);
        user
    }
}

/* 
pub fn get_admin_user() {
    let users: Vec<String> = get_users()
        .into_iter()
        .filter(|u| u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect();
}*/
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password=hash_passowrd(password);

    let users = get_users();
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }

    None
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Std input not working");
    input.trim().to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_works() {
        assert_eq!("Hello Vipin", greet("Vipin"));
    }
    #[test]
    fn test_login() {
        // assert_eq!(login("admin", "password"),LoginAction::Admin);
        // assert_eq!(login("ADMIN", "password"),LoginAction::Admin);
        // assert_eq!(login("bob", "password"),LoginAction::User);
        // assert_eq!(login("admin", "wrong-password"),LoginAction::Denied);

    }
}
