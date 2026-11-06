//! ACPI power management states.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepState { S0, S1, S2, S3, S4, S5 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState { Working, Sleeping, SoftOff, MechanicalOff }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sleep_states() {
        for s in [SleepState::S0, SleepState::S3, SleepState::S5] {
            let _ = s;
        }
    }
}
