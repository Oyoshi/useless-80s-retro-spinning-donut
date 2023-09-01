/********************** Constants **********************/

const DELTA_THETA: f32 = 0.01;
const DELTA_PHI: f32 = 0.01;

const R1: f32 = 1.0;
const R2: f32 = 2.0;
const K1: f32 = (SCREEN_WIDTH as f32) * K2 * 3.0 / (8.0 * (R1 + R2));
const K2: f32 = 5.0;

const SCREEN_WIDTH: u32 = 36;
const SCREEN_HEIGHT: u32 = 36;

const SYMBOLS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

const A0: f32 = 1.0;
const DELTA_A: f32 = 0.04;
const B0: f32 = 1.0;
const DELTA_B: f32 = 0.015;

/*******************************************************/

pub fn simulate() {
    let mut a = A0;
    let mut b = B0;
    print!("\x1b[2J");
    loop {
        render_frame(a, b);
        a += DELTA_A;
        b += DELTA_B;
    }
}

fn render_frame(a: f32, b: f32) {
    let char_output = compute_frame(a, b);
    print!("\x1b[H");
    for i in 0..SCREEN_HEIGHT {
        for j in 0..SCREEN_WIDTH {
            print!("{}", char_output[i as usize][j as usize]);
        }
        println!();
    }
}

fn compute_frame(a: f32, b: f32) -> [[char; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize] {
    let mut char_output = [[' '; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
    let mut z_buffer = [[0.0f32; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

    let sin_a = a.sin();
    let cos_a = a.cos();
    let sin_b = b.sin();
    let cos_b = b.cos();

    let mut theta = 0.0;
    while theta < 2.0 * std::f32::consts::PI {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        theta += DELTA_THETA;

        let mut phi = 0.0;
        while phi < 2.0 * std::f32::consts::PI {
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();
            phi += DELTA_PHI;

            let circle_x = R2 + R1 * cos_theta;
            let circle_y = R1 * sin_theta;

            let x =
                circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi) - circle_y * cos_a * sin_b;
            let y =
                circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi) + circle_y * cos_a * cos_b;
            let z = K2 + cos_a * circle_x * sin_phi + circle_y * sin_a;
            let ooz = 1.0 / z;

            let xp = ((SCREEN_WIDTH as f32) / 2.0 + K1 * ooz * x) as usize;
            let yp = ((SCREEN_HEIGHT as f32) / 2.0 - K1 * ooz * y) as usize;

            let luminance =
                cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta
                    + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);
            if luminance > 0.0 && ooz > z_buffer[xp][yp] {
                z_buffer[xp][yp] = ooz;
                char_output[xp][yp] = compute_char(luminance);
            }
        }
    }

    char_output
}

fn compute_char(luminance: f32) -> char {
    let luminance_index: usize = (luminance * 8.0) as usize;
    return SYMBOLS[luminance_index];
}