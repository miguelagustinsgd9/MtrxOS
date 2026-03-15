use crate::include::*;
pub fn ejecutar(sistema: &mut SystemTable<Boot>) {
    let mut ui = sdmll::Sdmll::init(sistema);
    
    ui.set_color(sistema, Color::White, Color::Black);
    ui.clear(sistema);

    ui.draw_window(
        sistema,
        0,
        0,
        ui.width,
        ui.height,
        " Sound Maker ",
        "",
        Color::White,
        Color::Black,
    );

    let notas = [
        ("DO", 261), ("RE", 293), ("MI", 329), ("FA", 349),
        ("SOL", 392), ("LA", 440), ("SI", 493), ("DO+", 523),
    ];

    ui.draw_text(sistema, 4, 4, "Teclas 1-8: Tocar notas | ESC: Salir");

    loop {
        if let Ok(Some(key)) = sistema.stdin().read_key() {
            match key {
                Key::Special(ScanCode::ESCAPE) => {
                    sound::silenciar();
                    ui.clear(sistema);
                    return;
                }
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ch >= '1' && ch <= '8' {
                        let idx = (ch as usize) - ('1' as usize);
                        let (nombre, frec) = notas[idx];
                        
                        ui.set_color(sistema, Color::Black, Color::LightGray);
                        ui.draw_text(sistema, 4, 8, " NOTA ACTUAL: ");
                        ui.draw_text(sistema, 18, 8, nombre);
                        
                        sound::emitir_beep(frec);
                        sistema.boot_services().stall(150_000);
                        sound::silenciar();
                        
                        ui.set_color(sistema, Color::White, Color::Black);
                        ui.draw_text(sistema, 4, 8, "                          ");
                    }
                }
                _ => {}
            }
        }
        sistema.boot_services().stall(10_000);
    }
}