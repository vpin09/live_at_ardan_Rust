fn main() {
    let m = 5;
    println!("{m}");

    // Mutable
    let mut n = 5;
    n += 1;
    println!("{n}");

    // Scope
    let x = 5;
    {
        let x = 6;
        println!("{x}");
    }
    println!("{x}");

    // function

    println!("double {}", double(5));

    println!("double or nothinng {}", double_or_nothing(10));

    let mut name = String::from("Vipin");

    greet_borrow(&name);
    greet_borrow_mut(&mut name);
    greet(name);

    let input = read_line();
    println!("input {}", input);
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Std input not working");
    input.trim().to_string()
}

fn greet_borrow_mut(name: &mut String) {
    *name = format!("Hello  {}", name);
}

fn greet_borrow(name: &String) {
    println!("calling greet borrowed");

    println!("Hello {}", &name);
}

fn greet(name: String) {
    println!("calling greet ");
    println!("Hello  {}", name);
}
fn double(n: i32) -> i32 {
    n * 2
}

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        return n * 2;
    }
    0
}
