use super::PrintableFormat;

pub struct Xbm {
    data : Vec<u8>,
    width : u8,
    height : u8,
}

impl Xbm {
    pub fn parse(input: &str) -> Xbm {
        print!("{}", input);
        let xbm = Xbm { data: vec![], width: 0, height: 0};

// TODO from C impl
//   end = data + len;

//   xptr->width = parse_str_int(data, len, "_width");
//   xptr->height = parse_str_int(data, len, "_height");

//   next = memchr(data, '{', len); // get substring where data start
//   if(next == NULL)
//     return -1;

//   ptr = next + 1;

//   xptr->data = (char *)malloc(xptr->width * xptr->height);
//   if(data == NULL)
//     return -1;
// 	long i = 0;
//   while(ptr < end) {
// 		if(ptr == NULL) {
// 			break;
// 		}
//     while(*ptr != 'x') {
//       ptr++;
// 			if(ptr >= end) {
// 				break;
// 			}
//     }
//     char x = (char)strtol(ptr + 1, NULL, 16);
//     xptr->data[i++] = x;
//     ptr++;
//   }

        xbm
    }
}

impl PrintableFormat for Xbm {
    fn print(&self) {
        print!("TODO");
    }
}
