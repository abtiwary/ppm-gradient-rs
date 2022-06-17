use std::{fs::File, io::Write};
use std::env;
use std::process;

fn main() {
    //let outfile_path = String::from("/Users/abtiwary/temp/outfile_test_1.ppm");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please specify a valid path for the output file!");
        process::exit(1);
    }
    let outfile_path = String::from(&args[1]);
    
    let width: u16 = 200;
    let height: u16 = 100;

    let mut header: Vec<u8> = Vec::new();
    String::from("P6\n")
        .as_bytes()
        .iter()
        .for_each(|b| { header.push(*b); });

    String::from(format!("{} {}\n", width, height))
        .as_bytes()
        .iter()
        .for_each(|b| { header.push(*b); });

    String::from("255\n")
        .as_bytes()
        .iter()
        .for_each(|b| { header.push(*b); });
    
    let mut image_data: Vec<u8> = vec![0; 3 * width as usize * height as usize];
    let mut image_data_idx = 0;
    for y in (0..height).rev() {
        for x in 0..width {
            let r: f32 = f32::from(x) / f32::from(width);
            let g: f32 = f32::from(y) / f32::from(height);
            let b: f32 = 0.2;
            let ir: u8 = (255.99 * r) as u8;
            let ig: u8 = (255.99 * g) as u8;
            let ib: u8 = (255.99 * b) as u8;

            //let offset: usize = (x * 3) as usize + (3 * y as usize * width as usize);
            image_data[image_data_idx] = ir;
            image_data[image_data_idx+1] = ig;
            image_data[image_data_idx+2] = ib;
            image_data_idx += 3;
        }
    }

    println!("{:?}", &image_data[0..32]);

    let mut f = File::create(outfile_path).unwrap();
    f.write(&header);
    f.write(&image_data);

    println!("done!");
}
