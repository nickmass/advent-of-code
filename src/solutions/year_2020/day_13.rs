pub fn part_one(input: &str) -> u32 {
    let mut lines = input.lines();
    let depart: u32 = lines.next().unwrap().trim().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|c| *c != "x")
        .filter_map(|n| n.parse().ok());

    let mut closest_bus = 0;
    let mut closest_depart = u32::MAX;
    for bus in buses {
        for n in 0.. {
            let time = n * bus;
            if time > depart && time < closest_depart {
                closest_depart = time;
                closest_bus = bus;
            }

            if time > closest_depart {
                break;
            }
        }
    }

    (closest_depart - depart) * closest_bus
}

pub fn part_two(input: &str) -> u64 {
    let mut lines = input.lines();
    let _depart = lines.next();
    let buses = lines.next().unwrap().split(',').filter_map(|n| {
        if n == "x" {
            Some(Bus::Any)
        } else {
            n.parse().ok().map(Bus::Id)
        }
    });

    let mut t_off = 0;
    let mut bus_reqs = Vec::new();
    let mut max_bus = 0;
    let mut max_bus_offset = 0;
    for bus in buses {
        match bus {
            Bus::Id(n) => {
                if n > max_bus {
                    max_bus = n;
                    max_bus_offset = t_off;
                }

                bus_reqs.push((n, t_off));
            }
            _ => (),
        }
        t_off += 1;
    }

    let mut step_by = max_bus;
    let mut n = max_bus - max_bus_offset;
    let mut last_n = n;
    let mut match_count = 1;
    let mut found_next_match = false;
    loop {
        let mut matches = 0;
        for (id, offset) in bus_reqs.iter() {
            if (n + offset) % id == 0 {
                matches += 1;
            } else {
                break;
            }
        }

        if matches == bus_reqs.len() {
            return n;
        }

        if matches > match_count {
            if found_next_match {
                match_count = matches;
                found_next_match = false;
                step_by = n - last_n;
            } else {
                last_n = n;
                found_next_match = true;
            }
        }

        n += step_by;
    }
}

enum Bus {
    Id(u64),
    Any,
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#"939
7,13,x,x,59,x,31,19"#;

    run_a(i, 295);
    run_b(i, 1068781);
}
