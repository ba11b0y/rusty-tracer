const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    render(IMAGE_WIDTH, IMAGE_HEIGHT);
}

fn render(width:u32, height:u32){
    println!("P3\n{width} {height}\n255\n");

    for j in 0..height{
        for i in 0..width{
            let r = i as f32/(width-1) as f32;
            let g = j as f32/(height-1) as f32;
            let b = 0.0;

            let ir = (255.999*r) as u32;
            let ig = (255.999*g) as u32;
            let ib = (255.999*b) as u32;

            println!("{ir} {ig} {ib}\n");
        }
    }
}
