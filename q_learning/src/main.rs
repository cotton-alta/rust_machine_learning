use std::collections::HashMap;

fn main() {
    // gridに関する処理
    let grid: [[i32; 4]; 5] = [[0; 4]; 5];
    println!("{:?}", grid);

    let mut field_type = HashMap::new();
    field_type.insert("normal", 0);
    field_type.insert("goal", 1);
    field_type.insert("wall", 2);
    field_type.insert("trap", 3);

    let mut actions = HashMap::new();
    actions.insert("up", 0);
    actions.insert("down", 1);
    actions.insert("left", 2);
    actions.insert("right", 3);

    let start_position = [0, 4];
    let agent_position = [0, 4];

    // agentに関する処理
}

fn step() {

}
