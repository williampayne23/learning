pub fn process_part_1(input: &str) {
    let mut max = -1;
    let mut running_total = 0;
    for line in input.lines() {
        if line.len() == 0 {
            if running_total > max {
                max = running_total;
            }
            running_total = 0;
            continue;
        }
        running_total += line.parse::<i32>().unwrap();
    }
    println!("{}", max);
}


pub fn process_part_2(input: &str) {
    let mut max = [-1,-1,-1];
    let mut running_total = 0;
    for line in input.lines() {
        if line.len() == 0 {
            if running_total > max[0] {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = running_total;
            } else if running_total > max[1] {
                max[2] = max[1];
                max[1] = running_total;
            } else if running_total > max[2] {
                max[2] = running_total;
            }
            running_total = 0;
            continue;
        }
        running_total += line.parse::<i32>().unwrap();
    }
    println!("{}", max[0] + max[1] + max[2]);
}
