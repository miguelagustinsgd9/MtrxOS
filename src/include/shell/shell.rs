use crate::include::*;
pub fn run(
    sistema: &mut SystemTable<Boot>,
    nombre_buffer: &mut [u8; 32],
    nombre_len: &mut usize,
    ticks_inicio: u32,
) {
    let mut es_root = false;
    let mut tabla_alias_corto: [[u8; 8]; 5] = [[0u8; 8]; 5];
    let mut tabla_alias_largo: [[u8; 16]; 5] = [[0u8; 16]; 5];
    let mut num_alias = 0;
    let mut microsegundos_frame = 40_000;

    loop {
        let nombre_actual = core::str::from_utf8(&nombre_buffer[..*nombre_len]).unwrap_or("pc");

        if es_root {
            write!(sistema.stdout(), "[root@mtrx-os /]# ").unwrap();
        } else {
            write!(sistema.stdout(), "[{}@mtrx-os ~]$ ", nombre_actual).unwrap();
        }

        let mut buffer = [0u8; 128];
        let mut indice = 0;

        loop {
            if let Ok(Some(evento)) = sistema.stdin().read_key() {
                match evento {
                    Key::Printable(tecla) => {
                        let valor = u16::from(tecla);
                        if valor == 13 {
                            writeln!(sistema.stdout(), "").unwrap();
                            break;
                        } else if valor == 8 {
                            if indice > 0 {
                                indice -= 1;
                                write!(sistema.stdout(), "\x08 \x08").unwrap();
                            }
                        } else if valor >= 32 && valor <= 126 {
                            if indice < buffer.len() {
                                buffer[indice] = valor as u8;
                                indice += 1;
                                write!(sistema.stdout(), "{}", valor as u8 as char).unwrap();
                            }
                        }
                    }
                    Key::Special(ScanCode::FUNCTION_1) => {
                        let cmd = b"help";
                        for (i, &byte) in cmd.iter().enumerate() {
                            buffer[i] = byte;
                        }
                        indice = cmd.len();
                        writeln!(sistema.stdout(), "help").unwrap();
                        break;
                    }
                    Key::Special(ScanCode::ESCAPE) => {
                        writeln!(sistema.stdout(), "\nApagando el sistema...").unwrap();
                        sistema.runtime_services().reset(
                            ResetType::SHUTDOWN,
                            uefi::Status::SUCCESS,
                            None,
                        );
                    }
                    _ => {}
                }
            }
        }

        let entrada_cruda = core::str::from_utf8(&buffer[..indice]).unwrap_or("");
        let entrada = entrada_cruda.trim();
        if entrada.is_empty() { continue; }

        for sub_entrada in entrada.split("&&") {
            let sub_trim = sub_entrada.trim();
            if sub_trim.is_empty() { continue; }

            let mut partes = sub_trim.splitn(2, ' ');
            let comando_crudo = partes.next().unwrap_or("");
            let argumentos = partes.next().unwrap_or("");

            let mut comando_temp = [0u8; 16];
            let mut es_alias = false;

            for i in 0..num_alias {
                let len_c = tabla_alias_corto[i].iter().position(|&x| x == 0).unwrap_or(8);
                if let Ok(corto) = core::str::from_utf8(&tabla_alias_corto[i][..len_c]) {
                    if comando_crudo == corto {
                        let len_l = tabla_alias_largo[i].iter().position(|&x| x == 0).unwrap_or(16);
                        comando_temp[..len_l].copy_from_slice(&tabla_alias_largo[i][..len_l]);
                        es_alias = true;
                        break;
                    }
                }
            }

            let comando_final = if es_alias {
                let len = comando_temp.iter().position(|&x| x == 0).unwrap_or(16);
                core::str::from_utf8(&comando_temp[..len]).unwrap_or(comando_crudo)
            } else {
                match comando_crudo {
                    "m" => "matrix",
                    "f" => "fetch",
                    "c" => "clear",
                    "h" => "help",
                    _   => comando_crudo,
                }
            };

            crate::commands::ejecutar_comando(
                comando_final,
                argumentos,
                sistema,
                &mut es_root,
                nombre_buffer,
                nombre_len,
                ticks_inicio,
                &mut microsegundos_frame,
                &mut tabla_alias_corto,
                &mut tabla_alias_largo,
                &mut num_alias,
            );
        }
        sistema.boot_services().stall(microsegundos_frame as usize);
    }
}
