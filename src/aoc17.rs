pub fn run() -> anyhow::Result<()> {
    println!("day17-1: {}", run_1(169..=206, -108..=-68)?);
    println!("day17-2: {}", run_2(169..=206, -108..=-68)?);
    Ok(())
}

type Range = std::ops::RangeInclusive<isize>;

fn run_1(x_range: Range, y_range: Range) -> anyhow::Result<isize> {
    let mut maxes = Vec::new();
    for x_vel in 0..900 {
        for y_vel in 0..900 {
            maxes.push(simulate((x_vel, y_vel), &x_range, &y_range));
        }
    }
    Ok(maxes.into_iter().flatten().max().unwrap())
}

fn simulate(mut vel: (isize, isize), x_range: &Range, y_range: &Range) -> Option<isize> {
    let mut pos = (0, 0);
    let mut max_y = 0;
    let mut did_hit = false;
    while pos.0 <= *x_range.end() && pos.1 >= *y_range.end() {
        pos.0 += vel.0;
        pos.1 += vel.1;
        max_y = max_y.max(pos.1);
        // dbg! {(x_range, y_range)};
        if x_range.contains(&pos.0) && y_range.contains(&pos.1) {
            did_hit = true;
        }
        match vel.0.cmp(&0) {
            std::cmp::Ordering::Greater => vel.0 -= 1,
            std::cmp::Ordering::Less => vel.0 += 1,
            _ => (),
        }
        vel.1 -= 1;
    }
    if did_hit {
        // println!("XXX");
        Some(max_y)
    } else {
        None
    }
}

fn run_2(x_range: Range, y_range: Range) -> anyhow::Result<usize> {
    let mut cnt = 0;
    for x_vel in 0..800 {
        for y_vel in -300..800 {
            if simulate((x_vel, y_vel), &x_range, &y_range).is_some() {
                dbg! {(x_vel,y_vel)};
                cnt += 1;
            }
        }
    }
    Ok(cnt)
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc17_run_1() {
        assert_eq!(super::run_1(20..=30, -10..=-5).unwrap(), 45);
    }

    #[test]
    fn aoc17_run_2() {
        // assert_eq!(super::run_2(20..=30, -10..=-5).unwrap(), 112);
    }
}
