use std::collections::HashMap;
use rand::Rng;

#[derive(Debug)]
struct Agent {
    alpha: i32,
    gamma: i32,
    epsilon: i32,
    actions: String,
    state: [i32; 2],
    reward_history: Vec<i32>,
    previous_state: [i32; 2],
    previous_action: i32,
    q_values: HashMap<String, [i32; 4]>,
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

    fn observe(&mut self, next_state: [i32; 2], reward: i32) {
        let next_key = next_state[0].to_string() + &next_state[1].to_string();
        if self.q_values.contains_key(&next_key) {
           self.q_values.insert(next_key, [0; 4]);
        }
        self.previous_state = self.state;
        self.state = next_state;

        self.reward_history.push(reward);
        self.learn(reward);
    }

    fn learn(&mut self, reward: i32) {
        println!("learn");
        let previous_state_key = self.previous_state[0].to_string() + &self.previous_state[1].to_string();
        let previous_action_key = self.previous_action as usize;
        let state_key = self.state[0].to_string() + &self.state[1].to_string();
        let q_value = self.q_values[&previous_state_key][previous_action_key];
        let mut max_q: i32 = 0;

        for &value in self.q_values[&state_key].iter() {
            if value > max_q {
                max_q = value;
            }
        }
        self.q_values.get_mut(&previous_state_key).unwrap()[previous_action_key] = q_value + (self.alpha * (reward + (self.gamma * max_q) - q_value));
    }
}

#[derive(Debug)]
struct Grid {
    field: [[i32; 4]; 5],
    field_type: HashMap<String, i32>,
    actions: HashMap<String, i32>,
    agent_position: [i32; 2],
    start_position: [i32; 2],
}

impl Grid {
    fn position_change(&mut self, x: i32, y: i32) {
        self.agent_position = [x, y]
    }

    fn step(&mut self, action: i32) -> ([i32; 2], i32, bool) {
        if self.judge_action(action) == false {
            return (self.agent_position, -1, false)
        }

        if action == self.actions["up"] {
            self.agent_position[1] += 1;
        } else if action == self.actions["down"] {
            self.agent_position[1] -= 1;
        } else if action == self.actions["left"] {
            self.agent_position[0] -= 1;
        } else {
            self.agent_position[0] += 1;
        }

        let goal = self.judge_end();
        let reward = self.get_reward();
        return (self.agent_position, reward, goal)
    }

    fn judge_action(&self, action: i32) -> bool {
        let mut agent_position = self.agent_position;

        if action == self.actions["up"] {
            agent_position[1] += 1;
        } else if action == self.actions["down"] {
            agent_position[1] -= 1;
        } else if action == self.actions["left"] {
            agent_position[0] -= 1;
        } else {
            agent_position[0] += 1;
        }

        if 20 <= agent_position[1] || 0 > agent_position[1] {
            return false
        }

        if 4 < agent_position[0] || 0 > agent_position[0] {
            return false
        }

        if self.judge_wall() {
            return false
        }
        true
    }

    fn judge_wall(&self) -> bool {
        false
    }

    fn judge_end(&self) -> bool {
        let x: usize = self.agent_position[0] as usize;
        let y: usize = self.agent_position[1] as usize;

        if self.field[x][y] == self.field_type["goal"] {
            return true
        }
        false
    }

    fn get_reward(&self) -> i32 {
        let x: usize = self.agent_position[0] as usize;
        let y: usize = self.agent_position[1] as usize;

        if self.field[x][y] == self.field_type["goal"] {
            10
        } else if self.field[x][y] == self.field_type["trap"] {
            -10
        } else {
            0
        }
    }

    fn reset(&mut self) {
        self.agent_position = self.start_position;
    }
}

fn main() {
    // gridに関する処理
    let mut grid = Grid {
        field: [[0; 4]; 5],
        field_type: HashMap::new(),
        actions: HashMap::new(),
        agent_position: [0, 4],
        start_position: [0, 0],
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
    grid.step(1);

    // agentに関する処理
    println!("{:?}", grid);

    let mut agent = Agent {
        alpha: 1,
        gamma: 1,
        epsilon: 1,
        actions: String::from("UP"),
        state: [0, 0],
        reward_history: vec![],
        previous_state: [0, 0],
        previous_action: 0,
        q_values: HashMap::new(),
    };

    agent.alpha = 2;

    agent.act();

    println!("{:?}", agent);

    step();
}

fn step() {
    println!("step function");
}
