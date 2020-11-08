use std::collections::HashMap;
use rand::Rng;

#[derive(Debug)]
struct Agent {
    alpha: f64,
    gamma: f64,
    epsilon: f64,
    actions: String,
    state: [i32; 2],
    reward_history: Vec<f64>,
    previous_state: [i32; 2],
    previous_action: i32,
    q_values: HashMap<String, [f64; 4]>,
}

impl Agent {
    fn init_q_table(&mut self, start_position: [i32; 2]) {
        let start_key = start_position[0].to_string() + &start_position[1].to_string();
        self.q_values = HashMap::new();
        self.q_values.insert(start_key, [0.0, 0.0, 0.0, 0.0]);
    }

    fn init_state(&self) {
    }

    fn act(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut action: i32 = 0;

        if rng.gen::<f64>() < self.epsilon {
            action = rng.gen_range(0, 4);
        } else {
            let state_key = self.state[0].to_string() + &self.state[1].to_string();
            let mut count: i32 = rng.gen_range(0, 4);
            let mut max_value = self.q_values[&state_key][count as usize];
            let mut max_key: i32 = count;
            for value in self.q_values[&state_key].iter() {
                if count > 3 {
                    count -= 4;
                }
                if *value > max_value {
                    max_value = *value;
                    max_key = count;
                }
                count += 1;
            }
            action = max_key;
        }
        self.previous_action = action;
        action
    }

    fn observe(&mut self, next_state: [i32; 2], reward: f64) {
        let next_key = next_state[0].to_string() + &next_state[1].to_string();
        if self.q_values.contains_key(&next_key) == false {
           self.q_values.insert(next_key, [0.0; 4]);
        }
        self.previous_state = self.state;
        self.state = next_state;

        if reward != -1000.0 {
            self.reward_history.push(reward);
            self.learn(reward);
        }
    }

    fn learn(&mut self, reward: f64) {
        let previous_state_key = self.previous_state[0].to_string() + &self.previous_state[1].to_string();
        let previous_action_key = self.previous_action as usize;
        let state_key = self.state[0].to_string() + &self.state[1].to_string();
        let q_value = self.q_values[&previous_state_key][previous_action_key];
        let mut max_q: f64 = self.q_values[&state_key][0];

        for value in self.q_values[&state_key].iter() {
            if *value > max_q {
                max_q = *value;
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

    fn step(&mut self, action: i32) -> ([i32; 2], f64, bool) {
        if self.judge_action(action) == false {
            return (self.agent_position, -1.0, false)
        }

        if action == self.actions["up"] {
            self.agent_position[1] -= 1;
        } else if action == self.actions["down"] {
            self.agent_position[1] += 1;
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
            agent_position[1] -= 1;
        } else if action == self.actions["down"] {
            agent_position[1] += 1;
        } else if action == self.actions["left"] {
            agent_position[0] -= 1;
        } else {
            agent_position[0] += 1;
        }

        if 4 < agent_position[1] || 0 > agent_position[1] {
            return false
        }

        if 3 < agent_position[0] || 0 > agent_position[0] {
            return false
        }

        if self.judge_wall(agent_position) {
            return false
        }
        true
    }

    fn judge_wall(&self, agent_position: [i32; 2]) -> bool {
        let x = agent_position[0] as usize;
        let y = agent_position[1] as usize;
        if self.field[y][x] == self.field_type["wall"] {
            return true
        }
        false
    }

    fn judge_end(&self) -> bool {
        let x: usize = self.agent_position[0] as usize;
        let y: usize = self.agent_position[1] as usize;

        if self.field[y][x] == self.field_type["goal"] {
            return true
        } else if self.field[y][x] == self.field_type["trap"] {
            return true
        }
        false
    }

    fn get_reward(&self) -> f64 {
        let x: usize = self.agent_position[0] as usize;
        let y: usize = self.agent_position[1] as usize;

        if self.field[y][x] == self.field_type["goal"] {
            100.0
        } else if self.field[y][x] == self.field_type["trap"] {
            -100.0
        } else {
            0.0
        }
    }

    fn reset(&mut self) -> [i32; 2] {
        self.agent_position = self.start_position;
        self.agent_position
    }
}

fn main() {
    // gridに関する処理
    let mut grid = Grid {
        field: [
            [0, 2, 0, 1],
            [0, 2, 0, 2],
            [0, 3, 0, 0],
            [0, 0, 2, 0],
            [3, 0, 0, 0]
        ],
        field_type: HashMap::new(),
        actions: HashMap::new(),
        agent_position: [0, 0],
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

    // agentに関する処理
    let mut agent = Agent {
        alpha: 0.2,
        gamma: 0.99,
        epsilon: 0.05,
        actions: String::from("up"),
        state: [0, 0],
        reward_history: vec![],
        previous_state: [0, 0],
        previous_action: 0,
        q_values: HashMap::new(),
    };

    agent.q_values.insert(String::from("00"), [0.0, 0.0, 0.0, 0.0]);

    // learning
    let epochs = 500;
    let mut judge_end = false;
    let mut reward: Vec<f64> = Vec::new();
    for epoch in 1..=epochs {
        let mut epoch_reward: Vec<f64> = Vec::new();
        let mut count = 0;
        while judge_end == false {
            let action = agent.act();
            let (state, reward, inner_judge_end) = grid.step(action);
            judge_end = inner_judge_end;
            agent.observe(state, reward);
            epoch_reward.push(reward);
            count += 1;
        }
        println!("{}", count);
        let state = grid.reset();
        agent.observe(state, -1000.0);
        judge_end = false;
        reward.push(epoch_reward.iter().sum());
    }
}
