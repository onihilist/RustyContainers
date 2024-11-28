use crate::core::container::RCContainer;
use crate::core::manager;
use crate::core::manager::loader::RCAction;

impl RCContainer {
    pub(crate) fn stop(&self) -> &Self {
        manager::loader::call_cmd(&self.id.clone().unwrap(),RCAction::STOP);
        self
    }

    pub(crate) fn start(&self) {
        // Start the container
    }

    pub(crate) fn pause(&self) {
        // Pause the container
    }

    pub(crate) fn resume(&self) {
        // Resume the container
    }

    pub(crate) fn discard(&self) {
        // Delete the container
    }
}
