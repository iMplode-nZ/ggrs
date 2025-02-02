use bincode;
use rand::{prelude::ThreadRng, thread_rng, Rng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use ggrs::{Frame, GGRSRequest, GameInput, GameState, GameStateCell};

pub const INPUT_SIZE: usize = std::mem::size_of::<u32>();
pub const MAX_PRED_FRAMES: usize = 8;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub struct GameStub {
    pub gs: GameStateStub,
}

impl GameStub {
    #[allow(dead_code)]
    pub fn new() -> GameStub {
        GameStub {
            gs: GameStateStub { frame: 0, state: 0 },
        }
    }

    #[allow(dead_code)]
    pub fn handle_requests(&mut self, requests: Vec<GGRSRequest<GameStateStub>>) {
        for request in requests {
            match request {
                GGRSRequest::LoadGameState { cell, .. } => self.load_game_state(cell),
                GGRSRequest::SaveGameState { cell, frame } => self.save_game_state(cell, frame),
                GGRSRequest::AdvanceFrame { inputs } => self.advance_frame(inputs),
            }
        }
    }

    fn save_game_state(&mut self, cell: GameStateCell<GameStateStub>, frame: Frame) {
        assert_eq!(self.gs.frame, frame);
        let checksum = calculate_hash(&self.gs);
        let game_state: GameState<GameStateStub> =
            GameState::<GameStateStub>::new_with_checksum(frame, Some(self.gs), checksum);
        cell.save(game_state);
    }

    fn load_game_state(&mut self, cell: GameStateCell<GameStateStub>) {
        let game_state: GameState<GameStateStub> = cell.load();
        self.gs = game_state.data.unwrap();
    }

    fn advance_frame(&mut self, inputs: Vec<GameInput>) {
        self.gs.advance_frame(inputs);
    }
}

pub struct RandomChecksumGameStub {
    pub gs: GameStateStub,
    rng: ThreadRng,
}

impl RandomChecksumGameStub {
    #[allow(dead_code)]
    pub fn new() -> RandomChecksumGameStub {
        RandomChecksumGameStub {
            gs: GameStateStub { frame: 0, state: 0 },
            rng: thread_rng(),
        }
    }

    #[allow(dead_code)]
    pub fn handle_requests(&mut self, requests: Vec<GGRSRequest<GameStateStub>>) {
        for request in requests {
            match request {
                GGRSRequest::LoadGameState { cell, .. } => self.load_game_state(cell),
                GGRSRequest::SaveGameState { cell, frame } => self.save_game_state(cell, frame),
                GGRSRequest::AdvanceFrame { inputs } => self.advance_frame(inputs),
            }
        }
    }

    fn save_game_state(&mut self, cell: GameStateCell<GameStateStub>, frame: Frame) {
        assert_eq!(self.gs.frame, frame);

        let random_checksum: u64 = self.rng.gen();
        cell.save(GameState::new_with_checksum(
            frame,
            Some(self.gs),
            random_checksum,
        ));
    }

    fn load_game_state(&mut self, cell: GameStateCell<GameStateStub>) {
        self.gs = cell.load().clone().data.expect("No data found.");
    }

    fn advance_frame(&mut self, inputs: Vec<GameInput>) {
        self.gs.advance_frame(inputs);
    }
}

#[derive(Default, Copy, Clone, Hash)]
pub struct GameStateStub {
    pub frame: i32,
    pub state: i32,
}

impl GameStateStub {
    fn advance_frame(&mut self, inputs: Vec<GameInput>) {
        let p0_inputs: u32 = bincode::deserialize(&inputs[0].buffer).unwrap();
        let p1_inputs: u32 = bincode::deserialize(&inputs[0].buffer).unwrap();

        if (p0_inputs + p1_inputs) % 2 == 0 {
            self.state += 2;
        } else {
            self.state -= 1;
        }
        self.frame += 1;
    }
}
