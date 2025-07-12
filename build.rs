fn main() {
    css_mod::Compiler::new()
        .add_modules("src/**/*.css")
        .unwrap()
        .compile("styles.generated.css")
        .unwrap();
}
