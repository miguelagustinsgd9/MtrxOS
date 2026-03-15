use crate::include::*;
pub fn emitir_beep(frecuencia: u32) {
    if frecuencia == 0 { return; }
    let divisor = 1193180 / frecuencia;
    unsafe {
        asm!("out 0x43, al", in("al") 0xb6u8);
        asm!("out 0x42, al", in("al") (divisor & 0xff) as u8);
        asm!("out 0x42, al", in("al") ((divisor >> 8) & 0xff) as u8);
        let mut tmp: u8;
        asm!("in al, 0x61", out("al") tmp);
        asm!("out 0x61, al", in("al") tmp | 0x03);
    }
}

pub fn silenciar() {
    unsafe {
        let mut tmp: u8;
        asm!("in al, 0x61", out("al") tmp);
        asm!("out 0x61, al", in("al") tmp & 0xfc);
    }
}
