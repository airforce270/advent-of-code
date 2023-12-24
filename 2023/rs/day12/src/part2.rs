use std::collections::HashMap;

use crate::custom_error::AocError;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Spring {
    Working,
    Broke,
    Unknown,
}

impl Spring {
    fn from_char(ch: char) -> Spring {
        match ch {
            '.' => Spring::Working,
            '#' => Spring::Broke,
            '?' => Spring::Unknown,
            _ => panic!("unhandled char: {ch}"),
        }
    }
}

struct Row {
    springs: Vec<Spring>,
    arrangements: Vec<u8>,
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let rows: Vec<Row> = input
        .lines()
        .map(|line| Row {
            springs: unfold(
                line.split(" ")
                    .next()
                    .unwrap()
                    .chars()
                    .map(|ch| Spring::from_char(ch))
                    .collect(),
                Some(Spring::Unknown),
            ),
            arrangements: unfold(
                line.split(" ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|num| num.parse::<u8>().unwrap())
                    .collect(),
                None,
            ),
        })
        .collect();

    let mut cache = HashMap::new();

    let mut sum: u64 = 0;

    let mut count = 0;
    for row in rows.iter() {
        let (result, new_cache) = count_row(row, cache.clone());
        sum += result;
        for (key, val) in new_cache.iter() {
            if !cache.contains_key(key) {
                cache.insert(key.clone(), *val);
            }
        }
        count += 1;
        println!("processed {count} lines, {} cache entries", cache.len());
    }

    Ok(sum.to_string())
}

fn unfold<I>(v: Vec<I>, sep: Option<I>) -> Vec<I>
where
    I: Clone,
{
    let mut ret = v.clone();
    for _ in 1..5 {
        if let Some(ref s) = sep {
            ret.push(s.clone());
        }
        for val in v.iter() {
            ret.push(val.clone());
        }
    }
    ret
}

fn count_row(row: &Row, cache: Cache) -> (u64, Cache) {
    count(row.springs.iter(), row.arrangements.iter(), cache)
}

type Cache = HashMap<CacheKey, u64>;
type CacheKey = (Vec<Spring>, Vec<u8>);

fn count<'a, 'b, I, I2>(springs: I, arrangements: I2, cache: HashMap<CacheKey, u64>) -> (u64, Cache)
where
    I: Iterator<Item = &'a Spring> + Clone,
    I2: Iterator<Item = &'b u8> + Clone,
{
    // Disclaimer: I didn't understand the problem well, and my solution is
    // heavily based on https://www.youtube.com/watch?v=g3Ms5e7Jdqo
    //
    // This is also terrible Rust code.

    if springs.clone().count() == 0 {
        return match arrangements.count() == 0 {
            true => (1, cache),
            false => (0, cache),
        };
    }
    if arrangements.clone().count() == 0 {
        return match springs.filter(|s| **s == Spring::Broke).count() > 0 {
            true => (0, cache),
            false => (1, cache),
        };
    };

    let cache_key: CacheKey = (
        springs.clone().cloned().collect(),
        arrangements.clone().cloned().collect(),
    );

    if cache.contains_key(&cache_key) {
        return (cache[&cache_key], cache);
    }

    let first_spring = springs.clone().next().unwrap();
    let spring_count = springs.clone().count();
    let first_arr = *arrangements.clone().next().unwrap();
    let first_arr_usize = <u8 as Into<usize>>::into(first_arr);

    let (result1, result_cache1) = match first_spring {
        Spring::Working | Spring::Unknown => count(
            springs
                .clone()
                .skip(1)
                .cloned()
                .collect::<Vec<Spring>>()
                .iter(),
            arrangements.clone().cloned().collect::<Vec<u8>>().iter(),
            cache,
        ),
        Spring::Broke => (0, cache),
    };
    let (result2, mut result_cache2) = match first_spring {
        Spring::Broke | Spring::Unknown => {
            if <u8 as Into<usize>>::into(first_arr) <= spring_count
                && springs
                    .clone()
                    .take(first_arr.into())
                    .filter(|sp| **sp == Spring::Working)
                    .peekable()
                    .peek()
                    .is_none()
                && (<u8 as Into<usize>>::into(first_arr) == spring_count
                    || *springs
                        .clone()
                        .skip(first_arr.into())
                        .next()
                        .unwrap_or(&Spring::Working)
                        != Spring::Broke)
            {
                count(
                    springs
                        .clone()
                        .skip(first_arr_usize + 1)
                        .cloned()
                        .collect::<Vec<Spring>>()
                        .iter(),
                    arrangements
                        .clone()
                        .skip(1)
                        .cloned()
                        .collect::<Vec<u8>>()
                        .iter(),
                    result_cache1,
                )
            } else {
                (0, result_cache1)
            }
        }
        Spring::Working => (0, result_cache1),
    };

    let result = result1 + result2;
    result_cache2.insert(cache_key, result);

    (result,result_cache2) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let want = "525152";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
