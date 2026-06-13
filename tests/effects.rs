use my_slint_kit::Preview;
use my_slint_kit::Theme;
use slint::ComponentHandle;
use std::sync::Once;

static INIT: Once = Once::new();

fn init_test_backend() {
    INIT.call_once(|| {
        struct TestPlatform {
            window: std::rc::Rc<slint::platform::software_renderer::MinimalSoftwareWindow>,
        }
        impl slint::platform::Platform for TestPlatform {
            fn create_window_adapter(
                &self,
            ) -> Result<std::rc::Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
                Ok(self.window.clone())
            }
        }
        let window = slint::platform::software_renderer::MinimalSoftwareWindow::new(
            slint::platform::software_renderer::RepaintBufferType::ReusedBuffer
        );
        let _ = slint::platform::set_platform(std::boxed::Box::new(TestPlatform { window }));
    });
}

#[test]
fn test_theme_switching_runtime() {
    init_test_backend();
    let preview = Preview::new().unwrap();
    let theme = preview.global::<Theme>();

    // Assert defaults
    assert_eq!(theme.get_is_dark(), false);

    // Assert toggle_theme
    my_slint_kit::theme::toggle_theme(&preview);
    assert_eq!(theme.get_is_dark(), true);

    // Assert set_dark_mode
    my_slint_kit::theme::set_dark_mode(&preview, false);
    assert_eq!(theme.get_is_dark(), false);
}

#[test]
fn test_high_fidelity_fallback() {
    init_test_backend();
    let preview = Preview::new().unwrap();
    let theme = preview.global::<Theme>();

    // Assert defaults
    assert_eq!(theme.get_enable_high_fidelity(), true);

    // Disable high fidelity and verify state change
    my_slint_kit::theme::set_high_fidelity(&preview, false);
    assert_eq!(theme.get_enable_high_fidelity(), false);
}

#[test]
fn test_slow_animations_env() {
    init_test_backend();
    
    // Set env variable
    std::env::set_var("SLINT_SLOW_ANIMATIONS", "1");

    let preview = Preview::new().unwrap();
    let theme = preview.global::<Theme>();

    // Assert default fast animation is 150ms before check
    assert_eq!(theme.get_animation().fast, 150);

    // Apply env check
    my_slint_kit::theme::apply_slow_animations_if_needed(&preview);

    // Verify fast animation is now slowed down 10x (1500ms)
    assert_eq!(theme.get_animation().fast, 1500);

    // Clean up env
    std::env::remove_var("SLINT_SLOW_ANIMATIONS");
}
