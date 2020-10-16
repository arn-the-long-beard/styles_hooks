mod ev_handlers;


mod seed_bind;
mod update_el;
mod utils;
mod reactive_enhancements;
pub use ev_handlers::StateAccessEventHandlers;
pub use seed_bind::{UpdateElLocal, InputBind};
pub use update_el::{StateAccessUpdateEl, LocalUpdateEl2};
pub use utils::{
    after_render, after_render_once, get_html_element_by_id, //handle_unmount,
    request_animation_frame,
};
pub use reactive_enhancements::ReactiveEnhancements;

pub use atomic_hooks::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
