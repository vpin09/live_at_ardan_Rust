use authentication::{ greet, login, read_line, LoginAction };
fn main() {
    println!("{}", greet("Vipin"));

    let mut tries: i8 = 0;
    loop {
        println!("please enter username ");
        let mut username = read_line();
        println!("please enter password");
        let mut password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    authentication::LoginRole::Admin => println!("Admin"),
                    authentication::LoginRole::User => println!("User"),
                }
                break;
            }
            Some(LoginAction::Denied) => {}
            None=>{
                println!("new User please register");
                break;
            }
        }

        tries += 1;
        if tries >= 3 {
            println!("login attempt limit exceed");
            break;
        }
        println!("Please try again : Inccorect  username or password");
        println!("login attemp left {}", 3 - tries);
    }
}
