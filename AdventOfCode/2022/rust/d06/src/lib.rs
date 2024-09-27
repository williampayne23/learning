pub fn process_part_1(input: &str) -> i32 {
    let mut chars = "".to_string();
    let mut i = 1;
    for c in input.chars() {
        let pos = chars.chars().position(|x| x == c);
        chars.push(c);
        if let Some(p) = pos {
            let (_, new_chars) = chars.split_at(p + 1);
            chars = new_chars.to_string();
        }
        if chars.len() == 4 {
            println!("{} {}", i, chars);
            return i
        }
        i += 1;
    }
    return -1;
}


pub fn process_part_2(input: &str) -> i32 {
    let mut chars = "".to_string();
    let mut i = 1;
    for c in input.chars() {
        let pos = chars.chars().position(|x| x == c);
        chars.push(c);
        if let Some(p) = pos {
            let (_, new_chars) = chars.split_at(p + 1);
            chars = new_chars.to_string();
        }
        if chars.len() == 14 {
            println!("{} {}", i, chars);
            return i
        }
        i += 1;
    }
    return -1;
}
