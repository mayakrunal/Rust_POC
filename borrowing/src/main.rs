fn main() {
    let s1 = String::from("Hello");

    let len = calculate_lenght(&s1);

    println!("The lenght of '{}' is {}", s1, len);

    let mut s = String::from("Hello");

    change(&mut s);

    println!("s = {s}");
}

fn calculate_lenght(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
