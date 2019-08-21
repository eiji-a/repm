

use std::io;
use std::env;

const ORESO: u32 = 8;
const VRESO: u32 = 2;
const RADIUS0: f64 = 10.0;

type Fingerprint = String;
type FpUnit = f64;
type ColorUnit = f64;
type Channel = [ColorUnit];
type Color = (f64, f64, f64);

enum Status {
    FILED, PEND, DISCARDED, DUPLICATED, INFERIOR,
}

struct ColorVector {
    pixel:  [Color; (VRESO * VRESO) as usize],
    imgid:  u32,
    fp:     String,
    status: Status,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Usage: repm <rad0> <rad1> <rate>")
    }

    let (r0, r1, rt) = parse_config(&args);
    eprintln!("R0:{:.2}/R1:{:.2}/Rate:{:.2}", r0, r1, rt);
    let img_filter   = make_image_filter(ORESO, VRESO);

    let cvs: Vec<(ColorVector, bool)> = read_image();
    eprintln!("nimages: {}", cvs.len());
    for (c, b) in &cvs {
        println!("ID:{}/st:{}", c.imgid, status2str(&c.status));
    }

}

fn parse_config(args: &[String])  -> (ColorUnit, FpUnit, f64) {
    let r0: f64 = args[1].parse::<f64>().unwrap();
    let r1: f64 = args[2].parse::<f64>().unwrap();
    let rt: f64 = args[3].parse::<f64>().unwrap();

    (r0, r1, rt)
}

fn make_image_filter(or: u32, vr: u32) -> [u8; 64] {
    [0,0,0,0,1,1,1,1,
     0,0,0,0,1,1,1,1,
     0,0,0,0,1,1,1,1,
     0,0,0,0,1,1,1,1,
     2,2,2,2,3,3,3,3,
     2,2,2,2,3,3,3,3,
     2,2,2,2,3,3,3,3,
     2,2,2,2,3,3,3,3]
}

fn read_image() -> Vec<(ColorVector, bool)> {
    let mut cs: Vec<(ColorVector, bool)> = Vec::new();

    loop {
      let mut input = String::new();
      match io::stdin().read_line(&mut input) {
          Ok(n) => {
              if n == 0 {
                  break;
              }
              cs.push(make_colorvector(&input.trim_end()));
          }
          Err(e) => panic!("I/O error:{}", &e),
      }
    }

    cs
}

fn make_colorvector(img: &str) -> (ColorVector, bool) {
    let iv: Vec<&str> = img.split("|").collect();

    let cv = ColorVector {
        pixel:  [(1.0,2.0,3.0), (4.0, 5.0, 6.0), (7.0,8.0,9.0), (1.0,2.0,3.0)],
        imgid:  iv[0].parse().unwrap(),
        fp:     iv[1].to_string(),
        status: str2status(iv[2]),
    };

    (cv, true)
}

fn str2status(st: &str) -> Status {
    match st {
        "filed"      => Status::FILED,
        "pending"    => Status::PEND,
        "discarded"  => Status::DISCARDED,
        "duplicated" => Status::DUPLICATED,
        "inferior"   => Status::INFERIOR,
        _            => Status::PEND,
    }
}

fn status2str(st: &Status) -> &'static str {
    match st {
        Status::FILED      => "filed",
        Status::PEND       => "pending",
        Status::DISCARDED  => "discarded",
        Status::DUPLICATED => "duplicated",
        Status::INFERIOR   => "inferior",
        _                  => "pending",
    }
}
