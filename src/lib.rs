use std::io::Read;
use std::io;

const TGA_HEADER_LENGTH: usize = 18;


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TgaHeader {
    id_length: u8,
    color_map_type: u8,
    data_type_code: u8,
    colour_map_origin: [u8; 2],
    colour_map_length: [u8; 2],
    colour_map_depth: u8,
    x_origin: [u8; 2],
    y_origin: [u8; 2],
    width: [u8; 2],
    height: [u8; 2],
    bits_per_pixel: u8,
    image_descriptor: u8,
}

impl TgaHeader {
    fn parse_from_buffer(buf: &[u8]) -> Option<TgaHeader> {
        if buf.len() >= TGA_HEADER_LENGTH {
            // The buffer must be at least the length (in bytes) of a TGA header.
            let header = TgaHeader {
                id_length: buf[0],
                color_map_type: buf[1],
                data_type_code: buf[2],
                colour_map_origin: [buf[3], buf[4]],
                colour_map_length: [buf[5], buf[6]],
                colour_map_depth: buf[7],
                x_origin: [buf[8], buf[9]],
                y_origin: [buf[10], buf[11]],
                width: [buf[12], buf[13]],
                height: [buf[14], buf[15]],
                bits_per_pixel: buf[16],
                image_descriptor: buf[17],
            };

            return Some(header);
        }

        None
    }

    fn colour_map_size(&self) -> u16 {
        let colour_map_length = (self.colour_map_length[1] << 8) as u16 
                              | self.colour_map_length[0] as u16;

        // From the TGA specification, the color map depth will be one of
        // 16, 24, or 32 bits. That is, it is always a multiple of 8.
        let colour_map_depth_bytes = (self.colour_map_depth / 8) as u16;

        colour_map_length * colour_map_depth_bytes
    }

    fn width(&self) -> usize {
        (((self.width[1] << 8) as u16) | (self.width[0] as u16)) as usize
    }

    fn height(&self) -> usize {
        (((self.height[1] << 8) as u16) | (self.height[0] as u16)) as usize
    }
}

pub struct TgaImage {
    header: TgaHeader,
    image_identification: Box<Vec<u8>>,
    colour_map_data: Box<Vec<u8>>,
    image_data: Box<Vec<u8>>,
}

impl TgaImage {
    pub fn new(
        header: TgaHeader, 
        image_identification: Box<Vec<u8>>, 
        colour_map_data: Box<Vec<u8>>, 
        image_data: Box<Vec<u8>>
    ) -> TgaImage {
        TgaImage {
            header: header, 
            image_identification: image_identification, 
            colour_map_data: colour_map_data, 
            image_data: image_data
        }
    }

    pub fn parse_from_buffer(buf: &[u8]) -> Result<TgaImage, TgaError> {
        let header = TgaHeader::parse_from_buffer(buf).unwrap();

        // Check the header.
        if header.data_type_code != 2 {
            return Err(
                // Fail here.
            );
        }

        if header.bits_per_pixel != 24 {
            return Err(
                // Fail here.
            );
        }

        if buf.len() < header.id_length as usize + TGA_HEADER_LENGTH {
            return Err(
                // Fail here.
            );
        }

        let slice = &buf[TGA_HEADER_LENGTH..buf.len()];
        let mut bytes = slice.bytes();
        let mut image_identification = Box::new(Vec::new());
        for _ in 0..header.id_length {
            let byte = bytes.next();
            match byte {
                Some(val) => image_identification.push(val.unwrap()),
                None => {
                    return Err(
                        // Fail here.
                    );
                }
            }
        }

        let colour_map_size = header.colour_map_size();
        
        let mut colour_map_data = Box::new(Vec::new());
        for _ in 0..colour_map_size {
            let byte = bytes.next();
            match byte {
                Some(val) => colour_map_data.push(val.unwrap()),
                None => {
                    return Err(
                        // Fail here.
                    );
                }
            }
        }

        let width = header.width();
        let height = header.height();
        let bytes_per_pixel = (header.bits_per_pixel / 8) as usize;
        let image_size = width * height * bytes_per_pixel;
        
        let mut image_data = Box::new(Vec::new());
        for _ in 0..image_size {
            let byte = bytes.next();
            match byte {
                Some(val) => image_data.push(val.unwrap()),
                None => {
                    return Err(
                        // Fail here.
                    );
                }
            }
        }

        let image = TgaImage::new(
            header, image_identification, colour_map_data, image_data
        );

        Ok(image)
    }
}
