use crate::include::*;

#[derive(Copy, Clone)]
struct Pipe {
    x: i32,
    gap_y: i32,
}

pub fn run(sistema: &mut SystemTable<Boot>, gl: &mut MtrxGl) {
    let mut seed = 123456789u32;
    let bird_x = gl.width / 4;
    let bird_radius = 12;
    let mut bird_y_fp = (gl.height / 2) << 8;
    let mut bird_v_fp = 0;
    let gravity_fp = 110;
    let jump_fp = -1900;
    let pipe_speed = 6;
    let pipe_width = 55;
    let gap_height = 150;
    let pipe_spacing = gl.width / 3;

    let mut pipes = [
        Pipe { x: gl.width, gap_y: gl.height / 2 },
        Pipe { x: gl.width + pipe_spacing, gap_y: gl.height / 2 },
        Pipe { x: gl.width + pipe_spacing * 2, gap_y: gl.height / 2 },
    ];

    let mut game_over = false;

    loop {
        let mut jump_requested = false;

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
                    if c == ' ' { jump_requested = true; }
                }
                _ => {}
            }
        }

        if game_over {
            if jump_requested {
                game_over = false;
                bird_y_fp = (gl.height / 2) << 8;
                bird_v_fp = 0;
                for i in 0..3 {
                    pipes[i].x = gl.width + (i as i32) * pipe_spacing;
                    pipes[i].gap_y = gl.height / 2;
                }
            }
        } else {
            if jump_requested { bird_v_fp = jump_fp; }
            bird_v_fp += gravity_fp;
            bird_y_fp += bird_v_fp;

            let by = bird_y_fp >> 8;
            if by - bird_radius < 0 || by + bird_radius > gl.height {
                game_over = true;
            }

            for p in pipes.iter_mut() {
                p.x -= pipe_speed;
                if p.x + pipe_width < 0 {
                    p.x += pipe_spacing * 3;
                    seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                    let range = gl.height - gap_height - 100;
                    p.gap_y = 50 + (gap_height / 2) + (seed as i32 % range).abs();
                }
                if bird_x + bird_radius > p.x && bird_x - bird_radius < p.x + pipe_width {
                    if by - bird_radius < p.gap_y - gap_height / 2 || by + bird_radius > p.gap_y + gap_height / 2 {
                        game_over = true;
                    }
                }
            }
        }

        gl.clear(0x4EC0CA);

        for p in pipes.iter() {
            gl.fill_rect(p.x, 0, pipe_width, p.gap_y - gap_height / 2, 0x73BF2E);
            gl.fill_rect(p.x, p.gap_y + gap_height / 2, pipe_width, gl.height, 0x73BF2E);
        }

        gl.fill_circle(bird_x, bird_y_fp >> 8, bird_radius, 0xF7E100);

        if game_over {
            gl.fill_rect(gl.width / 2 - 100, gl.height / 2 - 25, 200, 50, 0xFF0000);
        }

        sistema.boot_services().stall(30_000);
    }
}
