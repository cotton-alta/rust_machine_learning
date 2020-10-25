use std::collections::HashMap;
use rand::Rng;

#[derive(Debug)]
struct Agent {
    alpha: i32,
    gamma: i32,
    epsilon: i32,
    actions: String,
    state: String,
    previous_state: String,
    previous_action: String,
    q_values: [i32; 4],
}

impl Agent {
    fn init_q_table(&self) {
    }
    fn init_state(&self) {
    }
    fn act(&self) {
        let mut rng = rand::thread_rng();

        if rng.gen::<i32>() < self.epsilon {
            println!("random action");
        } else {
            println!("max action");
        }
    }
    fn observe(&self) {
    }
}

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

    fn step(&self, action: String) {
        if self.judge_action(action) == false {
            println!("false");
            return
        }
        println!("true");
    }

    fn judge_action(&self, action: String) -> bool {
        false
    }

    fn judge_end(&self) -> bool {
        false
    }

    fn get_reward(&self) -> f64 {
        0.2
    }
}

fn main() {
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
    grid.step(String::from("action"));

    // agentに関する処理
    println!("{:?}", grid);

    let mut agent = Agent {
        alpha: 1,
        gamma: 1,
        epsilon: 1,
        actions: String::from("UP"),
        state: String::from("state"),
        previous_state: String::from("state"),
        previous_action: String::from("DOWN"),
        q_values: [0; 4],
    };

    agent.alpha = 2;

    agent.act();

    println!("{:?}", agent);

    step();
}

fn step() {
    println!("step function");
}
