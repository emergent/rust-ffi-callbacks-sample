fn main() {
    cc::Build::new()
        .flag("-v")
        .file("src/extlib.c")
        .compile("extlib");
}
