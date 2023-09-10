use std::{fs,cmp};
use nannou::prelude::*;
use chrono::{DateTime,Local};
use nannou::image::{DynamicImage,ImageBuffer,};
use rand::Rng;

//color_settings
const P_COLOR : f32 = 0.7;
const BG_COLOR : f32 = 1.0;
//output_settings
const FRAME_LEN : u32 = 50;
const FRAME_RATE : u32 = 10;
const SAVE : bool = false;
const OUTPUT_DISCRIPTION : &str = "hourgalss";

//iteration_num
const N: usize = 1000;
//img_scale
const SCALE: f32 = 200.;
//sample_num
const SAMPLES: usize = 1000;
//initial k
const INI_K : f32 = 0.0;
//add_to_k_each_iteration
const ADD : f32 = 0.01;

const WIDTH: u32 = (2. * PI * SCALE) as u32 + 1;
const HEIGHT: u32 = (2. * PI * SCALE) as u32 + 1;

struct Model{
    runtime : DateTime<Local>,
    img : DynamicImage,
    k : f32,
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let runtime : DateTime<Local> = Local::now();

    let rgba_img = ImageBuffer::new(WIDTH, HEIGHT);

    let img = DynamicImage::ImageRgba8(rgba_img);

    //create new window
    app.new_window().size(WIDTH, HEIGHT).view(view).build().unwrap();

    let k : f32 = INI_K;


    Model { 
        runtime, 
        img, 
        k,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let frame_interval : u64 = 60/FRAME_RATE as u64;

    //adjust update span
    if app.elapsed_frames() % frame_interval != 0 {
        return;
    }

    let mut rgba_img = 
        model.img
            .clone()
            .into_rgba8();

    //reset color
    let back_color = (BG_COLOR * 255.) as u8;

    for (_x, _y, pixel) in rgba_img.enumerate_pixels_mut() {
        *pixel = nannou::image::Rgba([back_color * 0, back_color, back_color, 0]);
    }

    let k = model.k;
    let mut rng = rand::thread_rng();
    for _ in 0..SAMPLES {
        let mut theta = rng.gen::<f32>() * 2. * PI;
        let mut p = rng.gen::<f32>();
        for _ in 0..N {
            p = p + k * (theta).sin();

            theta = theta * p + p.sin();

            //theta = ((p*theta/k).sin() * p + k.sin()) * p * p.sin() + (k/theta).sin();//hourglass
            //theta = theta + p + k * theta * p;//flash
            //theta = theta + (p/k).sin() * p; //blend
            //theta = theta*(k/theta).sin() +(1.5*k).cos()* p; //strings3

            while p > PI {
                p -= 2. * PI;
            }
            while p < -PI {
                p += 2. * PI;
            }   

            while theta > PI {
                theta -= 2. * PI;
            }
            while theta < -PI {
                theta += 2. * PI;
            }

            let x = ((theta + PI) * SCALE).floor() as u32;
            let y = ((p + PI) * SCALE).floor() as u32;

            let point_color = (P_COLOR * 255.) as u8;

            let color = *rgba_img.get_pixel(x,y);

            let now_red = color.0[0] as u32;
            let now_alpha = color.0[3] as u32;

            let new_red = cmp::min(point_color as u32,now_red + 30);
            let new_alpha = cmp::min(255,now_alpha + 32);

            rgba_img.put_pixel(x,y, nannou::image::Rgba([new_red as u8, point_color, point_color, new_alpha as u8]));
            
        }
    }

    model.img = DynamicImage::ImageRgba8(rgba_img);
    model.k += ADD;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let frame_interval : u64 = 60/FRAME_RATE as u64;

    //adjust fps
    if app.elapsed_frames() % frame_interval != 0 {
        return;
    }

    //seqential number
    let now_frame : u64 = app.elapsed_frames() / frame_interval;

    let img = model.img.clone();

    //output in nannou window
    let texture = wgpu::Texture::from_image(app,&img);
    let draw = app.draw();
    frame.clear(BLACK);
    draw.texture(&texture);

    //output seqential png
    let seq_num_string = {
        let i : u64 = now_frame;

        if i < 10 {format!("000{}",i)}
        else if i < 100 {format!("00{}",i)}
        else if i < 1000 {format!("0{}",i)}
        else {format!("{}",i)}
    };

    if SAVE == true {
        let time_string :String  = model.runtime.format("%Y_%m_%d_%H_%M_%S").to_string(); 

        if now_frame <= 1 {
            fs::create_dir(format!("./assets/output/{}_{}", time_string, OUTPUT_DISCRIPTION,))
                .expect("Failed to create directory");
        }

        let file_name : String = format!("./assets/output/{}_{}/{}_{}_{}.png", time_string, OUTPUT_DISCRIPTION, time_string, OUTPUT_DISCRIPTION, seq_num_string);

        if now_frame < FRAME_LEN as u64{
            img.save(&file_name).unwrap();
            println!("{}",seq_num_string);
        }

    }

    draw.to_frame(app, &frame).unwrap();
}
