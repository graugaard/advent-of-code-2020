use crate::util::puzzle_input;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;
use std::str::FromStr;

fn split_str_into_container_and_contained_color(str: &str) -> (&str, &str) {
    let test: Vec<&str> = str.split(" bags contain ").collect();
    (test[0], test[1])
}

fn split_input_string(str: &str) -> Vec<(&str, Vec<BagVal>)> {
    let mut vec = Vec::new();
    for line in str.lines() {
        let (color, colors) = split_str_into_container_and_contained_color(line);
        vec.push((color, derive_colors(colors)));
    }
    vec
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct BagVal {
    color: String,
    number: u64,
}

impl FromStr for BagVal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let end = if s.ends_with("s") { 5 } else { 4 };

        let number = s[0..1].parse::<u64>().expect("Always a number");
        let color = s[2..s.len() - end].to_string();
        Ok(BagVal { color, number })
    }
}

fn derive_colors(str: &str) -> Vec<BagVal> {
    if "no other bags.".eq(str) {
        return vec![];
    }

    let res = str[..str.len() - 1]
        .split(", ")
        // assume we start with "\d " and ends in " bags" or " bag"
        .map(|str| str.parse().unwrap())
        .collect();

    res
}

fn build_graph_contained_to_containers<'a>(
    nodes: &'a [(&str, Vec<BagVal>)],
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut map = HashMap::new();
    for node in nodes.iter() {
        for contained in node.1.iter() {
            let color = contained.color.as_str();
            let n = map.get(color);
            if n == None {
                map.insert(color, HashSet::new());
            }
            let n = map.get_mut(color).expect("Have ensured value is present");
            n.insert(node.0);
        }
    }

    map
}

fn build_graph_from_container_to_contains<'a>(
    nodes: &'a [(&str, Vec<BagVal>)],
) -> HashMap<&'a str, Vec<&'a BagVal>> {
    let mut map = HashMap::new();

    for node in nodes.iter() {
        map.insert(node.0, Vec::from_iter(node.1.iter()));
    }

    map
}

fn count_bags(graph: &HashMap<&str, HashSet<&str>>) -> u64 {
    let mut visited = HashSet::new();
    visited.insert("shiny gold");
    let mut queue = VecDeque::new();
    queue.push_back("shiny gold");

    while let Some(color) = queue.pop_front() {
        let containers = graph.get(color);
        match containers {
            None => {
                continue;
            }
            Some(set) => {
                for &color in set.iter() {
                    if !visited.contains(color) {
                        visited.insert(color);
                        queue.push_back(color);
                    }
                }
            }
        }
    }
    if visited.len() == 0 {
        0
    } else {
        visited.len() as u64 - 1
    }
}

fn count_n_contained_bags(graph: &HashMap<&str, Vec<&BagVal>>, start: &str) -> u64 {
    let mut sum = 0;
    let bags = graph.get(start).expect("Should be present");

    for &bag in bags {
        // count bags themselves and those they contain
        sum += bag.number * (1 + count_n_contained_bags(graph, bag.color.as_str()));
    }

    sum
}

pub fn print_solution() {
    let input = puzzle_input::read_input("day07");

    let vec = split_input_string(&input);
    let graph = build_graph_contained_to_containers(&vec);
    println!("Day07 Solution Part 1: {}", count_bags(&graph));

    let graph = build_graph_from_container_to_contains(&vec);
    println!(
        "Day07 Solution Part 2: {}",
        count_n_contained_bags(&graph, "shiny gold")
    );
}

#[cfg(test)]
mod tests {
    use crate::day07::{
        build_graph_contained_to_containers, build_graph_from_container_to_contains, count_bags,
        count_n_contained_bags, derive_colors, split_input_string,
        split_str_into_container_and_contained_color, BagVal,
    };

    #[test]
    fn test_split_str() {
        let colors = split_str_into_container_and_contained_color(
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        );
        let expected_colors = ("dark orange", "3 bright white bags, 4 muted yellow bags.");
        assert_eq!(colors, expected_colors);
    }

    #[test]
    fn test_derive_colors() {
        let colors = derive_colors("3 bright white bags, 1 muted yellow bag.");
        assert_eq!(
            colors,
            vec![
                "3 bright white bags".parse::<BagVal>().unwrap(),
                "1 muted yellow bag".parse::<BagVal>().unwrap()
            ]
        );
        assert_eq!(derive_colors("no other bags."), Vec::new() as Vec<BagVal>)
    }

    #[test]
    fn test_split_input_string() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let vec = split_input_string(input);

        let expected = vec![
            (
                "light red",
                vec![
                    "1 bright white bag".parse::<BagVal>().unwrap(),
                    "2 muted yellow bags".parse::<BagVal>().unwrap(),
                ],
            ),
            (
                "dark orange",
                vec![
                    "3 bright white bag".parse::<BagVal>().unwrap(),
                    "4 muted yellow bags".parse::<BagVal>().unwrap(),
                ],
            ),
            (
                "bright white",
                vec!["1 shiny gold bags".parse::<BagVal>().unwrap()],
            ),
            (
                "muted yellow",
                vec![
                    "2 shiny gold bags".parse::<BagVal>().unwrap(),
                    "9 faded blue bags".parse::<BagVal>().unwrap(),
                ],
            ),
            (
                "shiny gold",
                vec![
                    "1 dark olive bag".parse::<BagVal>().unwrap(),
                    "2 vibrant plum bags".parse::<BagVal>().unwrap(),
                ],
            ),
            (
                "dark olive",
                vec![
                    "3 faded blue bag".parse::<BagVal>().unwrap(),
                    "4 dotted black bags".parse::<BagVal>().unwrap(),
                ],
            ),
            (
                "vibrant plum",
                vec![
                    "5 faded blue bags".parse::<BagVal>().unwrap(),
                    "6 dotted black bags".parse::<BagVal>().unwrap(),
                ],
            ),
            ("faded blue", vec![]),
            ("dotted black", vec![]),
        ];

        assert_eq!(vec, expected);
    }

    #[test]
    fn test_count_bags() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let vec = split_input_string(input);
        let graph = build_graph_contained_to_containers(&vec);

        assert_eq!(count_bags(&graph), 4);
    }

    #[test]
    fn test_contain_bag() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let vec = split_input_string(input);
        let graph = build_graph_from_container_to_contains(&vec);

        assert_eq!(count_n_contained_bags(&graph, "shiny gold"), 32);
    }
}
