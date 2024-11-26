#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_creation() {
        let process = Process::new(1);
        assert_eq!(process.id, 1);
        assert!(matches!(process.state, ProcessState::Ready));
    }

    #[test]
    fn test_process_state_transition() {
        let mut process = Process::new(1);
        process.run();
        assert!(matches!(process.state, ProcessState::Running));
        process.terminate();
        assert!(matches!(process.state, ProcessState::Terminated));
    }
}