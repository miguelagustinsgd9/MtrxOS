use crate::include::*;
pub fn obtener_nombre_cpu() -> &'static str {
    let res = core::arch::x86_64::__cpuid(0);
    match res.ebx {
        0x756e6547 => "Intel Processor",
        0x68747541 => "AMD Processor",
        _ => "Generic x86_64 CPU",
    }
}
