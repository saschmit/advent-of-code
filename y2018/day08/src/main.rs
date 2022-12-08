#[derive(Debug)]
struct Node<'a> {
    children : Vec<Node<'a>>,
    metadata : &'a [usize],
}

fn parse_node<'a>(active : &'a [usize]) -> (Node<'a>, usize) {
    let mut start = 0;
    let num_children = active[0];
    let num_metadata = active[1];

    start += 2;
    let mut children = Vec::new();
    for _ in 0..num_children {
        let (child, offset) = parse_node(&active[start..]);
        children.push(child);
        start += offset;
    }

    let this_node = Node {
        children: children,
        metadata: &active[start..start+num_metadata],
    };

    (this_node, start + num_metadata)
}

fn dfs_sum(top : &Node) -> usize {
    top.children.iter().fold(top.metadata.iter().fold(0, |acc, x| acc + x),
        |acc, x| acc + dfs_sum(x))
}

fn get_value(top : &Node) -> usize {
    top.metadata.iter().fold(0, |acc, x| acc +
        if top.children.len() == 0 {
            *x
        } else {
            match top.children.get(*x-1) {
                None => 0,
                Some(node) => get_value(node),
            }
        }
    )
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let buff = String::from_utf8(std::fs::read(file).unwrap()).unwrap();
    let mut nums = Vec::new();
    for num in buff.split_whitespace() {
        nums.push(usize::from_str_radix(num, 10).unwrap());
    }
    let nums = nums;

    let (tree, consumed) = parse_node(&nums[..]);
    assert_eq!(consumed, nums.len());

    println!("Sum of metadata: {}", dfs_sum(&tree));
    println!("Value: {}", get_value(&tree));
}
