pub struct Process {
    id: u32,
    state: ProcessState,
}

pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

impl Process {
    pub fn new(id: u32) -> Self {
        Process {
            id,
            state: ProcessState::Ready,
        }
    }

    pub fn run(&mut self) {
        self.state = ProcessState::Running;
    }

    pub fn terminate(&mut self) {
        self.state = ProcessState::Terminated;
    }
}