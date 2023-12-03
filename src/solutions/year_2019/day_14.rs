use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    let reactions: HashMap<_, _> = input
        .lines()
        .map(|l| {
            let mut sides = l.split(" => ");
            let input = sides.next().unwrap();
            let output = sides.next().unwrap();

            let mut output = output.split_whitespace();
            let output_n: u64 = output.next().unwrap().parse().unwrap();
            let output_chem = output.next().unwrap();

            let mut inputs = Vec::new();

            for input in input.split(", ") {
                let mut input = input.split_whitespace();
                let input_n: u64 = input.next().unwrap().parse().unwrap();
                let input_chem = input.next().unwrap();

                inputs.push((input_chem, input_n))
            }

            (output_chem, (output_n, inputs))
        })
        .collect();

    let mut inventory = HashMap::new();
    fill_request(&reactions, &mut inventory, "FUEL", 1)
}

pub fn part_two(input: &str) -> u64 {
    let reactions: HashMap<_, _> = input
        .lines()
        .map(|l| {
            let mut sides = l.split(" => ");
            let input = sides.next().unwrap();
            let output = sides.next().unwrap();

            let mut output = output.split_whitespace();
            let output_n: u64 = output.next().unwrap().parse().unwrap();
            let output_chem = output.next().unwrap();

            let mut inputs = Vec::new();

            for input in input.split(", ") {
                let mut input = input.split_whitespace();
                let input_n: u64 = input.next().unwrap().parse().unwrap();
                let input_chem = input.next().unwrap();

                inputs.push((input_chem, input_n))
            }

            (output_chem, (output_n, inputs))
        })
        .collect();

    let n = 1_000_000_000_000;
    let mut inventory = HashMap::new();
    let ore_per_fuel_max = fill_request(&reactions, &mut inventory, "FUEL", 1);
    let mut fuel_min = n / ore_per_fuel_max;
    let mut fuel_max;

    loop {
        inventory.clear();
        let ore_count = fill_request(&reactions, &mut inventory, "FUEL", fuel_min);
        if ore_count < n {
            fuel_min *= 2;
        } else {
            fuel_max = fuel_min;
            fuel_min /= 2;
            break;
        }
    }

    loop {
        if fuel_max - fuel_min <= 1 {
            return fuel_min;
        }

        inventory.clear();
        let next_attempt = fuel_min + (fuel_max - fuel_min) / 2;
        let ore_count = fill_request(&reactions, &mut inventory, "FUEL", next_attempt);
        if ore_count < n {
            fuel_min = next_attempt;
        } else {
            fuel_max = next_attempt;
        }
    }
}

fn fill_request<'a>(
    reactions: &HashMap<&'a str, (u64, Vec<(&'a str, u64)>)>,
    inventory: &mut HashMap<&'a str, u64>,
    chem: &'a str,
    count: u64,
) -> u64 {
    let (n, inputs) = reactions.get(chem).expect("chem must exist");

    let needs = if count % n == 0 {
        count / n
    } else {
        (count / n) + 1
    };

    let mut ore_count = 0;

    for (in_chem, in_count) in inputs {
        let required_amount = needs * in_count;

        if in_chem == &"ORE" {
            ore_count += required_amount;
            continue;
        }

        let in_chem_current = inventory.remove(in_chem).unwrap_or(0);

        let new_amount = if required_amount > in_chem_current {
            ore_count += fill_request(
                reactions,
                inventory,
                in_chem,
                required_amount - in_chem_current,
            );

            let in_chem_new = inventory.remove(in_chem).unwrap_or(0);
            (in_chem_current + in_chem_new) - required_amount
        } else {
            in_chem_current - required_amount
        };

        inventory.insert(in_chem, new_amount);
    }

    inventory.insert(chem, needs * n);

    ore_count
}

#[test]
fn test() {
    let input = r#"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
"#;

    assert_eq!(2210736, part_one(input));
    assert_eq!(460664, part_two(input));
}
