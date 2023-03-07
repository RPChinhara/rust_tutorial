fn main() {
    let mut s = String::from("hello world");
    let _word = first_word(&s[0..6]);
    let _word = first_word(&s[..]);
    let _word = first_word(&s);

    let my_string_literal = "hello world";
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);
    let _word = first_word(&my_string_literal);

    s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return  &s[0..i];
        }
    }

    &s[..]
}