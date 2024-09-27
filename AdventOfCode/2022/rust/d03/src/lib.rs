
pub fn to_priority(input: char) -> usize {
    " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().position(|c| c == input).unwrap()
}

pub fn process_part_1(input: &str) {
    let mut total: usize = 0;
    for line in input.lines() {
        //Split string in half
        let (p1, p2) = line.split_at(line.len()/2);
        for c1 in p1.chars() {
            let mut matched = false;
            for c2 in p2.chars() {
                if c1 == c2 {
                    total += to_priority(c1);
                    matched = true;
                    break
                }
            }
            if matched {
                break
            }
        }
    }
    println!("{}", total);
}


pub fn process_part_2(input: &str) {
    //Get three lines at a time from input
    let mut lines = input.lines();
    let mut total: usize = 0;
    loop {
        let line1 = lines.next();
        let line2 = lines.next();
        let line3 = lines.next();
        if line1 == None || line2 == None || line3 == None {
            break
        }
        let line1 = line1.unwrap();
        let line2 = line2.unwrap();
        let line3 = line3.unwrap();
        println!("{} {} {}", line1, line2, line3);
        let mut matched_chars = "".to_owned();
        for c1 in line1.chars() {
            for c2 in line2.chars() {
                if c1 == c2 {
                    //Append to matched_chars
                    matched_chars.push(c1);
                }
            }
        }
        println!("Matched Chars: {}", matched_chars);
        for c1 in matched_chars.chars() {
            let mut done = false;
            for c2 in line3.chars() {
                if c1 == c2 {
                    println!("{} == {}", c1, c2);
                    total += to_priority(c1);
                    done = true;
                    break
                }
            }
            if done {
                break
            }
        }
    }
    println!("{}", total);
}
