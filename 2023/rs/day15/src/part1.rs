use crate::custom_error::AocError;

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut values = Vec::new();

    for item in input.replace("\n", "").split(',') {
        values.push(hash_value(item));
    }

    Ok(values.iter().sum::<u32>().to_string())
}

fn hash_value(input: &str) -> u32 {
    let mut val = 0;

    for c in input.chars() {
        val += c as u32;
        val *= 17;
        val %= 256;
    }

    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_simple() {
        let input = "HASH";
        let want = "52";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_process_complex() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let want = "1320";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
