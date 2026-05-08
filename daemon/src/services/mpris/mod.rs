use crate::states::SharedState;

pub mod player_interface;
pub mod root_interface;

pub struct Mpris {
    pub shared_state: SharedState,
}
