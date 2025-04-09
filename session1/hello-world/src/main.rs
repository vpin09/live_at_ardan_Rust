use authentication::{greet,read_line,login};
fn main() {
    println!("{}", greet("Vipin"));


    let mut tries:i8=0;
    loop {
        println!("please enter username ");
        let mut username=read_line();
        println!("please enter password");
        let mut password=read_line();

        if login(&username, &password) {

            println!("you have been succesfully logined");
            break;

        }

        tries += 1;
        if tries >= 3 {
            println!("login attempt limit exceed");
            break;
        }

        println!("Please try again : Inccorect  username or password");
        println!("login attemp left {}", 3-tries);

    }


}
