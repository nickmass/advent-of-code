pub fn part_one(input: &str) -> usize {
    solve_part_one::<25, 6>(input)
}

pub fn solve_part_one<const WIDTH: usize, const HEIGHT: usize>(input: &str) -> usize {
    let layers = input.trim().as_bytes().chunks(WIDTH * HEIGHT);

    let mut result = (usize::MAX, 0);
    for layer in layers {
        let mut counts = [0; 3];

        layer
            .iter()
            .map(|p| p - 48)
            .map(usize::from)
            .filter(|&n| n < 3)
            .for_each(|i| counts[i] += 1);

        let zeros = counts.first().copied().unwrap_or(0);
        if zeros < result.0 {
            let ones = counts.get(1).copied().unwrap_or(0);
            let twos = counts.get(2).copied().unwrap_or(0);

            result = (zeros, ones * twos);
        }
    }

    result.1
}

pub fn part_two(input: &str) -> String {
    solve_part_two::<25, 6>(input)
}

pub fn solve_part_two<const WIDTH: usize, const HEIGHT: usize>(input: &str) -> String {
    let layers = input.trim().as_bytes().chunks(WIDTH * HEIGHT).rev();

    let mut canvas = vec![false; WIDTH * HEIGHT];

    for layer in layers {
        for (index, pixel) in layer.iter().enumerate() {
            canvas[index] = match pixel {
                b'0' => false,
                b'1' => true,
                _ => canvas[index],
            };
        }
    }

    let mut image = String::new();
    for row in canvas.chunks(WIDTH) {
        for &pixel in row {
            image.push_str(if pixel { "##" } else { "  " });
        }
        image.push('\n');
    }

    image
}

#[test]
fn test() {
    let input = r#"123456789012"#;

    assert_eq!(1, solve_part_one::<3, 2>(input));

    let input = "0222112222120000";
    assert_eq!("  ##\n##  \n", solve_part_two::<2, 2>(input));
}
