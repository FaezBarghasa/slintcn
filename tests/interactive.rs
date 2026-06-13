use my_slint_kit::Preview;
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
fn test_interactive_components() {
    init_test_backend();
    let preview = Preview::new().unwrap();

    // 1. Text input bidirectional bindings
    preview.set_test_input_text("Hello Slint".into());
    assert_eq!(preview.get_test_input_text(), "Hello Slint");

    // 2. Checkbox toggling
    assert_eq!(preview.get_test_checkbox_checked(), true);
    preview.set_test_checkbox_checked(false);
    assert_eq!(preview.get_test_checkbox_checked(), false);

    // 3. Switch toggling
    assert_eq!(preview.get_test_switch_checked(), true);
    preview.set_test_switch_checked(false);
    assert_eq!(preview.get_test_switch_checked(), false);

    // 4. Slider updates and proportional progress bar evaluation
    assert_eq!(preview.get_test_slider_value(), 50.0);
    assert_eq!(preview.get_test_progress(), 0.5);

    preview.set_test_slider_value(80.0);
    assert_eq!(preview.get_test_slider_value(), 80.0);
    // Verifying automatic binding propagation of progress: test_slider.value / 100.0
    assert_eq!(preview.get_test_progress(), 0.8);

    // 5. Radio group selection
    assert_eq!(preview.get_test_radio_selected(), 0);
    preview.set_test_radio_selected(2);
    assert_eq!(preview.get_test_radio_selected(), 2);

    // 6. Tab switching
    assert_eq!(preview.get_test_tab_active(), 0);
    preview.set_test_tab_active(1);
    assert_eq!(preview.get_test_tab_active(), 1);
}
