fn split_str_into_container_and_contained_color(str: &str) -> (&str, &str) {
    let test: Vec<&str> = str.split(" bags contain ").collect();
    (test[0], test[1])
}

fn derive_colors(str: &str) -> Option<Vec<&str>> {
    if "no other bags".eq(str) {
        return None;
    }

    let res = str[..str.len() - 1]
        .split(", ")
        // assume we start with "\d " and ends in " bags" or " bag"
        .map(|str| {
            let end = if str.ends_with("s") { 5 } else { 4 };
            &str[2..str.len() - end]
        })
        .collect();

    Some(res)
}

#[cfg(test)]
mod tests {
    use crate::day07::{derive_colors, split_str_into_container_and_contained_color};

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
        assert_eq!(colors, Some(vec!["bright white", "muted yellow"]));
        assert_eq!(derive_colors("no other bags"), None)
    }
}
