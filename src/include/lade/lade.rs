use crate::include::*;

pub fn iniciar_lade(sistema: &mut SystemTable<Boot>) {
    let _ = sistema.stdout().enable_cursor(false);
    let (mc, mr) = if let Ok(Some(modo)) = sistema.stdout().current_mode() {
        (modo.columns(), modo.rows())
    } else {
        (80, 25)
    };

    let mut cf = Color::Cyan;
    let mut ma = false;
    let mut va = 1;
    let mut sm = 0;
    let mut red = true;

    loop {
        if red {
            let _ = sistema.stdout().set_color(Color::White, cf);
            let _ = sistema.stdout().clear();
            if va == 1 {
                dibujar_ventana(sistema, mc, mr, 50, 10, "LADE - MtrxOS", "Bienvenido a LADE Desktop.\n[ESC] Menu\n[ENTER] Cerrar", Color::White, Color::Blue);
            }
            if ma {
                dibujar_ventana(sistema, mc, mr, 26, 12, "Menu", "", Color::Black, Color::LightGray);
                let mx = mc.saturating_sub(26) / 2 + 2;
                let my = mr.saturating_sub(12) / 2 + 2;
                let ops = ["Fondo", "Acerca", "Monitor", "Calculadora", "Reiniciar", "Apagar"];
                for (i, op) in ops.iter().enumerate() {
                    let _ = sistema.stdout().set_cursor_position(mx, my + i);
                    let _ = write!(sistema.stdout(), "{} {}", if sm == i { ">" } else { " " }, op);
                }
            }
            red = false;
        }

        if let Ok(Some(evento)) = sistema.stdin().read_key() {
            match evento {
                Key::Special(ScanCode::ESCAPE) => {
                    ma = !ma;
                    if ma { sm = 0; }
                    red = true;
                }
                Key::Special(ScanCode::UP) if ma && sm > 0 => { sm -= 1; red = true; }
                Key::Special(ScanCode::DOWN) if ma && sm < 5 => { sm += 1; red = true; }
                Key::Printable(key) if u16::from(key) == 13 => {
                    if ma {
                        ma = false;
                        match sm {
                            0 => cf = match cf { Color::Cyan => Color::Red, Color::Red => Color::Green, _ => Color::Cyan },
                            1 => va = 1,
                            2 => monitor::ejecutar(sistema, mc, mr),
                            3 => calculadora::ejecutar(sistema, mc, mr),
                            4 => sistema.runtime_services().reset(ResetType::COLD, uefi::Status::SUCCESS, None),
                            5 => sistema.runtime_services().reset(ResetType::SHUTDOWN, uefi::Status::SUCCESS, None),
                            _ => {}
                        }
                    } else { va = 0; }
                    red = true;
                }
                _ => {}
            }
        }
    }
}

pub fn dibujar_ventana(sistema: &mut SystemTable<Boot>, mc: usize, mr: usize, an: usize, al: usize, tit: &str, txt: &str, fg: Color, bg: Color) {
    let x = mc.saturating_sub(an) / 2;
    let y = mr.saturating_sub(al) / 2;

    let _ = sistema.stdout().set_color(Color::Black, Color::Black);
    for i in 1..=al {
        let _ = sistema.stdout().set_cursor_position(x + 2, y + i);
        for _ in 0..an { let _ = write!(sistema.stdout(), " "); }
    }

    let _ = sistema.stdout().set_color(fg, bg);
    for i in 0..al {
        let _ = sistema.stdout().set_cursor_position(x, y + i);
        if i == 0 {
            let _ = write!(sistema.stdout(), "┌");
            for _ in 0..(an.saturating_sub(2)) { let _ = write!(sistema.stdout(), "─"); }
            let _ = write!(sistema.stdout(), "┐");
        } else if i == al - 1 {
            let _ = write!(sistema.stdout(), "└");
            for _ in 0..(an.saturating_sub(2)) { let _ = write!(sistema.stdout(), "─"); }
            let _ = write!(sistema.stdout(), "┘");
        } else {
            let _ = write!(sistema.stdout(), "│");
            for _ in 0..(an.saturating_sub(2)) { let _ = write!(sistema.stdout(), " "); }
            let _ = write!(sistema.stdout(), "│");
        }
    }

    let t_pos = x + (an.saturating_sub(tit.len() + 2)) / 2;
    let _ = sistema.stdout().set_cursor_position(t_pos, y);
    let _ = write!(sistema.stdout(), " {} ", tit);

    for (i, line) in txt.lines().enumerate() {
        let _ = sistema.stdout().set_cursor_position(x + 2, y + 2 + i);
        let _ = write!(sistema.stdout(), "{}", line);
    }
}