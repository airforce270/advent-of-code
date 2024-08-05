use crate::custom_error::AocError;
use std::collections::HashMap;

enum Operation {
    Dash,
    Equal(u32),
}

struct Lens {
    label: String,
    operation: Operation,
}

impl Lens {
    fn new(input: &str) -> Self {
        let op = if input.contains('=') {
            Operation::Equal(
                input
                    .split('=')
                    .last()
                    .expect("equal input has at least one value")
                    .parse::<u32>()
                    .expect("equal focal length is parsable as a u32"),
            )
        } else {
            Operation::Dash
        };
        let label = match op {
            Operation::Dash => input.replace("-", ""),
            Operation::Equal(_) => input
                .split('=')
                .next()
                .expect("equal input has a label")
                .to_string(),
        };

        Self {
            label: label.to_string(),
            operation: op,
        }
    }

    fn box_no(&self) -> u32 {
        hash_value(&self.label)
    }
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();
    for i in 0..256 {
        boxes.insert(i, Vec::new());
    }

    for lens_str in input.replace("\n", "").split(',') {
        let lens = Lens::new(lens_str);
        let box_no = lens.box_no();

        let lenses = boxes
            .get_mut(&box_no)
            .expect(format!("box {} exists", box_no).as_str());

        match lens.operation {
            Operation::Equal(_) => {
                if lenses.iter().any(|l| l.label == lens.label) {
                    let index = lenses.iter().position(|l| l.label == lens.label).expect(
                        format!("box {} has a lens with label {}", box_no, lens.label).as_str(),
                    );
                    lenses[index] = lens;
                } else {
                    lenses.push(lens);
                }
            }
            Operation::Dash => {
                lenses.retain(|l| l.label != lens.label);
            }
        }
    }

    Ok(boxes
        .iter()
        .map(|(i, lenses)| calc_box(*i, lenses))
        .sum::<u32>()
        .to_string())
}

fn calc_box(box_no: u32, b: &Vec<Lens>) -> u32 {
    let mut total = 0;

    for (i, item) in b.iter().enumerate() {
        total += match item.operation {
            Operation::Equal(focal_length) => {
                (box_no + 1)
                    * (<usize as TryInto<u32>>::try_into(i).expect("i is a u32") + 1)
                    * focal_length
            }
            Operation::Dash => 0,
        }
    }

    total
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
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let want = "145";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
