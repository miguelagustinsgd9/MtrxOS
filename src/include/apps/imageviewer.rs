use crate::include::*;
fn draw_char(gl: &mut MtrxGl, x: i32, y: i32, ch: char, color: u32, scale: i32) {
    let bitmap: [u8; 15] = match ch {
        'A' => [0,1,0, 1,0,1, 1,1,1, 1,0,1, 1,0,1],
        'E' => [1,1,1, 1,0,0, 1,1,0, 1,0,0, 1,1,1],
        'G' => [0,1,1, 1,0,0, 1,0,1, 1,0,1, 0,1,1],
        'I' => [1,1,1, 0,1,0, 0,1,0, 0,1,0, 1,1,1],
        'M' => [1,0,1, 1,1,1, 1,0,1, 1,0,1, 1,0,1],
        'N' => [1,1,0, 1,0,1, 1,0,1, 1,0,1, 1,0,1],
        '1' => [0,1,0, 1,1,0, 0,1,0, 0,1,0, 1,1,1],
        '2' => [1,1,1, 0,0,1, 1,1,1, 1,0,0, 1,1,1],
        '3' => [1,1,1, 0,0,1, 0,1,1, 0,0,1, 1,1,1],
        '/' => [0,0,1, 0,0,1, 0,1,0, 1,0,0, 1,0,0],
        ':' => [0,0,0, 0,1,0, 0,0,0, 0,1,0, 0,0,0],
        _   => [0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0],
    };
    for row in 0..5 {
        for col in 0..3 {
            if bitmap[(row * 3 + col) as usize] == 1 {
                gl.fill_rect(x + col * scale, y + row * scale, scale, scale, color);
            }
        }
    }
}

pub fn ejecutar(sistema: &mut SystemTable<Boot>, gl: &mut MtrxGl) {
    let t = 0xFF00FF;
    let b = 0x000000;
    let w = 0xFFFFFF;
    let y = 0xFFCC00;
    let d = 0xCC9900;
    let g = 0x555555;
    let l = 0xAAAAAA;

    let img_tux: [u32; 256] = [
        t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t,
        t, t, t, t, t, t, b, b, b, b, t, t, t, t, t, t,
        t, t, t, t, t, b, w, w, w, w, b, t, t, t, t, t,
        t, t, t, t, b, w, b, w, w, b, w, b, t, t, t, t,
        t, t, t, t, b, w, w, w, w, w, w, b, t, t, t, t,
        t, t, t, b, w, w, w, w, w, w, w, w, b, t, t, t,
        t, t, t, b, b, w, w, w, w, w, w, b, b, t, t, t,
        t, t, b, w, b, w, w, w, w, w, w, b, w, b, t, t,
        t, t, b, w, b, w, w, w, w, w, w, b, w, b, t, t,
        t, t, b, w, b, b, w, w, w, w, b, b, w, b, t, t,
        t, t, t, b, b, w, w, w, w, w, w, b, b, t, t, t,
        t, t, t, t, b, w, w, w, w, w, w, b, t, t, t, t,
        t, t, t, b, b, b, b, b, b, b, b, b, b, t, t, t,
        t, t, b, y, y, b, b, b, b, b, b, y, y, b, t, t,
        t, b, y, y, y, y, b, b, b, b, y, y, y, y, b, t,
        t, t, b, b, b, b, t, t, t, t, b, b, b, b, t, t,
    ];

    let img_folder: [u32; 256] = [
        t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,
        t,t,b,b,b,b,t,t,t,t,t,t,t,t,t,t,
        t,b,y,y,y,y,b,t,t,t,t,t,t,t,t,t,
        b,y,y,y,y,y,y,b,b,b,b,b,b,b,t,t,
        b,y,y,y,y,y,y,y,y,y,y,y,y,y,b,t,
        b,d,d,d,d,d,d,d,d,d,d,d,d,d,b,t,
        b,d,y,y,y,y,y,y,y,y,y,y,y,y,b,t,
        b,d,y,y,y,y,y,y,y,y,y,y,y,y,b,t,
        b,d,y,y,y,y,y,y,y,y,y,y,y,y,b,t,
        b,d,y,y,y,y,y,y,y,y,y,y,y,y,b,t,
        b,d,y,y,y,y,y,y,y,y,y,y,y,y,b,t,
        b,d,y,y,y,y,y,y,y,y,y,y,y,y,b,t,
        b,d,y,y,y,y,y,y,y,y,y,y,y,y,b,t,
        b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,t,
        t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,
        t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,
    ];

    let img_disk: [u32; 256] = [
        t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,
        t,b,b,b,b,b,b,b,b,b,b,b,b,t,t,t,
        t,b,g,g,g,g,g,g,g,g,g,g,b,b,t,t,
        t,b,g,w,w,w,w,w,w,w,g,g,g,b,t,t,
        t,b,g,w,w,w,w,w,w,w,g,g,g,b,t,t,
        t,b,g,w,w,w,w,w,w,w,g,g,g,b,t,t,
        t,b,g,g,g,g,g,g,g,g,g,g,g,b,t,t,
        t,b,g,g,g,g,g,g,g,g,g,g,g,b,t,t,
        t,b,g,l,l,l,l,l,l,l,l,g,g,b,t,t,
        t,b,g,l,b,b,b,b,b,b,l,g,g,b,t,t,
        t,b,g,l,b,b,b,b,b,b,l,g,g,b,t,t,
        t,b,g,l,b,b,b,b,b,b,l,g,g,b,t,t,
        t,b,g,l,l,l,l,l,l,l,l,g,g,b,t,t,
        t,b,b,b,b,b,b,b,b,b,b,b,b,b,t,t,
        t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,
        t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,
    ];

    let images = [img_tux, img_folder, img_disk];
    let total_imgs = 3;
    let mut current_img = 0;
    
    let img_w = 16;
    let img_h = 16;
    let scale = 20;

    let total_w = img_w * scale;
    let total_h = img_h * scale;
    let start_x = (gl.width - total_w) / 2;
    let start_y = (gl.height - total_h) / 2 - 20;

    let mut redraw = true;

    loop {
        if redraw {
            gl.clear(0x1B2B34);

            let img_data = &images[current_img];
            for row in 0..img_h {
                for col in 0..img_w {
                    let color = img_data[(row * img_w + col) as usize];
                    if color != t {
                        let px = start_x + (col * scale);
                        let py = start_y + (row * scale);
                        gl.fill_rect(px, py, scale, scale, color);
                    }
                }
            }

            let num_char = (b'1' + current_img as u8) as char;
            let text = ['I','M','A','G','E','N',':',' ', num_char, '/','3'];
            let text_scale = 3;
            let char_w = 4 * text_scale;
            let mut curr_x = (gl.width - (text.len() as i32 * char_w)) / 2;
            let text_y = gl.height - 50;

            for c in text.iter() {
                draw_char(gl, curr_x, text_y, *c, 0xFFFFFF, text_scale);
                curr_x += char_w;
            }

            redraw = false;
        }

        if let Ok(Some(evento)) = sistema.stdin().read_key() {
            match evento {
                Key::Special(ScanCode::ESCAPE) => {
                    sistema.runtime_services().reset(ResetType::COLD, uefi::Status::SUCCESS, None);
                }
                Key::Special(ScanCode::LEFT) => {
                    current_img = (current_img + total_imgs - 1) % total_imgs;
                    redraw = true;
                }
                Key::Special(ScanCode::RIGHT) => {
                    current_img = (current_img + 1) % total_imgs;
                    redraw = true;
                }
                _ => {}
            }
        }
        sistema.boot_services().stall(15_000);
    }
}