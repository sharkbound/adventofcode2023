use daytemplate::{Day, DayPart};

pub struct Day9Part1;

impl Day for Day9Part1 {
    type ParseOutput = Vec<Vec<i64>>;

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        9
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        input.lines()
            .map(|line|
                line.split(' ')
                    .flat_map(|x| x.parse::<i64>())
                    .collect::<Vec<i64>>()
            ).collect()
    }

    fn solve(&self) {
        let input = self.input();
        // let input = self.sample("part_1");
        let parsed = self.parse(&input);
        let mut total = 0;
        for line in parsed.iter() {
            let mut prev = 0;
            let mut current = 0;
            for diff in generate_differences(line).iter().rev() {
                match diff.last() {
                    Some(&current_last_value) => {
                        current = current_last_value + prev;
                        prev = current_last_value + prev;
                    }
                    None => {}
                }
            }
            total += current;
        }

        println!("Day 9 Part 1: {:?}", total);
    }
}

fn diff_once(data: &[i64]) -> Option<Vec<i64>> {
    if data.len() < 2 {
        return None;
    }
    let mut diffs = Vec::new();
    for i in 0..data.len() - 1 {
        diffs.push(data[i + 1] - data[i]);
    }
    Some(diffs)
}

fn generate_differences(data: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut out = Vec::new();
    let mut current_diffs = data.clone();
    out.push(current_diffs.clone());
    loop {
        let diffs = match diff_once(&current_diffs) {
            Some(diffs) => diffs,
            None => return out,
        };
        out.push(diffs.clone());
        if diffs_are_zero(&diffs) {
            return out;
        }
        current_diffs = diffs;
    }
}

fn diffs_are_zero(diffs: &[i64]) -> bool {
    diffs.iter().all(|&x| x == 0)
}


/*
[[10, 13, 16, 21, 30, 45], 
[3, 3, 5, 9, 15], 
[0, 2, 4, 6], 
[2, 2, 2], 
[0, 0]]

10  13  16  21  30  45  68
   3   3   5   9  15  23
     0   2   4   6   8
       2   2   2   2
         0   0   0
*/