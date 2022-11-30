use std::collections::HashMap;
const LEN: usize = 9;

pub fn run() {
    println!("23:1 {}", run_1(389547612, 100));
    println!("23:2 {}", run_2(389547612));
}

fn to_indices(mut val: usize) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    let mut idx = LEN;
    while val > 0 {
        idx -= 1;
        res.insert(val % 10, idx);
        val /= 10;
    }
    res
}

fn from_indices(idx: &HashMap<usize, usize>) -> usize {
    let mut res = 0;
    let start_idx = idx.values().min().unwrap().clone();
    let end_idx = start_idx + LEN;
    for i in start_idx..end_idx {
        res *= 10;
        let val = idx.iter().find(|(_, idx)| i == **idx).unwrap();
        res += val.0;
    }

    res
}

fn make_move(current_idx: usize, input: &mut HashMap<usize, usize>) {
    let taken: Vec<usize> = ((current_idx + 1)..(current_idx + 4)).map(|a| a).collect();
    todo!()
}

fn run_1(cups: usize, iterations: usize) -> usize {
    let mut idx = to_indices(cups);
    let mut cur_idx = 0;

    for _ in 0..iterations {
        make_move(cur_idx, &mut idx);
        cur_idx += 1;
    }

    from_indices(&idx)
}

fn run_2(cups: usize) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    const INPUT_1: usize = 389125467;

    #[test]
    fn aoc23_run_idx() {
        let idx = super::to_indices(INPUT_1);
        assert_eq!(idx.len(), 9);
        assert_eq!(super::from_indices(&idx), INPUT_1);
    }
    #[test]
    fn aoc23_run_make_move() {
        let mut idx = super::to_indices(INPUT_1);
        super::make_move(0, &mut idx);
        assert_eq!(super::from_indices(&idx), 328915467);
    }

    #[test]
    fn aoc23_run_1() {
        assert_eq!(super::run_1(INPUT_1, 10), 92658374);
        assert_eq!(super::run_1(INPUT_1, 100), 67384529);
    }
}
