use super::Netpbm;

pub fn print_netpgm(p: &Netpbm) {
    for i in 0..p.header.height {
        for j in 0..p.header.width {
            let pix: u32;

            if p.header.bit_depth == 16 {
                pix = p.data[(i * p.header.width + j) as usize] * 65535 / p.header.max_value / 255;
            } else {
                pix = (p.data[(i * p.header.width + j) as usize] * 255 / p.header.max_value) as u32;
            }

            print!("\x1b[48;2;{pix};{pix};{pix}m  ");
        }
        println!();
    }

    print!("\x1b[0m  ");
}
