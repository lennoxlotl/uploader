use serde::Serialize;

pub mod fairing;
pub mod index;
pub mod v1;

/// Responsible for displaying the success status of JSON responses
#[derive(Debug, Serialize)]
pub struct SuccessReporter {
    success: bool,
}

impl SuccessReporter {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
