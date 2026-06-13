use my_slint_kit::Preview;
use std::time::Instant;
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
fn test_foundation_headless_preview() {
    init_test_backend();
    
    // Warm up the rendering engine and window adapter initialization
    let _warmup = Preview::new().unwrap();

    let start = Instant::now();
    let _preview = Preview::new().unwrap();
    let duration = start.elapsed();

    // Verify setup time is under 15ms
    assert!(duration.as_millis() <= 15, "Setup took too long: {}ms", duration.as_millis());
}

#[test]
fn test_button_properties() {
    init_test_backend();
    let preview = Preview::new().unwrap();
    
    // Default disabled state should be false
    assert_eq!(preview.get_test_button_disabled(), false);
    
    // Set to disabled
    preview.set_test_button_disabled(true);
    assert_eq!(preview.get_test_button_disabled(), true);
    
    // Check outlined property
    assert_eq!(preview.get_test_button_outlined(), false);
    preview.set_test_button_outlined(true);
    assert_eq!(preview.get_test_button_outlined(), true);
}
