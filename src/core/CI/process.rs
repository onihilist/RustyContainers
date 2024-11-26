use crate::core::container::RCContainer;

impl RCContainer {

    pub(crate) fn before_each(mut self, command: &str) -> Self {
        //log into the container and do the command before user use it
        self
    }

    pub(crate) fn after_each(mut self, command: &str) -> Self {
        //log into the container and do the command after user use it
        self
    }

    pub(crate) fn before(mut self, command: &str) -> Self {
        //log into the container at the start before the first one container
        self
    }

    pub(crate) fn after(mut self, command: &str) -> Self {
        //log into the container at the start after the last one container
        self
    }
}
