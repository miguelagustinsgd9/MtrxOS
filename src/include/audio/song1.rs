use crate::include::*;
pub fn reproducir(sistema: &mut SystemTable<Boot>) {
    let notas: [u32; 8] = [261, 293, 329, 349, 392, 440, 493, 523];

    writeln!(sistema.stdout(), "Reproduciendo song1...").unwrap();

    for nota in notas.iter() {
        sound::emitir_beep(*nota);
        sistema.boot_services().stall(150_000);
        sound::silenciar();
        sistema.boot_services().stall(50_000);
    }
}
