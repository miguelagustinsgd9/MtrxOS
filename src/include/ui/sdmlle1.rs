use crate::include::*;
pub fn ejecutar(sistema: &mut SystemTable<Boot>) {
    let mut ui = sdmll::Sdmll::init(sistema);
    let mut contador: usize = 0;
    let mut pct: usize = 0;
    let mut ventana_abierta = true;
    let mut redibujar = true;

    ui.clear(sistema);

    loop {
        if redibujar {
            ui.set_color(sistema, Color::White, Color::Black);
            ui.clear(sistema);

            ui.draw_status_bar(sistema, "MtrxOS - SDMLL Demo [ESC para salir]");

            if ventana_abierta {
                let (vx, vy, vw, vh) = (10, 5, 42, 12);

                ui.draw_shadow(sistema, vx, vy, vw, vh);

                ui.draw_window(
                    sistema, vx, vy, vw, vh,
                    "System Diagnostics",
                    "Estado: OK\nSDMLL: Modo Directo UEFI\nLade Layer: Cargada\n\n[ENTER] Test  [Arrows] Sel\n[ESC] Volver a Shell",
                    Color::White, Color::Blue
                );

                ui.draw_progress(sistema, vx + 4, vy + 8, vw - 8, pct);

                ui.draw_button(sistema, vx + 4, vy + 10, "Aceptar", contador % 2 == 0, Color::White, Color::Blue);
                ui.draw_button(sistema, vx + 20, vy + 10, "Cerrar", contador % 2 != 0, Color::White, Color::Blue);
            }
            redibujar = false;
        }

        if let Ok(Some(key)) = sistema.stdin().read_key() {
            match key {
                Key::Special(ScanCode::ESCAPE) => {
                    ui.set_color(sistema, Color::White, Color::Black);
                    ui.clear(sistema);
                    return;
                },
                Key::Special(ScanCode::RIGHT) | Key::Special(ScanCode::LEFT) => {
                    contador = contador.wrapping_add(1);
                    redibujar = true;
                }
                Key::Printable(c) if u16::from(c) == 13 => {
                    if ventana_abierta && contador % 2 != 0 {
                        ventana_abierta = false;
                    } else if ventana_abierta {
                        pct = if pct >= 100 { 0 } else { pct + 10 };
                    }
                    redibujar = true;
                }
                Key::Printable(c) if u16::from(c) == 'r' as u16 || u16::from(c) == 'R' as u16 => {
                    if !ventana_abierta {
                        ventana_abierta = true;
                        pct = 0;
                        redibujar = true;
                    }
                }
                _ => {}
            }
        }
        sistema.boot_services().stall(10_000);
    }
}
