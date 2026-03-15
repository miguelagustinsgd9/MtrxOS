use crate::include::*;
pub fn ejecutar(sistema: &mut SystemTable<Boot>) {
    let mut ui = sdmll::Sdmll::init(sistema);
    let mut buffer = [' '; 4096];
    
    let init_text = "Indicaciones:\n- Escriba sus notas aqui.\n- Use Backspace para borrar.\n- Presione ESC para salir.\n\n";
    let mut len: usize = 0;
    for c in init_text.chars() {
        buffer[len] = c;
        len += 1;
    }

    ui.set_color(sistema, Color::White, Color::Black);
    ui.clear(sistema);

    ui.draw_window(
        sistema,
        0,
        0,
        ui.width,
        ui.height,
        " Bloc de Notas ",
        "",
        Color::White,
        Color::Black,
    );

    let min_x = 2;
    let min_y = 2;
    let max_x = ui.width.saturating_sub(3);
    let max_y = ui.height.saturating_sub(3);

    let mut cx = min_x;
    let mut cy = min_y;

    ui.set_color(sistema, Color::White, Color::Black);
    for i in 0..len {
        let ch = buffer[i];
        if ch == '\n' {
            cx = min_x;
            cy += 1;
        } else {
            ui.draw_pixel(sistema, cx, cy, ch);
            cx += 1;
            if cx > max_x {
                cx = min_x;
                cy += 1;
            }
        }
    }

    ui.set_color(sistema, Color::Black, Color::LightGray);
    ui.draw_pixel(sistema, cx, cy, ' ');

    loop {
        if let Ok(Some(key)) = sistema.stdin().read_key() {
            ui.set_color(sistema, Color::White, Color::Black);
            ui.draw_pixel(sistema, cx, cy, ' ');

            match key {
                Key::Special(ScanCode::ESCAPE) => {
                    ui.set_color(sistema, Color::White, Color::Black);
                    ui.clear(sistema);
                    let _ = sistema.stdout().enable_cursor(true);
                    return;
                }
                Key::Printable(c) => {
                    let val = u16::from(c);
                    if val == 8 {
                        if len > 0 {
                            len -= 1;
                            cx = min_x;
                            cy = min_y;
                            for i in 0..len {
                                if buffer[i] == '\n' {
                                    cx = min_x;
                                    cy += 1;
                                } else {
                                    cx += 1;
                                    if cx > max_x {
                                        cx = min_x;
                                        cy += 1;
                                    }
                                }
                            }
                            ui.set_color(sistema, Color::White, Color::Black);
                            ui.draw_pixel(sistema, cx, cy, ' ');
                        }
                    } else if val == 13 {
                        if len < 4096 && cy < max_y {
                            buffer[len] = '\n';
                            len += 1;
                            cx = min_x;
                            cy += 1;
                        }
                    } else {
                        let ch = char::from(c);
                        let next_y = if cx + 1 > max_x { cy + 1 } else { cy };
                        if len < 4096 && next_y <= max_y {
                            buffer[len] = ch;
                            ui.set_color(sistema, Color::White, Color::Black);
                            ui.draw_pixel(sistema, cx, cy, ch);
                            len += 1;
                            cx += 1;
                            if cx > max_x {
                                cx = min_x;
                                cy += 1;
                            }
                        }
                    }
                }
                _ => {}
            }

            if cy <= max_y {
                ui.set_color(sistema, Color::Black, Color::LightGray);
                ui.draw_pixel(sistema, cx, cy, ' ');
            }
        }
        sistema.boot_services().stall(10_000);
    }
}