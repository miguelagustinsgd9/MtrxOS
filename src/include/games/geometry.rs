use crate::include::*;

struct Spike {
    x: i32,
    w: i32,
    h: i32,
}

pub fn run(sistema: &mut SystemTable<Boot>, gl: &mut MtrxGl) {
    let mut seed = 987654321u32;
    let ground_y = gl.height * 3 / 4;
    let player_size = 30;
    let player_x = gl.width / 5;
    let mut player_y_fp = (ground_y - player_size) << 8;
    let mut player_vy_fp = 0;
    let gravity_fp = 170;
    let jump_fp = -2400;
    let speed = 9;

    let spacing = gl.width / 2;
    let mut spikes = [
        Spike { x: gl.width, w: 34, h: 45 },
        Spike { x: gl.width + spacing, w: 34, h: 45 },
        Spike { x: gl.width + spacing * 2, w: 34, h: 45 },
    ];

    let mut game_over = false;

    loop {
        let mut jump = false;

        if let Ok(Some(evento)) = sistema.stdin().read_key() {
            match evento {
                Key::Special(ScanCode::ESCAPE) => {
                    sistema.runtime_services().reset(
                        ResetType::COLD,
                        uefi::Status::SUCCESS,
                        None,
                    );
                },
                Key::Printable(t) => {
                    let c = u16::from(t) as u8 as char;
                    if c == 'q' || c == 'Q' { break; }
                    if c == ' ' { jump = true; }
                }
                _ => {}
            }
        }

        if game_over {
            if jump {
                game_over = false;
                player_y_fp = (ground_y - player_size) << 8;
                player_vy_fp = 0;
                for i in 0..3 {
                    spikes[i].x = gl.width + (i as i32) * spacing;
                }
            }
        } else {
            let py_int = player_y_fp >> 8;
            let on_ground = py_int >= ground_y - player_size;

            if on_ground {
                player_y_fp = (ground_y - player_size) << 8;
                player_vy_fp = 0;
                if jump {
                    player_vy_fp = jump_fp;
                }
            } else {
                player_vy_fp += gravity_fp;
            }

            player_y_fp += player_vy_fp;
            let py = player_y_fp >> 8;

            for i in 0..3 {
                spikes[i].x -= speed;
                if spikes[i].x + spikes[i].w < 0 {
                    seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                    spikes[i].x += spacing * 2;
                    spikes[i].h = 35 + (seed as i32 % 25).abs();
                }

                let p_left = player_x;
                let p_right = player_x + player_size;
                let p_top = py;
                let p_bottom = py + player_size;

                let s_left = spikes[i].x + 8;
                let s_right = spikes[i].x + spikes[i].w - 8;
                let s_top = ground_y - spikes[i].h + 5;

                if p_right > s_left && p_left < s_right && p_bottom > s_top {
                    game_over = true;
                }
            }
        }

        gl.clear(0x2B1B54);

        gl.fill_rect(0, ground_y, gl.width, gl.height - ground_y, 0x110A24);
        gl.draw_line(0, ground_y, gl.width, ground_y, 0xFFFFFF);

        for i in 0..3 {
            let sx = spikes[i].x;
            let sy = ground_y;
            let sw = spikes[i].w;
            let sh = spikes[i].h;
            gl.fill_triangle(sx, sy, sx + sw / 2, sy - sh, sx + sw, sy, 0xFF0044);
        }

        let py = player_y_fp >> 8;
        gl.fill_rect(player_x, py, player_size, player_size, 0x00FFCC);

        if game_over {
            gl.fill_rect(gl.width / 2 - 100, gl.height / 2 - 25, 200, 50, 0xFF0000);
        }

        sistema.boot_services().stall(30_000);
    }
}
