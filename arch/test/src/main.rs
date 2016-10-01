/// This function is where the kernel sets up IRQ handlers
/// It is increcibly unsafe, and should be minimal in nature

extern "C" {
    fn kmain() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn kstart() -> ! {
    kmain();
}
