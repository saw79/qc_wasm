use ecs::EntityId;

pub struct TurnQueue {
    queue: Vec<EntityId>,
    idx: usize,
}

impl TurnQueue {
    pub fn new() -> Self {
        TurnQueue {
            queue: vec![],
            idx: 0,
        }
    }

    pub fn inc(&mut self) {
        self.idx += 1;
        if self.idx >= self.queue.len() {
            self.idx = 0;
        }
    }

    pub fn get(&mut self) -> EntityId {
        self.queue[self.idx]
    }

    pub fn add(&mut self, id: EntityId) {
        self.queue.push(id);
    }

    pub fn remove(&mut self, id: EntityId) {
        match self.get_idx(id) {
            Some(rem_idx) => {
                self.queue.remove(rem_idx);
                if rem_idx < self.idx {
                    self.idx -= 1;
                }
            },
            None => {},
        };
    }

    fn get_idx(&self, id: EntityId) -> Option<usize> {
        for i in 0..self.queue.len() {
            if self.queue[i] == id {
                return Some(i);
            }
        }

        None
    }
}

