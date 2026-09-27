fn main() {
    let preview = my_slint_kit::Preview::new().unwrap();
    my_slint_kit::theme::apply_slow_animations_if_needed(&preview);
    preview.run().unwrap();
}
