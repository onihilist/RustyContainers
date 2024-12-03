use crate::core::container::RCContainer;
use crate::core::manager;
use crate::core::manager::loader::RCAction;

impl RCContainer {
    pub(crate) fn stop(&self) -> &Self {
        manager::loader::call_cmd(&self.name.clone(), RCAction::STOP);
        self
    }

    pub(crate) fn start(&self) -> &Self {
        manager::loader::call_cmd(&self.name.clone(), RCAction::START);
        self
    }

    pub(crate) fn pause(&self) -> &Self {
        manager::loader::call_cmd(&self.name.clone(), RCAction::PAUSE);
        self
    }

    pub(crate) fn resume(&self) -> &Self {
        manager::loader::call_cmd(&self.name.clone(), RCAction::RESUME);
        self
    }

    pub(crate) fn discard(&self) -> &Self {
        manager::loader::call_cmd(&self.name.clone(), RCAction::DISCARD);
        self
    }

    pub(crate) fn prune_all(&self) -> &Self {
        manager::loader::call_cmd(&self.name.clone(), RCAction::PRUNE_ALL);
        self
    }
}
