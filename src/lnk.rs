use ableton_link::*;

pub fn init() -> Link {
    let mut link = Link::new(110.0);
    link.enable(true);
    link.enable_start_stop_sync(true);
    link
}
