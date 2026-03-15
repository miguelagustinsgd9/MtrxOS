use crate::include::*;
pub fn ejecutar(sistema: &mut SystemTable<Boot>, mc: usize, mr: usize) {
    let mut buf = [0u8; 8192];
    lade::dibujar_ventana(sistema, mc, mr, 40, 10, "SysMonitor", "", Color::Black, Color::LightGray);
    let x = (mc.saturating_sub(40)) / 2 + 2;
    let y = (mr.saturating_sub(10)) / 2 + 2;

    loop {
        let mut ram: u64 = 0;
        if let Ok(mmap) = sistema.boot_services().memory_map(&mut buf) {
            for e in mmap.entries() { if e.ty == uefi::table::boot::MemoryType::CONVENTIONAL { ram += e.page_count * 4096; } }
        }

        let _ = sistema.stdout().set_color(Color::Black, Color::LightGray);
        let _ = sistema.stdout().set_cursor_position(x, y);
        let _ = write!(sistema.stdout(), "RAM: {} MB       ", ram / 1048576);
        let _ = sistema.stdout().set_cursor_position(x, y + 2);
        let _ = write!(sistema.stdout(), "Res: {}x{}       ", mc, mr);
        let _ = sistema.stdout().set_cursor_position(x, y + 4);
        let _ = write!(sistema.stdout(), "[ESC] Salir");

        if let Ok(Some(evento)) = sistema.stdin().read_key() {
            if let Key::Special(ScanCode::ESCAPE) = evento { break; }
        }
        sistema.boot_services().stall(200_000);
    }
}
