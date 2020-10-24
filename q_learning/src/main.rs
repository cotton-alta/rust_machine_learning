use std::collections::HashMap;

fn main() {
    #[derive(Debug)]
    struct Grid {
        field: [[i32; 4]; 5],
        field_type: HashMap<String, i32>,
        actions: HashMap<String, i32>,
        agent_position: [i32; 2],
    }

    impl Grid {
        fn position_change(&mut self, x: i32, y: i32) {
            self.agent_position = [x, y]
        }
    }

    // gridに関する処理
    let mut grid = Grid {
        field: [[0; 4]; 5],
        field_type: HashMap::new(),
        actions: HashMap::new(),
        agent_position: [0, 4],
    };

    grid.field_type.insert(String::from("normal"), 0);
    grid.field_type.insert(String::from("goal"), 1);
    grid.field_type.insert(String::from("wall"), 2);
    grid.field_type.insert(String::from("trap"), 3);

    grid.actions.insert(String::from("up"), 0);
    grid.actions.insert(String::from("down"), 1);
    grid.actions.insert(String::from("left"), 2);
    grid.actions.insert(String::from("right"), 3);

    grid.position_change(1, 1);

    // agentに関する処理
    println!("{:?}", grid);
    step();
}

fn step() {
    println!("step function");
}
