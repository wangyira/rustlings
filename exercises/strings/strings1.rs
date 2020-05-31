// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

// I AM NOT DONE

fn main() {
    let answer = current_favorite_color1();
}

//sol1
fn current_favorite_color() -> &'static str {
    "blue"
}

//sol2 
fn current_favorite_color1() -> String {
    String::from("blue")
}

//error
// fn current_favorite_color2() -> & str {
//     "blue"
// }
//you can't return a reference to data which isn't owned by something outside the function.

//https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=17a63feac96135d189ac4912dcfe6925