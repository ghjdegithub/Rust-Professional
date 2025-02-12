use std::cmp::Ordering;

pub fn goldbach_conjecture() -> String {
    let mut accumulator = PrimeAccumulator::new();
    let vec = (3..)
        .filter(|&i| i % 2 != 0)
        .filter(|&i| {
            let mut find = false;
            let primes = accumulator.get_primes_in_range(i);
            for &prime in primes {
                if find {
                    break;
                }
                let mut idx: i32 = 0;
                loop {
                    let result = prime + (2 * idx.pow(2) as u32);
                    match result.cmp(&i) {
                        Ordering::Less => {
                            idx += 1;
                        }
                        Ordering::Equal => {
                            find = true;
                            break;
                        }
                        Ordering::Greater => {
                            break;
                        }
                    }
                }
            }
            !find
        })
        .take(2);

    vec.map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

struct PrimeAccumulator {
    primes: Vec<u32>,
    limit: u32,
}

impl PrimeAccumulator {
    fn new() -> Self {
        // 初始化时，素数表为空，起始值为 2
        PrimeAccumulator {
            primes: vec![2],
            limit: 2,
        }
    }

    // 获取当前的所有素数
    fn get_primes(&self) -> &[u32] {
        &self.primes
    }

    // 扩展素数表，筛选出新的素数
    fn expand_primes(&mut self, new_limit: u32) {
        let mut candidate = self.limit + 1;

        // 如果 new_limit 小于或等于当前的 limit，不需要扩展
        if new_limit <= self.limit {
            return;
        }

        // 扩展素数表直到 new_limit
        while candidate <= new_limit {
            let mut is_prime = true;
            for &prime in &self.primes {
                if prime * prime > candidate {
                    break;
                }
                if candidate % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                self.primes.push(candidate);
            }
            candidate += 1;
        }

        // 更新 limit
        self.limit = new_limit;
    }

    // 获取指定范围内的所有素数
    fn get_primes_in_range(&mut self, new_limit: u32) -> &[u32] {
        self.expand_primes(new_limit);
        self.get_primes()
    }
}

struct CustomSequence {
    max: u32,
    cached_results: Vec<u32>,
}

impl CustomSequence {
    fn new(max: u32) -> Self {
        CustomSequence {
            max,
            cached_results: Vec::new(),
        }
    }

    // 生成并缓存所有小于等于 max 的结果
    fn generate(&mut self) {
        // 如果当前的 cached_results 没有计算到 max，需要重新计算并扩展缓存
        if self.cached_results.len() < self.max as usize {
            for i in (self.cached_results.len() as u32 + 1)..=self.max {
                let result = 2 * i.pow(2);
                self.cached_results.push(result);
            }
        }
    }

    // 获取指定范围内的结果
    fn get_range(&mut self, end: u32) -> Vec<u32> {
        // 确保缓存中包含所有可能的结果
        self.generate();

        // 返回指定范围内的值
        let end_idx = end as usize;

        self.cached_results[..end_idx].to_vec()
    }

    // 动态增加最大值
    fn set_max(&mut self, new_max: u32) {
        if new_max > self.max {
            self.max = new_max;
            self.generate(); // 重新生成更大的范围
        }
    }
}
