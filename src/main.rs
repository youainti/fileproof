//use std::env;
//use std::fs::File;
use std::io::Write;

extern crate qrcodegen;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::{Read};

const BUFFER_SIZE: usize = 1024;

fn write_svg(filename: &str, svg_contents: &str){
    //!Write the SVG file

    //Create file
    let mut file = std::fs::File::create(&filename).expect("create failed");

    file.write_all(&svg_contents.as_bytes()).expect("write failed");
    println!("QR-code written to {}", &filename );
}

fn create_qrcode_from_text(text: &str, ecc: QrCodeEcc) -> String {
    //!create an svg qr code from a given text.
    //Create a qr code
    let qr = QrCode::encode_text(&text, ecc).unwrap();
    return qr.to_svg_string(4);
}

fn checksum_calc<D: Digest + Default, R: Read>(reader: &mut R) -> String {
    //! This calculates the checksum using whatever format is given. 
    //! The buffered read was taken from the SHASUM package.
    //! I found the formatting bit (from u8 to a hex string) online somewhere.

    //Create some sort of object.
    let mut sh = D::default();
    //Set up a buffer 
    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
            // Read data into the buffer (checking for errors.
            let n = match reader.read(&mut buffer) {
                Ok(n) => n,
                Err(_) => panic!("Error reading the buffer."),
            };

            //Add the data
            sh.update(&buffer[..n]);
            //check for the end of the file (bitstream) by seeing if the buffer was full.
            if n == 0 || n < BUFFER_SIZE {
                break;
        }
    }

    //Take the sha2 object, finalize it, and then turn it into a HEX string.
    return sh.finalize()
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<String>();

}


fn print_help() {
    //!This is to display a help function
    //todo
    print!("
           sdfsdfssdf
           "
        )
}



fn main() {
    //Get the files to checksum
    let args = env::args();
    // Process files listed in command line arguments one by one
    // If no files provided process input from stdin
    if args.len() > 1 {
        for path in args.skip(1) {
            if let Ok(mut file) = fs::File::open(&path) {
                //get the sum
                let sum = checksum_calc::<Sha256, _>(&mut file);


                //create qr code
                let svg: String = create_qrcode_from_text(&format!("{} {}", &sum, &path), 
                                                          QrCodeEcc::High);

                //Create the path for the svg
                let svg_path  = format!("{}-sum.{}", path, "svg");

                //write the svg file (testing for now)
                write_svg(&svg_path, &svg);
            }
        }
    }

}
