pub fn install_panic_hook() {
    std::panic::set_hook(Box::new(|_| {
        eprintln!("humanizar-nucleo: falha inesperada no processo")
    }));
}
