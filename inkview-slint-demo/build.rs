fn main() {
    println!("cargo:EMBED_TEXTURES=1");

    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
    slint_build::print_rustc_flags().unwrap();
}
