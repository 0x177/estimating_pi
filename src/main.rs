use raylib::prelude::*;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut coprime_count : f32 = 0.0;
    let mut total: f32 = 0.0;

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    
    rl.set_target_fps(30);
    while !rl.window_should_close() {

        let a: i32 = rng.gen_range(00000..99999);
        let b: i32 = rng.gen_range(00000..99999);

        let result: f32 = gcd(a as f32,b as f32);

        if result == 1.0 {
            coprime_count += 1.0;
        }
        total += 1.0;

        let pie = ((6.0 * total) / coprime_count).sqrt();
 

        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::BLACK);
        d.draw_text(a.to_string().as_str(), 12, 12, 20, Color::PINK);
        d.draw_text(b.to_string().as_str(), 12, 40, 20, Color::PINK);
        d.draw_text(result.to_string().as_str(), 90, 12, 20, Color::PINK);
        d.draw_text(pie.to_string().as_str(),140,12,20,Color::PINK);
        d.draw_text("ESTIMATING PI...",0,240,50,Color::PINK);
    }
}


fn gcd( a: f32, b: f32) -> f32 {
    if b > a {
        gcd(b,a);
    }
    let r = a % b;
    if r == 0.0 {
        return b;
    } else {
        return gcd(b,r);
    }

}
