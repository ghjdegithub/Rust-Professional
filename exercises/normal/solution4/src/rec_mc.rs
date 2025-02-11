use std::cmp::min;

pub fn dp_rec_mc(amount: u32) -> u32 {
    let cashes = [1, 2, 5, 10, 20, 30, 50, 100];
    let mut rec = vec![0; (amount + 1) as usize];
    for i in 1..=amount {
        let mut min_cashes_num = i;
        for c in cashes.iter().filter(|c| *c <= &i) {
            let curr = rec[(i - c) as usize] + 1;
            min_cashes_num = min(min_cashes_num, curr);
        }
        rec[i as usize] = min_cashes_num;
    }

    rec[amount as usize] as u32
}
