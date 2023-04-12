use super::Netpbm;

pub fn print_netppm(p: &Netpbm) {
    for i in 0..p.header.height {
        for j in (0..p.header.width * 3).step_by(3) {
            let (r, g, b): (u32, u32, u32);

            if p.header.bit_depth == 16 {
                // clamping to 0-255, but still supporting 0-65535
                r = p.data[(i * 3 * p.header.width + j + 0) as usize] * 65535
                    / p.header.max_value
                    / 255;
                g = p.data[(i * 3 * p.header.width + j + 1) as usize] * 65535
                    / p.header.max_value
                    / 255;
                b = p.data[(i * 3 * p.header.width + j + 2) as usize] * 65535
                    / p.header.max_value
                    / 255;
            } else {
                r = (p.data[(i * 3 * p.header.width + j + 0) as usize] * 255 / p.header.max_value)
                    as u32;
                g = (p.data[(i * 3 * p.header.width + j + 1) as usize] * 255 / p.header.max_value)
                    as u32;
                b = (p.data[(i * 3 * p.header.width + j + 2) as usize] * 255 / p.header.max_value)
                    as u32;
            }

            print!("\x1b[48;2;{r};{g};{b}m  \x1b[m");
        }
        println!();
    }
}
