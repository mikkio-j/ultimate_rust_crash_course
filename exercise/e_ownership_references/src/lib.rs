pub fn check_is_plural(s: &String) -> bool {
    s.ends_with("s")
}

pub fn inspect(s: &String) {
    let is_plural: bool = check_is_plural(s);
    if is_plural {
        println!("Argument is plural!");
    } else {
        println!("Argument is singular!");
    }
}

pub fn change(s: &mut String) {
    let is_plural: bool = check_is_plural(s);

    if !is_plural {
        s.push_str("s");
    }
}

pub fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}

pub fn bedazzle(s: &mut String) {
    *s = String::from("sparkly");
}
