use raqote::*;
use png;

fn main() {
    let path = "./src/example.png";
    let file = std::fs::File::open(path).unwrap();

    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();

    let mut image: Vec<u32> = Vec::new();
    // Vec<u8>RGBA -> Vec<u32>ARGB
    for i in buf.chunks(4) {
        // Original alpha channel
        // image.push(((i[3] as u32) << 24) | ((i[0] as u32) << 16) | (i[1] as u32) << 8 | (i[2] as u32));

        // Alpha channel to 0xff
        image.push((0xff << 24) | ((i[0] as u32) << 16) | (i[1] as u32) << 8 | (i[2] as u32));
    }

    let src = Source::Image(
        Image {
            width: info.width as i32,
            height: info.height as i32,
            data: &image[..],
        },
        ExtendMode::Pad,
        FilterMode::Nearest,
        Transform::default(),
    );

    let mut dt = DrawTarget::new(info.width as i32, info.height as i32);
    dt.clear(SolidSource::from_unpremultiplied_argb(
        0xff, 0xff, 0xff, 0xff,
    ));

    dt.fill_rect(0., 0., info.width as f32, info.height as f32, &src, &DrawOptions::new());

    dt.write_png("./src/out.png").unwrap();
}