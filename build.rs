fn main() {
    slint_build::compile("ui/app.slint").unwrap();

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("fragrance_icon.ico");
        res.compile().expect("Failed to compile Windows resources");
    }
}