fn walk(maze: &mut Vec<&str>) {
    println!("{:?}", maze);

    for y in maze.iter() {
        for x in y.chars() {
            print!("{}", x);
            if x == '@' {
                println!("Found!");
                println!("{},{}", x, y);
                break
            }
        }
    }
}

#[test]
fn test_basic() {
    let mut maze = vec!(
        "########################",
        "#f.D.E.e.C.b.A.@.a.B.c.#",
        "######################.#",
        "#d.....................#",
        "########################");

    println!("TEST");
    println!("{:?}", maze);

    walk(&mut maze);
    // 1. Add starting node (@)
    // 2. For each possible direction from available nodes
    //   - walk until letter or crossroad
    //      - if key then move node from unavailable nodes to available nodes
    //   - add new node to graph and:
    //      - if have key add to available nodes
    //      - else to unavailable nodes

}



fn main() {
    println!("Hello, world!");
}
