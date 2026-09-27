use crate::Preview;
use crate::Theme;
use slint::ComponentHandle;

/// Toggles the global theme between light and dark modes at runtime.
pub fn toggle_theme(preview: &Preview) {
    let theme = preview.global::<Theme>();
    theme.set_is_dark(!theme.get_is_dark());
}

/// Sets the global dark mode state.
pub fn set_dark_mode(preview: &Preview, dark: bool) {
    let theme = preview.global::<Theme>();
    theme.set_is_dark(dark);
}

/// Sets whether high-fidelity rendering effects (like shadows and soft gradients) are enabled.
pub fn set_high_fidelity(preview: &Preview, enabled: bool) {
    let theme = preview.global::<Theme>();
    theme.set_enable_high_fidelity(enabled);
}

/// Checks the `SLINT_SLOW_ANIMATIONS` environment variable and slows down transition durations by 10x if set to "1".
pub fn apply_slow_animations_if_needed(preview: &Preview) {
    if std::env::var("SLINT_SLOW_ANIMATIONS").map(|v| v == "1").unwrap_or(false) {
        let theme = preview.global::<Theme>();
        let mut anim = theme.get_animation();
        anim.fast *= 10;
        anim.normal *= 10;
        anim.slow *= 10;
        theme.set_animation(anim);
    }
}
