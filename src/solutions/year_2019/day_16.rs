pub fn part_one(input: &str) -> String {
    let mut signal: Vec<_> = input
        .chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some((c as u32 - '0' as u32) as u8)
            } else {
                None
            }
        })
        .collect();

    let mut next_signal = Vec::with_capacity(signal.len());
    let patterns: Vec<Vec<_>> = (0..signal.len())
        .map(|n| {
            (n..signal.len() + 1)
                .map(|i| pattern_at(i, n + 1))
                .collect()
        })
        .collect();
    for _ in 0..100 {
        next_signal.clear();

        for position in 0..signal.len() {
            let sum: i32 = signal[position..]
                .iter()
                .copied()
                .zip(patterns[position].iter())
                .map(|(d, p)| d as i32 * p)
                .sum();

            next_signal.push((sum.abs() % 10) as u8);
        }

        std::mem::swap(&mut signal, &mut next_signal);
    }

    signal
        .into_iter()
        .take(8)
        .map(|n| (n + b'0') as char)
        .collect()
}

pub fn part_two(input: &str) -> String {
    let input_len = input.trim().len();
    let repeat = 10000;
    let total_len = input_len * repeat;

    let input = input
        .chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some((c as u32 - '0' as u32) as u8)
            } else {
                None
            }
        })
        .cycle()
        .take(total_len);

    let offset = input.clone().take(7).enumerate().fold(0, |acc, (idx, n)| {
        acc + (n as usize * 10usize.pow(6 - idx as u32))
    });

    // Would run into performance issues if the offset ended having leading zeros,
    // would even give incorrect results if offset < (total_len / 2)
    assert!(offset > total_len / 2);

    let mut signal: Vec<_> = input.skip(offset).collect();

    for _phase in 0..100 {
        let mut sum = 0;
        for n in signal.iter_mut().rev() {
            sum += *n as i32;
            *n = (sum % 10) as u8
        }
    }

    signal
        .into_iter()
        .take(8)
        .map(|n| (n + b'0') as char)
        .collect()
}

fn pattern_at(index: usize, dupe_count: usize) -> i32 {
    let pattern = [0, 1, 0, -1];
    let true_idx = ((index + 1) / dupe_count) % 4;

    pattern[true_idx]
}

#[test]
fn test() {
    let inputs = [
        ("80871224585914546619083218645595)", "24176176"),
        ("19617804207202209144916044189917", "73745418"),
        ("69317163492948606335995924319873", "52432133"),
    ];

    for (input, result) in inputs {
        assert_eq!(result, part_one(input));
    }

    let inputs = [
        ("03036732577212944063491565474664", "84462026"),
        ("02935109699940807407585447034323", "78725270"),
        ("03081770884921959731165446850517", "53553731"),
    ];

    for (input, result) in inputs {
        assert_eq!(result, part_two(input));
    }
}
