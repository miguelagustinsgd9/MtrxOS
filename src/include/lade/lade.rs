use crate::include::*;
pub fn iniciar_lade(sistema: &mut SystemTable<Boot>) {
    let _ = sistema.stdout().enable_cursor(false);
    let (max_cols, max_rows) = if let Ok(Some(modo)) = sistema.stdout().current_mode() {
        (modo.columns(), modo.rows())
    } else { (80, 25) };

    let mut color_fondo = Color::Cyan;
    let mut menu_abierto = false;
    let mut ventana_activa = 1;
    let mut sel_menu = 0;
    let mut redibujar = true;

    loop {
        if redibujar {
            let _ = sistema.stdout().set_color(Color::White, color_fondo);
            let _ = sistema.stdout().clear();
            if ventana_activa == 1 {
                dibujar_ventana(sistema, max_cols, max_rows, 50, 10, "LADE - MtrxOS", "Bienvenido a LADE Desktop.\n[ESC] Menu\n[ENTER] Cerrar", Color::White, Color::Blue);
            }
            if menu_abierto {
                dibujar_ventana(sistema, max_cols, max_rows, 26, 12, "Menu", "", Color::Black, Color::LightGray);
                let mx = (max_cols.saturating_sub(26)) / 2 + 2;
                let my = (max_rows.saturating_sub(12)) / 2 + 2;
                let ops = ["Fondo", "Acerca", "Monitor", "Calculadora", "Reiniciar", "Apagar"];
                for (i, op) in ops.iter().enumerate() {
                    let _ = sistema.stdout().set_cursor_position(mx, my + i);
                    if sel_menu == i { let _ = write!(sistema.stdout(), "> {}", op); }
                    else { let _ = write!(sistema.stdout(), "  {}", op); }
                }
            }
            redibujar = false;
        }

        if let Ok(Some(evento)) = sistema.stdin().read_key() {
            match evento {
                Key::Special(ScanCode::ESCAPE) => {
                    if menu_abierto { menu_abierto = false; }
                    else { menu_abierto = true; sel_menu = 0; }
                    redibujar = true;
                }
                Key::Special(ScanCode::UP) if menu_abierto && sel_menu > 0 => { sel_menu -= 1; redibujar = true; }
                Key::Special(ScanCode::DOWN) if menu_abierto && sel_menu < 5 => { sel_menu += 1; redibujar = true; }
                Key::Printable(key) if u16::from(key) == 13 => {
                    if menu_abierto {
                        menu_abierto = false;
                        let _ = sistema.stdout().set_color(Color::White, color_fondo);
                        let _ = sistema.stdout().clear();

                        match sel_menu {
                            0 => color_fondo = match color_fondo { Color::Cyan => Color::Red, Color::Red => Color::Green, _ => Color::Cyan },
                            1 => ventana_activa = 1,
                            2 => monitor::ejecutar(sistema, max_cols, max_rows),
                            3 => calculadora::ejecutar(sistema, max_cols, max_rows),
                            4 => sistema.runtime_services().reset(ResetType::COLD, uefi::Status::SUCCESS, None),
                            5 => sistema.runtime_services().reset(ResetType::SHUTDOWN, uefi::Status::SUCCESS, None),
                            _ => {}
                        }
                    } else { ventana_activa = 0; }
                    redibujar = true;
                }
                _ => {}
            }
        }
    }
}

pub fn dibujar_ventana(sistema: &mut SystemTable<Boot>, mc: usize, mr: usize, an: usize, al: usize, tit: &str, txt: &str, fg: Color, bg: Color) {
    let x = (mc.saturating_sub(an)) / 2;
    let y = (mr.saturating_sub(al)) / 2;
    let _ = sistema.stdout().set_color(Color::Black, Color::Black);
    for i in (y + 1)..=(y + al) { let _ = sistema.stdout().set_cursor_position(x + 1, i); for _ in 0..an { let _ = write!(sistema.stdout(), " "); } }
    let _ = sistema.stdout().set_color(fg, bg);
    for i in y..(y + al) { let _ = sistema.stdout().set_cursor_position(x, i); for _ in 0..an { let _ = write!(sistema.stdout(), " "); } }
    let _ = sistema.stdout().set_cursor_position(x, y);
    let _ = write!(sistema.stdout(), "┌");
    for _ in 0..(an.saturating_sub(2)) { let _ = write!(sistema.stdout(), "─"); }
    let _ = write!(sistema.stdout(), "┐");
    for i in 1..(al - 1) {
        let _ = sistema.stdout().set_cursor_position(x, y + i);
        let _ = write!(sistema.stdout(), "│");
        let _ = sistema.stdout().set_cursor_position(x + an - 1, y + i);
        let _ = write!(sistema.stdout(), "│");
    }
    let _ = sistema.stdout().set_cursor_position(x, y + al - 1);
    let _ = write!(sistema.stdout(), "└");
    for _ in 0..(an.saturating_sub(2)) { let _ = write!(sistema.stdout(), "─"); }
    let _ = write!(sistema.stdout(), "┘");

    let t_len = tit.len();
    let t_pos = x + (an / 2).saturating_sub(t_len / 2);
    let _ = sistema.stdout().set_cursor_position(t_pos, y);
    let _ = write!(sistema.stdout(), "{} ", tit);

    for (i, line) in txt.split('\n').enumerate() {
        let _ = sistema.stdout().set_cursor_position(x + 2, y + 2 + i);
        let _ = write!(sistema.stdout(), "{}", line);
    }
}
