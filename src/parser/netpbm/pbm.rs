use super::Netpbm;

pub fn print_netpbm_p1(p: &Netpbm, r: u8, g: u8, b: u8) {
    for i in 0..p.header.height {
        for j in 0..p.header.width {
            let pix = p.data[(i * p.header.width + j) as usize];

            if pix > 0 {
                print!("\x1b[48;2;{r};{g};{b}m  ");
            } else {
                print!("\x1b[0m  ");
            }
        }
        println!();
    }

    print!("\x1b[0m  ");
}

pub fn print_netpbm_p4(p: &Netpbm, r: u8, g: u8, b: u8) {
    let width_in_bytes = (p.header.width + 7) / 8;
    for i in 0..p.header.height {
        for j in 0..p.header.width {
            let byte = p.data[(i * width_in_bytes + j) as usize];

            for k in 7..=0 {
                if byte & (1 << k) > 0 {
                    print!("\x1b[48;2;{r};{g};{b}m  ");
                } else {
                    print!("\x1b[0m  ");
                }
            }
        }
        println!();
    }

    print!("\x1b[0m  ");
}
