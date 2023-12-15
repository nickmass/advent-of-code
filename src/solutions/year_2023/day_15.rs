pub fn part_one(input: &str) -> usize {
    input.trim().split(",").map(hash).sum()
}

pub fn part_two(input: &str) -> usize {
    let mut map: Vec<Vec<Entry>> = vec![Vec::new(); 256];

    'steps: for step in input.trim().split(",") {
        if step.ends_with('-') {
            let label = step.trim_end_matches('-');
            let hash = hash(label);

            if let Some(idx) = map[hash].iter().position(|e| e.0 == label) {
                map[hash].remove(idx);
            }
        } else {
            let (label, value) = step.split_once('=').unwrap();
            let value = value.parse().unwrap();
            let hash = hash(label);

            for old_entry in map[hash].iter_mut() {
                if old_entry.0 == label {
                    old_entry.1 = value;
                    continue 'steps;
                }
            }

            map[hash].push(Entry(label, value));
        }
    }

    let mut result = 0;

    for (b_idx, bucket) in map.into_iter().enumerate() {
        for (e_idx, entry) in bucket.into_iter().enumerate() {
            result += (b_idx + 1) * (e_idx + 1) * entry.1;
        }
    }

    result
}

#[derive(Debug, Copy, Clone)]
struct Entry<'a>(&'a str, usize);

fn hash(input: &str) -> usize {
    let mut current_value = 0;

    for c in input.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

#[test]
fn test() {
    let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    assert_eq!(1320, part_one(input));
    assert_eq!(145, part_two(input));
}
