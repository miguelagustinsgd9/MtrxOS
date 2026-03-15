use crate::include::*;
const SIN_LUT: [i32; 65] = [
    0, 6, 12, 18, 25, 31, 37, 43, 49, 56, 62, 68, 74, 80, 86, 92, 97, 103, 109, 115, 120, 126, 131, 136, 142, 147, 152, 157, 162, 167, 171, 176, 180, 185, 189, 193, 197, 201, 205, 208, 212, 215, 219, 222, 225, 228, 231, 233, 236, 238, 240, 242, 244, 246, 247, 249, 250, 252, 253, 254, 254, 255, 255, 256, 256
];

fn isin(mut a: i32) -> i32 {
    a = (a % 256 + 256) % 256;
    if a < 64 {
        SIN_LUT[a as usize]
    } else if a < 128 {
        SIN_LUT[(128 - a) as usize]
    } else if a < 192 {
        -SIN_LUT[(a - 128) as usize]
    } else {
        -SIN_LUT[(256 - a) as usize]
    }
}

fn icos(a: i32) -> i32 {
    isin(a + 64)
}

#[derive(Copy, Clone)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

fn transform(p: Vec3, cx: i32, cy: i32, cz: i32, pitch: i32, yaw: i32, roll: i32) -> Vec3 {
    let dx = p.x - cx;
    let dy = p.y - cy;
    let dz = p.z - cz;

    let sy = isin(yaw);
    let cy_yaw = icos(yaw);
    let x1 = (dx * cy_yaw - dz * sy) >> 8;
    let z1 = (dx * sy + dz * cy_yaw) >> 8;

    let sp = isin(pitch);
    let cp = icos(pitch);
    let y1 = (dy * cp - z1 * sp) >> 8;
    let z2 = (dy * sp + z1 * cp) >> 8;

    let sr = isin(roll);
    let cr = icos(roll);
    let x2 = (x1 * cr - y1 * sr) >> 8;
    let y2 = (x1 * sr + y1 * cr) >> 8;

    Vec3 { x: x2, y: y2, z: z2 }
}

fn project(p: Vec3, w: i32, h: i32) -> Option<(i32, i32)> {
    if p.z <= 20 {
        return None;
    }
    let fov = 350;
    let sx = w / 2 + (p.x * fov) / p.z;
    let sy = h / 2 - (p.y * fov) / p.z;
    Some((sx, sy))
}

fn get_height(x: i32, z: i32) -> i32 {
    let x1 = ((x / 45) % 256 + 256) % 256;
    let z1 = ((z / 45) % 256 + 256) % 256;
    let x2 = ((x / 17) % 256 + 256) % 256;
    let z2 = ((z / 23) % 256 + 256) % 256;
    let x3 = ((x / 97) % 256 + 256) % 256;
    let z3 = ((z / 89) % 256 + 256) % 256;

    let h1 = (isin(x1) * icos(z1)) >> 8;
    let h2 = (icos(x2) * isin(z2)) >> 7;
    let h3 = (isin(x3) * icos(z3)) >> 8;

    let mut h = (h1 + h2 + (h3 * 3)) * 4;
    if h < -150 {
        h = -150;
    }
    h
}

pub fn run(sistema: &mut SystemTable<Boot>, gl: &mut MtrxGl) {
    let mut cam_x = 0;
    let mut cam_y = 1500;
    let mut cam_z = 0;
    
    let mut pitch = 0;
    let mut yaw = 0;
    let mut roll = 0;
    let mut speed = 50;

    let cell_size = 350;
    let grid_radius = 12;
    let grid_width = grid_radius * 2 + 1;
    let mut pts = [(0, 0, false, 0); 625];

    loop {
        if let Ok(Some(evento)) = sistema.stdin().read_key() {
            match evento {
                Key::Special(ScanCode::ESCAPE) => {
                    sistema.runtime_services().reset(ResetType::COLD, uefi::Status::SUCCESS, None);
                },
                Key::Special(ScanCode::UP) => { pitch = (pitch - 4 + 256) % 256; },
                Key::Special(ScanCode::DOWN) => { pitch = (pitch + 4) % 256; },
                Key::Special(ScanCode::LEFT) => { 
                    yaw = (yaw - 4 + 256) % 256; 
                    roll = (roll + 8) % 256;
                },
                Key::Special(ScanCode::RIGHT) => { 
                    yaw = (yaw + 4) % 256; 
                    roll = (roll - 8 + 256) % 256;
                },
                Key::Printable(t) => {
                    let c = u16::from(t) as u8 as char;
                    if c == 'w' || c == 'W' { speed = (speed + 10).min(300); }
                    if c == 's' || c == 'S' { speed = (speed - 10).max(10); }
                    if c == 'q' || c == 'Q' {
                        sistema.runtime_services().reset(ResetType::COLD, uefi::Status::SUCCESS, None);
                    }
                }
                _ => {}
            }
        }

        if roll > 0 && roll < 128 {
            roll -= 1;
        } else if roll >= 128 {
            roll = (roll + 1) % 256;
        }

        cam_x -= (speed * isin(yaw) * icos(pitch)) >> 16;
        cam_y += (speed * isin(pitch)) >> 8;
        cam_z += (speed * icos(yaw) * icos(pitch)) >> 16;

        let ground_y = get_height(cam_x, cam_z);
        if cam_y < ground_y + 120 {
            cam_y = ground_y + 120;
        }

        gl.clear(0x050A1F);

        let base_x = (cam_x / cell_size) * cell_size;
        let base_z = (cam_z / cell_size) * cell_size;

        for j in 0..grid_width {
            let z = base_z + (j - grid_radius) * cell_size;
            for i in 0..grid_width {
                let x = base_x + (i - grid_radius) * cell_size;
                let y = get_height(x, z);
                let p3d = transform(Vec3 { x, y, z }, cam_x, cam_y, cam_z, pitch, yaw, roll);
                let pt = project(p3d, gl.width, gl.height);
                let idx = (j * grid_width + i) as usize;
                match pt {
                    Some((px, py)) => pts[idx] = (px, py, true, y),
                    None => pts[idx] = (0, 0, false, y),
                }
            }
        }

        for j in 0..grid_width {
            for i in 0..grid_width {
                let idx = (j * grid_width + i) as usize;
                let (px, py, valid, y) = pts[idx];
                if !valid { continue; }

                let color = if y <= -140 {
                    0x0044BB 
                } else if y <= -100 {
                    0x0088FF 
                } else if y > 350 {
                    0xFFFFFF 
                } else if y > 200 {
                    0x8B5A2B 
                } else {
                    0x00AA44 
                };

                if i < grid_width - 1 {
                    let idx_r = (j * grid_width + i + 1) as usize;
                    let (px2, py2, valid2, _) = pts[idx_r];
                    if valid2 { gl.draw_line(px, py, px2, py2, color); }
                }
                if j < grid_width - 1 {
                    let idx_d = ((j + 1) * grid_width + i) as usize;
                    let (px2, py2, valid2, _) = pts[idx_d];
                    if valid2 { gl.draw_line(px, py, px2, py2, color); }
                }
            }
        }

        let cx = gl.width / 2;
        let cy = gl.height / 2;

        gl.draw_line(cx - 30, cy, cx - 10, cy, 0x00FF00);
        gl.draw_line(cx + 10, cy, cx + 30, cy, 0x00FF00);
        gl.draw_line(cx, cy - 10, cx, cy - 5, 0x00FF00);

        let roll_offset_x = (isin(roll) * 50) >> 8;
        let roll_offset_y = (icos(roll) * 50) >> 8;
        let horizon_y = cy + ((isin(pitch) * 150) >> 8);

        gl.draw_line(
            cx - roll_offset_y, 
            horizon_y - roll_offset_x, 
            cx + roll_offset_y, 
            horizon_y + roll_offset_x, 
            0xFF00FF
        );

        let speed_bar_h = (speed * 100) / 300;
        gl.draw_rect(20, cy - 50, 10, 100, 0xFFFFFF);
        gl.fill_rect(20, cy + 50 - speed_bar_h, 10, speed_bar_h, 0x00FFFF);

        let alt_bar_h = (cam_y.max(0) * 100) / 4000;
        let alt_bar_h_clamped = alt_bar_h.min(100);
        gl.draw_rect(gl.width - 30, cy - 50, 10, 100, 0xFFFFFF);
        gl.fill_rect(gl.width - 30, cy + 50 - alt_bar_h_clamped, 10, alt_bar_h_clamped, 0xFF0000);

        sistema.boot_services().stall(20_000);
    }
}