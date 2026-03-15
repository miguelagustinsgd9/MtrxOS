use crate::include::*;
pub fn ejecutar(sistema: &mut SystemTable<Boot>, mc: usize, mr: usize) {
    let mut n1: i64 = 0;
    let mut n2: i64 = 0;
    let mut op = ' ';
    let mut res: i64 = 0;
    let mut paso = 0;
    let mut redibujar = true;

    loop {
        if redibujar {
            lade::dibujar_ventana(sistema, mc, mr, 32, 11, "Calculadora", "", Color::Black, Color::LightGray);
            let x = (mc.saturating_sub(32)) / 2 + 4;
            let y = (mr.saturating_sub(11)) / 2 + 2;

            let _ = sistema.stdout().set_cursor_position(x, y);
            let _ = write!(sistema.stdout(), "N1: {}", if paso >= 1 { n1 } else { 0 });
            let _ = sistema.stdout().set_cursor_position(x, y + 1);
            let _ = write!(sistema.stdout(), "OP: {}", op);
            let _ = sistema.stdout().set_cursor_position(x, y + 2);
            let _ = write!(sistema.stdout(), "N2: {}", if paso >= 3 { n2 } else { 0 });
            let _ = sistema.stdout().set_cursor_position(x, y + 4);
            let _ = write!(sistema.stdout(), "RESULTADO: {}", res);
            let _ = sistema.stdout().set_cursor_position(x, y + 6);
            let _ = write!(sistema.stdout(), "[ESC] Salir");
            redibujar = false;
        }

        if let Ok(Some(k)) = sistema.stdin().read_key() {
            match k {
                Key::Special(ScanCode::ESCAPE) => break,
                Key::Printable(key) => {
                    let c = u16::from(key) as u8 as char;
                    match c {
                        '0'..='9' => {
                            let d = (c as i64) - 48;
                            if paso <= 1 { n1 = n1 * 10 + d; paso = 1; redibujar = true; }
                            else if paso >= 2 && paso < 4 { n2 = n2 * 10 + d; paso = 3; redibujar = true; }
                        }
                        '+'|'-'|'*'|'/' if paso == 1 => { op = c; paso = 2; redibujar = true; }
                        _ if u16::from(key) == 13 => {
                            if paso == 3 {
                                res = match op {
                                    '+' => n1 + n2,
                                    '-' => n1 - n2,
                                    '*' => n1 * n2,
                                    '/' => if n2 != 0 { n1 / n2 } else { 0 },
                                    _ => 0,
                                };
                                paso = 4;
                                redibujar = true;
                            } else if paso == 4 {
                                n1 = 0; n2 = 0; op = ' '; res = 0; paso = 0;
                                redibujar = true;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
