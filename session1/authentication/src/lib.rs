pub fn greet(name : &str) -> String{
    format!("Hello {}", name)
}

#[derive(Debug,PartialEq)]
pub  enum  LoginAction{
    Granted(LoginRole),
    Denied
    
}
#[derive(Debug,PartialEq,Eq)]
pub enum LoginRole {
    Admin,
    User,  
}
pub  fn login(username: &str, password:&str) ->Option<LoginAction> {

    let username=username.to_lowercase();

    if username != "admin" && username != "bob" {
        return  None;
    }
    if username=="admin" && password=="password" {
       Some(LoginAction::Granted(LoginRole::Admin))
    }else if username=="bob" && password=="password" {
        Some(LoginAction::Granted(LoginRole::User))
    }else {
        Some(LoginAction::Denied)
    }
    
}

pub fn read_line() ->String {
    let mut input=String::new();
    std::io::stdin().read_line(&mut input).expect("Std input not working");
    input.trim().to_string()
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_works(){
        assert_eq!("Hello Vipin",greet("Vipin"));
    }
    #[test]

    fn test_login() {
        // assert_eq!(login("admin", "password"),LoginAction::Admin);
        // assert_eq!(login("ADMIN", "password"),LoginAction::Admin);
        // assert_eq!(login("bob", "password"),LoginAction::User);
        // assert_eq!(login("admin", "wrong-password"),LoginAction::Denied);

    }
    
}
