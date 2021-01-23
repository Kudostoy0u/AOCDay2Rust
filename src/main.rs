use std::fs;
fn part1(stringtocompute: Vec<&str>, chartofind: &str, min: u16, max: u16) -> u32 {
    let mut occurences = 0;
    for i in stringtocompute {
        if &i == &chartofind {
            occurences += 1;
        }
    }
    if occurences >= min && occurences <= max {
        return 1;
    } else {
        return 0;
    }
}
fn part2(stringtocompute: Vec<&str>, chartofind: &str, min: u16, max: u16) -> u32 {
    let mut positionsfound = 0;
    for i in 0..stringtocompute.len() {
        if stringtocompute[i] == chartofind && i == min.into() {
            positionsfound += 1;
        } else if stringtocompute[i] == chartofind && i == max.into() {
            positionsfound += 1;
        }
    }
    if positionsfound == 1 {
        return 1;
    } else {
        return 0;
    }
}
fn main() {
    let contents = fs::read_to_string("./day2.txt").expect("Bad file read");
    let mut values = contents.split("\n").collect::<Vec<_>>();
    values.pop();
    let mut valids: u32 = 0;
    let mut valids2: u32 = 0;
    for i in &values {
        let nums = &i.split("-").collect::<Vec<_>>();
        let min = nums[0].parse::<u16>().unwrap();
        let max = nums[1].split(" ").collect::<Vec<_>>()[0]
            .parse::<u16>()
            .unwrap();
        let chartofind = nums[1].split(":").collect::<Vec<_>>()[0]
            .split(" ")
            .collect::<Vec<_>>()[1];
        let stringtocompute = i.split(": ").collect::<Vec<_>>()[1]
            .split("")
            .collect::<Vec<_>>();
        valids += part1(stringtocompute.clone(), chartofind, min, max);
        valids2 += part2(stringtocompute, chartofind, min, max);
    }
    println!("Part 1: {:?}\nPart 2: {:?}", valids, valids2);
}
