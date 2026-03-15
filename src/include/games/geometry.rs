use crate::include::*;

#[derive(Copy, Clone)]
struct Object {
    x: i32,
    w: i32,
    h: i32,
    is_platform: bool,
    inverted: bool,
    active: bool,
}

pub fn run(sistema: &mut SystemTable<Boot>, gl: &mut MtrxGl) {
    let mut seed = 4294967295u32;
    let ground_y = gl.height * 3 / 4;
    let player_size = 30;
    let player_x = gl.width / 5;
    
    let mut player_y_fp = (ground_y - player_size) << 8;
    let mut player_vy_fp = 0;
    let gravity_fp = 185;
    let jump_fp = -2900;
    let mut speed = 12;
    let mut score = 0;
    
    let mut objects = [Object { x: -100, w: 0, h: 0, is_platform: false, inverted: false, active: false }; 4];
    let mut next_spawn_x = gl.width;

    let mut bg_stars = [0i32; 15];
    let mut bg_stars_y = [0i32; 15];
    for i in 0..15 {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        bg_stars[i] = (seed % (gl.width as u32)) as i32;
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        bg_stars_y[i] = (seed % (ground_y as u32)) as i32;
    }

    let mut trail = [player_y_fp >> 8; 6];
    let mut game_over = false;

    loop {
        let mut jump_input = false;

        if let Ok(Some(evento)) = sistema.stdin().read_key() {
            match evento {
                Key::Special(ScanCode::ESCAPE) => {
                    sistema.runtime_services().reset(ResetType::COLD, uefi::Status::SUCCESS, None);
                },
                Key::Printable(t) => {
                    let c = u16::from(t) as u8 as char;
                    if c == ' ' { jump_input = true; }
                }
                _ => {}
            }
        }

        if game_over {
            if jump_input {
                game_over = false;
                player_y_fp = (ground_y - player_size) << 8;
                player_vy_fp = 0;
                speed = 12;
                score = 0;
                next_spawn_x = gl.width;
                for obj in objects.iter_mut() { obj.active = false; }
            }
        } else {
            score += 1;
            if score % 400 == 0 && speed < 28 { speed += 1; }

            for i in (1..6).rev() { trail[i] = trail[i - 1]; }
            trail[0] = player_y_fp >> 8;

            if next_spawn_x < gl.width + 100 {
                for obj in objects.iter_mut() {
                    if !obj.active {
                        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                        let rand = seed % 100;
                        
                        obj.active = true;
                        obj.x = next_spawn_x;
                        
                        if rand < 30 {
                            obj.is_platform = true;
                            obj.inverted = false;
                            obj.w = 120 + (seed % 80) as i32;
                            obj.h = 80;
                            next_spawn_x += obj.w + 200 + (seed % 100) as i32;
                        } else {
                            obj.is_platform = false;
                            obj.w = 34;
                            obj.h = 44;
                            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                            obj.inverted = (seed % 3) == 0;
                            next_spawn_x += 300 + (seed % 150) as i32;
                        }
                        break;
                    }
                }
            }

            let py_int = player_y_fp >> 8;
            let mut on_surface = py_int >= ground_y - player_size;
            let mut current_surface_y = ground_y;

            for obj in objects.iter() {
                if obj.active && obj.is_platform {
                    let plat_top = ground_y - obj.h;
                    if player_x + player_size > obj.x && player_x < obj.x + obj.w {
                        if py_int + player_size <= plat_top + 10 && player_vy_fp >= 0 {
                            on_surface = py_int >= plat_top - player_size;
                            current_surface_y = plat_top;
                        }
                    }
                }
            }

            if on_surface {
                player_y_fp = (current_surface_y - player_size) << 8;
                player_vy_fp = 0;
                if jump_input { player_vy_fp = jump_fp; }
            } else {
                player_vy_fp += gravity_fp;
            }

            player_y_fp += player_vy_fp;
            let py = player_y_fp >> 8;

            let p_rect = (player_x + 6, py + 6, player_size - 12, player_size - 12);
            for obj in objects.iter_mut() {
                if obj.active {
                    obj.x -= speed;
                    if obj.x + obj.w < 0 { obj.active = false; }

                    if !obj.is_platform {
                        if p_rect.0 + p_rect.2 > obj.x + 8 && p_rect.0 < obj.x + obj.w - 8 {
                            if obj.inverted {
                                let s_bottom = ground_y - 110;
                                let s_tip = s_bottom + obj.h;
                                if p_rect.1 < s_tip { game_over = true; }
                            } else {
                                let s_top = ground_y - obj.h + 8;
                                if p_rect.1 + p_rect.3 > s_top { game_over = true; }
                            }
                        }
                    } else {
                        let plat_top = ground_y - obj.h;
                        if p_rect.0 + p_rect.2 > obj.x && p_rect.0 < obj.x + 10 {
                            if p_rect.1 + p_rect.3 > plat_top + 10 { game_over = true; }
                        }
                    }
                }
            }
            
            for i in 0..15 {
                bg_stars[i] -= speed / 3;
                if bg_stars[i] < 0 {
                    bg_stars[i] = gl.width;
                    seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                    bg_stars_y[i] = (seed % (ground_y as u32)) as i32;
                }
            }
            next_spawn_x -= speed;
        }

        gl.clear(0x1A1A2E);

        for i in 0..15 {
            gl.draw_pixel(bg_stars[i], bg_stars_y[i], 0x4F4F6A);
            gl.draw_pixel(bg_stars[i] + 1, bg_stars_y[i], 0x4F4F6A);
            gl.draw_pixel(bg_stars[i], bg_stars_y[i] + 1, 0x4F4F6A);
            gl.draw_pixel(bg_stars[i] + 1, bg_stars_y[i] + 1, 0x4F4F6A);
        }

        gl.fill_rect(0, ground_y, gl.width, gl.height - ground_y, 0x16213E);

        let offset = (score * speed) % 60;
        for i in 0..(gl.width / 60 + 2) {
            let lx = i * 60 - offset;
            gl.draw_line(lx, ground_y, lx - 30, ground_y + 60, 0x0F3460);
        }

        gl.draw_line(0, ground_y, gl.width, ground_y, 0xE94560);
        gl.draw_line(0, ground_y + 1, gl.width, ground_y + 1, 0xE94560);

        for obj in objects.iter() {
            if obj.active {
                if obj.is_platform {
                    gl.fill_rect(obj.x, ground_y - obj.h, obj.w, obj.h, 0x0F3460);
                    gl.draw_rect(obj.x, ground_y - obj.h, obj.w, obj.h, 0x00FFCC);
                } else {
                    let sx = obj.x;
                    let sw = obj.w;
                    let sh = obj.h;
                    if obj.inverted {
                        let sy = ground_y - 110;
                        gl.fill_triangle(sx, sy, sx + sw / 2, sy + sh, sx + sw, sy, 0xE94560);
                    } else {
                        let sy = ground_y;
                        gl.fill_triangle(sx, sy, sx + sw / 2, sy - sh, sx + sw, sy, 0xE94560);
                    }
                }
            }
        }

        let py = player_y_fp >> 8;

        for i in (0..6).rev() {
            let t_y = trail[i];
            let t_x = player_x - (i as i32 * 8);
            let color_val = 20 + ((6 - i as u32) * 15);
            let color = (color_val << 16) | (color_val << 8) | (color_val + 50);
            gl.fill_rect(t_x, t_y + 6, player_size - 12, player_size - 12, color);
        }

        gl.fill_rect(player_x, py, player_size, player_size, 0xE94560);
        gl.fill_rect(player_x + 6, py + 6, player_size - 12, player_size - 12, 0x0F3460);
        gl.fill_rect(player_x + 10, py + 10, player_size - 20, player_size - 20, 0x00FFCC);

        if game_over {
            gl.fill_rect(gl.width / 2 - 120, gl.height / 2 - 30, 240, 60, 0xFF0000);
            gl.fill_rect(gl.width / 2 - 115, gl.height / 2 - 25, 230, 50, 0x1A1A2E);
            gl.fill_rect(gl.width / 2 - 15, gl.height / 2 - 15, 30, 30, 0xE94560);
        }

        sistema.boot_services().stall(18_000);
    }
}