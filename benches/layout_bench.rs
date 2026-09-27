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

fn main() {
    init_test_backend();
    let preview = Preview::new().unwrap();

    // Warm up iteration
    for i in 0..500 {
        preview.set_test_slider_value(i as f32 % 100.0);
        let _val = preview.get_test_progress();
    }

    let iterations = 10000;
    let start = Instant::now();

    for i in 0..iterations {
        // Trigger a value update which dirties the binding graph
        preview.set_test_slider_value((i % 100) as f32);
        // Accessing the dependent property forces re-evaluation of: test_progress_bar.progress: test_slider.value / 100.0
        let val = preview.get_test_progress();
        std::hint::black_box(val);
    }

    let duration = start.elapsed();
    let per_iteration = duration / iterations as u32;

    println!("Completed {} iterations.", iterations);
    println!("Total duration: {:?}", duration);
    println!("Average time per layout re-evaluation: {:?}", per_iteration);

    assert!(
        per_iteration.as_secs_f64() < 0.0015,
        "Layout binding evaluation took longer than 1.5ms limit: {:?}",
        per_iteration
    );
    println!("Benchmark success: evaluation takes < 1.5ms.");
}
