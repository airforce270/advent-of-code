use crate::custom_error::AocError;

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut elf_totals: Vec<i32> = vec![];

    let mut current_elf_items: Vec<i32> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            elf_totals.push(current_elf_items.iter().sum());
            current_elf_items.clear();
            continue;
        }
        current_elf_items.push(line.parse().unwrap());
    }

    Ok(elf_totals.iter().max().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let want = "24000";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
