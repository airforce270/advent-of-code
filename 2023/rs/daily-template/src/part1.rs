use crate::custom_error::AocError;

pub fn process(_input: &str) -> std::result::Result<String, AocError> {
    todo!("{{project-name}} - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "";
        let want = "";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
