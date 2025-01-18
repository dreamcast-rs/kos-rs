pub fn border_color(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let old_color = unsafe { crate::ffi::dc::video::vid_border_color(r, g, b) };
    (((old_color >> 16) & 0xFF) as u8, ((old_color >> 8) & 0xFF) as u8, (old_color & 0xFF) as u8)
}
