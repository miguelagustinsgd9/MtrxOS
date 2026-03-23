use crate::include::*;

pub fn reproducir(sistema: &mut SystemTable<Boot>) {
    let melodia: [u32; 64] = [
        330, 392, 440, 330, 330, 392, 440, 330,
        349, 440, 523, 349, 349, 440, 523, 349,
        294, 349, 440, 294, 294, 349, 440, 294,
        262, 330, 392, 262, 262, 330, 392, 523,
        660, 784, 880, 660, 660, 784, 880, 660,
        698, 880, 1046, 698, 698, 880, 1046, 698,
        587, 698, 880, 587, 587, 698, 880, 587,
        523, 659, 784, 523, 523, 440, 392, 330
    ];

    let _ = writeln!(sistema.stdout(), "Cargando: MtrxOS Hymn");

    for (i, nota) in melodia.iter().enumerate() {
        sound::emitir_beep(*nota);
        
        let duracion = if i % 4 == 3 {
            300_000
        } else {
            150_000
        };
        
        sistema.boot_services().stall(duracion);
        sound::silenciar();
        
        let pausa = if i % 4 == 3 { 100_000 } else { 30_000 };
        sistema.boot_services().stall(pausa);
    }

    let _ = writeln!(sistema.stdout(), "Listo.");
}