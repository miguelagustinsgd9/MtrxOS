use crate::include::*;
pub fn ejecutar_panic(sistema: &mut SystemTable<Boot>, motivo: &str) {
    let _ = sistema.stdout().enable_cursor(false);
    let tiempo = sistema.runtime_services().get_time().ok();
    let stdout = sistema.stdout();

    let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Red);
    let _ = stdout.clear();

    let logo = [
        "      /\\      ",
        "     /  \\     ",
        "    /    \\    ",
        "   /  /\\  \\   ",
        "  /  /  \\  \\  ",
        " /__/    \\__\\  ",
    ];

    for (i, linea) in logo.iter().enumerate() {
        let _ = stdout.set_cursor_position(2, 1 + i);
        let _ = write!(stdout, "{}", linea);
    }

    let _ = stdout.set_cursor_position(20, 2);
    let _ = write!(stdout, "MtrxOS KERNEL PANIC :( ");

    let _ = stdout.set_cursor_position(20, 4);
    let _ = write!(stdout, "Se ha detectado una excepcion no controlada.");

    let _ = stdout.set_cursor_position(20, 5);
    let _ = write!(stdout, "El sistema se ha detenido para proteger los datos.");

    let _ = stdout.set_cursor_position(2, 10);
    let _ = write!(stdout, "DETALLES TECNICOS:");
    let _ = stdout.set_cursor_position(2, 11);
    let _ = write!(stdout, "--------------------------------------------------");

    let _ = stdout.set_cursor_position(2, 12);
    let _ = write!(stdout, "ERROR_CODE: {}", if motivo.is_empty() { "CRITICAL_SERVICE_FAILED" } else { motivo });

    if let Some(t) = tiempo {
        let _ = stdout.set_cursor_position(2, 13);
        let _ = write!(stdout, "TIME_STAMP: {:02}:{:02}:{:02} | DATE: {:02}/{:02}/{}",
                       t.hour(), t.minute(), t.second(), t.day(), t.month(), t.year());
    }

    let _ = stdout.set_cursor_position(2, 15);
    let _ = write!(stdout, "MEM_DUMP: 0x0000000000000F02h - 0xFFFFFFFFFFFFFFFFh");

    let _ = stdout.set_cursor_position(2, 17);
    let _ = write!(stdout, "--------------------------------------------------");

    let _ = stdout.set_cursor_position(2, 19);
    let _ = write!(stdout, "Presione cualquier tecla para reiniciar...");

    loop {
        if let Ok(Some(_)) = sistema.stdin().read_key() {
            sistema.runtime_services().reset(
                uefi::table::runtime::ResetType::COLD,
                uefi::Status::ABORTED,
                None
            );
        }
    }
}
