pub fn part_one(input: &str) -> u16 {
    input
        .trim()
        .lines()
        .filter_map(Blueprint::parse)
        .map(|b| b.find_max_score(24, false))
        .sum()
}

pub fn part_two(input: &str) -> u16 {
    input
        .trim()
        .lines()
        .filter_map(Blueprint::parse)
        .take(3)
        .map(|b| b.find_max_score(32, true))
        .product()
}

#[derive(Debug, Clone, Copy)]
enum Action {
    DoNothing,
    BuildOreRobot,
    BuildClayRobot,
    BuildObsidianRobot,
    BuildGeodeRobot,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Resources {
    ore: u16,
    ore_robots: u16,
    clay: u16,
    clay_robots: u16,
    obsidian: u16,
    obsidian_robots: u16,
    geodes: u16,
    geode_robots: u16,
    time_remaining: u16,
}

impl Resources {
    fn new(time_remaining: u16) -> Self {
        Resources {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geodes: 0,
            geode_robots: 0,
            time_remaining,
        }
    }
}

struct Blueprint {
    id: u16,
    ore_robot_ore: u16,
    clay_robot_ore: u16,
    obsidian_robot_ore: u16,
    obsidian_robot_clay: u16,
    geode_robot_ore: u16,
    geode_robot_obsidian: u16,
}

impl Blueprint {
    fn parse(line: &str) -> Option<Self> {
        let line = line.trim_start_matches("Blueprint ");
        let (id, line) = line.split_once(": Each ore robot costs ")?;
        let (ore_robot_ore, line) = line.split_once(" ore. Each clay robot costs ")?;
        let (clay_robot_ore, line) = line.split_once(" ore. Each obsidian robot costs ")?;
        let (obsidian_robot_ore, line) = line.split_once(" ore and ")?;
        let (obsidian_robot_clay, line) = line.split_once(" clay. Each geode robot costs ")?;
        let (geode_robot_ore, line) = line.split_once(" ore and ")?;
        let (geode_robot_obsidian, _line) = line.split_once(" ")?;

        let id = id.parse().ok()?;
        let ore_robot_ore = ore_robot_ore.parse().ok()?;
        let clay_robot_ore = clay_robot_ore.parse().ok()?;
        let obsidian_robot_ore = obsidian_robot_ore.parse().ok()?;
        let obsidian_robot_clay = obsidian_robot_clay.parse().ok()?;
        let geode_robot_ore = geode_robot_ore.parse().ok()?;
        let geode_robot_obsidian = geode_robot_obsidian.parse().ok()?;

        Some(Blueprint {
            id,
            ore_robot_ore,
            clay_robot_ore,
            obsidian_robot_ore,
            obsidian_robot_clay,
            geode_robot_ore,
            geode_robot_obsidian,
        })
    }

    fn possible_actions(&self, resources: &Resources) -> impl Iterator<Item = Action> {
        let ore_robot = (resources.ore >= self.ore_robot_ore).then_some(Action::BuildOreRobot);
        let clay_robot = (resources.ore >= self.clay_robot_ore).then_some(Action::BuildClayRobot);
        let obsidian_robot = (resources.ore >= self.obsidian_robot_ore
            && resources.clay >= self.obsidian_robot_clay)
            .then_some(Action::BuildObsidianRobot);
        let geode_robot = (resources.ore >= self.geode_robot_ore
            && resources.obsidian >= self.geode_robot_obsidian)
            .then_some(Action::BuildGeodeRobot);

        let do_nothing = (resources.ore / 2 < self.ore_robot_ore).then_some(Action::DoNothing);

        [
            do_nothing,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        ]
        .into_iter()
        .flatten()
    }

    fn perform_action(&self, resources: &mut Resources, action: Action) {
        resources.time_remaining -= 1;

        resources.ore += resources.ore_robots;
        resources.clay += resources.clay_robots;
        resources.obsidian += resources.obsidian_robots;
        resources.geodes += resources.geode_robots;

        match action {
            Action::DoNothing => (),
            Action::BuildOreRobot => {
                resources.ore -= self.ore_robot_ore;
                resources.ore_robots += 1;
            }
            Action::BuildClayRobot => {
                resources.ore -= self.clay_robot_ore;
                resources.clay_robots += 1;
            }
            Action::BuildObsidianRobot => {
                resources.ore -= self.obsidian_robot_ore;
                resources.clay -= self.obsidian_robot_clay;
                resources.obsidian_robots += 1;
            }
            Action::BuildGeodeRobot => {
                resources.ore -= self.geode_robot_ore;
                resources.obsidian -= self.geode_robot_obsidian;
                resources.geode_robots += 1;
            }
        }
    }

    fn score(&self, resources: &Resources, score_method: bool) -> u16 {
        if score_method {
            resources.geodes
        } else {
            self.id * resources.geodes
        }
    }

    fn find_max_score(&self, time_remaining: u16, score_method: bool) -> u16 {
        let mut max_geodes_at_time = [0; 100];

        let resources = Resources::new(time_remaining);
        let mut attempted = crate::HashSet::new();
        let mut max_score = 0;
        let mut tree = Vec::new();
        tree.extend(
            self.possible_actions(&resources)
                .map(|a| (a, resources.clone())),
        );

        while let Some((action, mut resources)) = tree.pop() {
            self.perform_action(&mut resources, action);

            if resources.time_remaining == 1 {
                self.perform_action(&mut resources, Action::DoNothing);
                let score = self.score(&resources, score_method);
                if score > max_score {
                    max_score = score;
                }

                continue;
            }

            let time_idx = resources.time_remaining as usize;

            // This is hacky and not safe to assume that it wont filter out the optimal path
            if max_geodes_at_time[time_idx] > resources.geodes + 2 {
                continue;
            } else if max_geodes_at_time[time_idx] < resources.geodes {
                max_geodes_at_time[time_idx] = resources.geodes;
            }

            if !attempted.contains(&resources) {
                tree.extend(
                    self.possible_actions(&resources)
                        .map(|a| (a, resources.clone())),
                );
                attempted.insert(resources);
            }
        }

        max_score
    }
}

#[test]
#[ignore = "too slow"]
fn test() {
    let input = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;

    assert_eq!(33, part_one(input));
    assert_eq!(56 * 62, part_two(input));
}
