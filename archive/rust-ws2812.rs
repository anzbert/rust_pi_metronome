use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::spi;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;

fn main() -> Result<(), Box<dyn Error>> {
    // const NUM_LEDS: usize = 64;

    let spi = spi::Spi::new(
        spi::Bus::Spi0,
        spi::SlaveSelect::Ss0,
        3_000_000,
        spi::Mode::Mode0,
    )?;
    // let mut buffer: [u8; 255] = [0; 255];
    let mut ws = Ws2812::new(spi);
    let mut data: [RGB8; 3] = [RGB8::default(); 3];
    let empty: [RGB8; 3] = [RGB8::default(); 3];

    data[0] = RGB8 {
        r: 0,
        g: 0,
        b: 0x10,
    };
    data[1] = RGB8 {
        r: 0,
        g: 0x10,
        b: 0,
    };
    data[2] = RGB8 {
        r: 0x10,
        g: 0,
        b: 0,
    };

    println!("starting loop...");
    loop {
        let cloned_data = data.iter().cloned();
        println!("cloning");
        match ws.write(cloned_data) {
            Ok(x) => println!("{:?}", x),
            Err(e) => println!("{:?}", e),
        }
        println!("write high");
        thread::sleep(Duration::from_millis(1000));
        ws.write(empty.iter().cloned()).unwrap();
        println!("write low");
        thread::sleep(Duration::from_millis(1000));
    }

    #[allow(unreachable_code)]
    Ok(())
}
