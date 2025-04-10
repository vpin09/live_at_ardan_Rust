pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

#[derive(Debug, PartialEq, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}
#[derive(Debug, PartialEq, Clone)]
pub enum LoginRole {
    Admin,
    User,
}

pub struct User {
    username: String,
    password: String,
    role: LoginRole,
}
impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> Self {
        User {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_users() -> [User; 2] {
    [
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User),
    ]
}
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    let users = get_users();
    if let Some(user) = users.iter().find(|user| user.username == username) {
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
