fn check_sum_of_3(start: u64, mark: u64, values: Vec<u64>, sum: u64) -> Option<u64>{
    for val in values.clone() {
        if start + mark + val == sum {
            return Some(start * mark * val);
        }
    }
    return None;
}

pub fn exo(values: Vec<u64>) -> u64{
    let mut vals = values.clone();

    loop {
        if vals.is_empty() {
            break;
        }
        let start = vals.pop().unwrap_or(u64::MAX);
        if start == u64::MAX {
            break;
        }
        for val in vals.clone() {
            match check_sum_of_3(start, val, vals.clone(), 2020) {
                Some(res) => {
                    return res;
                },
                _ => {}
            }
        }
    }

    return u64::MAX;
}
