use rs_ws281x::*;

// LED array format is BGRA in u8

pub fn init() -> Controller {
    ControllerBuilder::new()
        .freq(800_000) // default freq: 800_000
        .dma(10) // default DMA: 10
        .channel(
            0,
            ChannelBuilder::new()
                .pin(10) // default GPIO: 10 (SPI0 MOSI)
                .count(64)
                .strip_type(StripType::Ws2812)
                .brightness(3) // default: 255
                .build(),
        )
        .build()
        .unwrap()
}

// let leds = controller.leds_mut(0);

// // program leds array with RGBA value
// for led in leds {
//     *led = [0, 0, 255, 0];
// }

// // draw
// controller.render().unwrap();
