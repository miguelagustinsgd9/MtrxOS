use crate::include::*;
pub struct Sdmll {
    pub width: usize,
    pub height: usize,
    pub fg: Color,
    pub bg: Color,
}

impl Sdmll {
    pub fn init(sistema: &mut SystemTable<Boot>) -> Self {
        let (w, h) = if let Ok(Some(modo)) = sistema.stdout().current_mode() {
            (modo.columns(), modo.rows())
        } else {
            (80, 25)
        };
        let _ = sistema.stdout().enable_cursor(false);
        Self {
            width: w,
            height: h,
            fg: Color::White,
            bg: Color::Black,
        }
    }

    pub fn set_color(&mut self, sistema: &mut SystemTable<Boot>, fg: Color, bg: Color) {
        self.fg = fg;
        self.bg = bg;
        let _ = sistema.stdout().set_color(fg, bg);
    }

    pub fn clear(&self, sistema: &mut SystemTable<Boot>) {
        let _ = sistema.stdout().clear();
    }

    pub fn draw_pixel(&self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, ch: char) {
        if x < self.width && y < self.height {
            let _ = sistema.stdout().set_cursor_position(x, y);
            let _ = write!(sistema.stdout(), "{}", ch);
        }
    }

    pub fn draw_text(&self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, text: &str) {
        if y >= self.height { return; }
        let mut cx = x;
        for c in text.chars() {
            if cx >= self.width { break; }
            let _ = sistema.stdout().set_cursor_position(cx, y);
            let _ = write!(sistema.stdout(), "{}", c);
            cx += 1;
        }
    }

    pub fn h_line(&self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, len: usize, ch: char) {
        if y >= self.height || x >= self.width || len == 0 { return; }
        let end = (x + len).min(self.width);
        let _ = sistema.stdout().set_cursor_position(x, y);
        for _ in x..end {
            let _ = write!(sistema.stdout(), "{}", ch);
        }
    }

    pub fn v_line(&self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, len: usize, ch: char) {
        if x >= self.width || y >= self.height || len == 0 { return; }
        let end = (y + len).min(self.height);
        for cy in y..end {
            let _ = sistema.stdout().set_cursor_position(x, cy);
            let _ = write!(sistema.stdout(), "{}", ch);
        }
    }

    pub fn draw_line(&self, sistema: &mut SystemTable<Boot>, x0: usize, y0: usize, x1: usize, y1: usize, ch: char) {
        let mut x = x0 as isize;
        let mut y = y0 as isize;
        let end_x = x1 as isize;
        let end_y = y1 as isize;
        let dx = (end_x - x).abs();
        let sx = if x < end_x { 1 } else { -1 };
        let dy = -(end_y - y).abs();
        let sy = if y < end_y { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                let _ = sistema.stdout().set_cursor_position(x as usize, y as usize);
                let _ = write!(sistema.stdout(), "{}", ch);
            }
            if x == end_x && y == end_y { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_rect(&self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, w: usize, h: usize) {
        if x >= self.width || y >= self.height || w < 2 || h < 2 { return; }
        let end_x = (x + w - 1).min(self.width - 1);
        let end_y = (y + h - 1).min(self.height - 1);
        let act_w = end_x - x + 1;
        let act_h = end_y - y + 1;

        self.draw_pixel(sistema, x, y, '┌');
        self.draw_pixel(sistema, end_x, y, '┐');
        self.draw_pixel(sistema, x, end_y, '└');
        self.draw_pixel(sistema, end_x, end_y, '┘');

        if act_w > 2 {
            self.h_line(sistema, x + 1, y, act_w - 2, '─');
            self.h_line(sistema, x + 1, end_y, act_w - 2, '─');
        }
        if act_h > 2 {
            self.v_line(sistema, x, y + 1, act_h - 2, '│');
            self.v_line(sistema, end_x, y + 1, act_h - 2, '│');
        }
    }

    pub fn fill_rect(&self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, w: usize, h: usize, ch: char) {
        if x >= self.width || y >= self.height || w == 0 || h == 0 { return; }
        let end_y = (y + h).min(self.height);
        for cy in y..end_y {
            self.h_line(sistema, x, cy, w, ch);
        }
    }

    pub fn draw_shadow(&self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, w: usize, h: usize) {
        let prev_fg = self.fg;
        let prev_bg = self.bg;
        let _ = sistema.stdout().set_color(Color::DarkGray, Color::Black);
        self.h_line(sistema, x + 1, y + h, w, '░');
        self.v_line(sistema, x + w, y + 1, h, '░');
        self.draw_pixel(sistema, x + w, y + h, '░');
        let _ = sistema.stdout().set_color(prev_fg, prev_bg);
    }

    pub fn draw_window(&mut self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, w: usize, h: usize, tit: &str, txt: &str, fg: Color, bg: Color) {
        self.set_color(sistema, fg, bg);
        self.fill_rect(sistema, x, y, w, h, ' ');
        self.draw_rect(sistema, x, y, w, h);

        let mut t_len = 0;
        for _ in tit.chars() { t_len += 1; }

        if t_len > 0 && w > 2 {
            let max_t = t_len.min(w - 2);
            let tx = x + (w - max_t) / 2;
            self.draw_pixel(sistema, tx.saturating_sub(1), y, ' ');
            let mut offset = 0;
            for c in tit.chars().take(max_t) {
                self.draw_pixel(sistema, tx + offset, y, c);
                offset += 1;
            }
            self.draw_pixel(sistema, tx + max_t, y, ' ');
        }

        let mut cy = y + 2;
        for line in txt.split('\n') {
            if cy >= y + h - 1 { break; }
            self.draw_text(sistema, x + 2, cy, line);
            cy += 1;
        }
    }

    pub fn draw_button(&mut self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, txt: &str, sel: bool, def_fg: Color, def_bg: Color) {
        if sel {
            self.set_color(sistema, Color::Black, Color::LightGray);
        } else {
            self.set_color(sistema, def_fg, def_bg);
        }
        if x < self.width && y < self.height {
            let _ = sistema.stdout().set_cursor_position(x, y);
            let _ = write!(sistema.stdout(), "[ {} ]", txt);
        }
    }

    pub fn draw_progress(&mut self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, w: usize, pct: usize) {
        if w < 2 || x >= self.width || y >= self.height { return; }
        let pct = pct.min(100);
        let f_w = w - 2;
        let p_w = (f_w * pct) / 100;

        let prev_fg = self.fg;
        let prev_bg = self.bg;

        self.set_color(sistema, Color::White, prev_bg);
        self.draw_pixel(sistema, x, y, '[');
        self.draw_pixel(sistema, x + w - 1, y, ']');

        self.set_color(sistema, Color::Green, prev_bg);
        if p_w > 0 {
            self.h_line(sistema, x + 1, y, p_w, '█');
        }

        self.set_color(sistema, Color::DarkGray, prev_bg);
        if f_w > p_w {
            self.h_line(sistema, x + 1 + p_w, y, f_w - p_w, '-');
        }

        self.set_color(sistema, prev_fg, prev_bg);
    }

    pub fn draw_status_bar(&mut self, sistema: &mut SystemTable<Boot>, text: &str) {
        let y = self.height.saturating_sub(1);
        self.set_color(sistema, Color::Black, Color::Cyan);
        self.h_line(sistema, 0, y, self.width, ' ');
        self.draw_text(sistema, 1, y, text);

        if let Ok(time) = sistema.runtime_services().get_time() {
            let h = time.hour();
            let m = time.minute();
            let tx = self.width.saturating_sub(6);
            if tx > 0 {
                let _ = sistema.stdout().set_cursor_position(tx, y);
                let _ = write!(sistema.stdout(), "{:02}:{:02}", h, m);
            }
        }
    }

    pub fn draw_list_box(&mut self, sistema: &mut SystemTable<Boot>, x: usize, y: usize, w: usize, h: usize, items: &[&str], selected: usize, fg: Color, bg: Color) {
        self.set_color(sistema, fg, bg);
        self.fill_rect(sistema, x, y, w, h, ' ');
        self.draw_rect(sistema, x, y, w, h);

        let end_y = (y + h - 1).min(self.height);
        let mut cy = y + 1;

        for (i, item) in items.iter().enumerate() {
            if cy >= end_y { break; }
            if i == selected {
                self.set_color(sistema, Color::Black, Color::White);
            } else {
                self.set_color(sistema, fg, bg);
            }
            let _ = sistema.stdout().set_cursor_position(x + 1, cy);
            let mut cx = x + 1;
            let _ = write!(sistema.stdout(), " ");
            cx += 1;
            for c in item.chars() {
                if cx >= x + w - 2 { break; }
                let _ = write!(sistema.stdout(), "{}", c);
                cx += 1;
            }
            while cx < x + w - 1 {
                let _ = write!(sistema.stdout(), " ");
                cx += 1;
            }
            cy += 1;
        }
    }
}
