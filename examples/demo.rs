use xrrs::{ app::App };

#[cfg_attr(target_os = "android", ndk_glue::main)]
fn main() {
    App::new()
    .run();
}
