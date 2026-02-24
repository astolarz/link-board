fn main() {
    #[cfg(feature="esp32")]
    embuild::espidf::sysenv::output();
}
