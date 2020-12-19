use std::collections::{HashSet, HashMap, VecDeque};
use crate::puzzle_input;

fn split_str_into_container_and_contained_color(str: &str) -> (&str, &str) {
    let test: Vec<&str> = str.split(" bags contain ").collect();
    (test[0], test[1])
}

fn split_input_string(str: &str) -> Vec<(&str, Vec<&str>)> {
    let mut vec = Vec::new();
    for line in str.lines() {
        let (color, colors) = split_str_into_container_and_contained_color(line);
        vec.push((color, derive_colors(colors)));
    }
    vec
}

fn derive_colors(str: &str) -> Vec<&str> {
    if "no other bags.".eq(str) {
        return vec![];
    }

    let res = str[..str.len() - 1]
      .split(", ")
      // assume we start with "\d " and ends in " bags" or " bag"
      .map(|str| {
          let end = if str.ends_with("s") { 5 } else { 4 };
          &str[2..str.len() - end]
      })
      .collect();

    res
}

fn build_graph_contained_to_containers<'a>(nodes: &'a [(&str, Vec<&str>)]) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut map = HashMap::new();
    for node in nodes.iter() {
        for &contained in node.1.iter() {
            let n = map.get(contained);
            if n == None {
                map.insert(contained, HashSet::new());
            }
            let n = map.get_mut(contained).expect("Have ensured value is present");
            n.insert(node.0);
        }
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
            None => { continue; }
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
    if visited.len() == 0 { 0 } else { visited.len() as u64 - 1 }
}

pub fn print_solution() {
    let input = puzzle_input::read_input("day07");

    let vec = split_input_string(&input);
    let graph = build_graph_contained_to_containers(&vec);
    println!("Day07 Solution Part 1: {}", count_bags(&graph));
}

#[cfg(test)]
mod tests {
    use crate::day07::{derive_colors, split_str_into_container_and_contained_color, split_input_string, count_bags, build_graph_contained_to_containers};

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
        assert_eq!(colors, vec!["bright white", "muted yellow"]);
        assert_eq!(derive_colors("no other bags."), Vec::new() as Vec<&str>)
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
            ("light red", vec!["bright white", "muted yellow"]),
            ("dark orange", vec!["bright white", "muted yellow"]),
            ("bright white", vec!["shiny gold"]),
            ("muted yellow", vec!["shiny gold", "faded blue"]),
            ("shiny gold", vec!["dark olive", "vibrant plum"]),
            ("dark olive", vec!["faded blue", "dotted black"]),
            ("vibrant plum", vec!["faded blue", "dotted black"]),
            ("faded blue", vec![]),
            ("dotted black", vec![])
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
}
