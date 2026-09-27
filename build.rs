use std::env;
use std::path::PathBuf;

fn main() {
    let mut config = slint_build::CompilerConfiguration::new();
    
    if env::var("CARGO_FEATURE_SOFTWARE_RENDERER").is_ok() {
        config = config.embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    } else {
        config = config.embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
    }
    
    if let Ok(custom_theme) = env::var("USE_CUSTOM_THEME") {
        config = config.with_include_paths(vec![PathBuf::from(custom_theme)]);
    }

    slint_build::compile_with_config("ui/preview.slint", config).unwrap();
}
