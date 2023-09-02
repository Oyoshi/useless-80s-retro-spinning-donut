use ansi_term::Colour::Fixed;

/********************** Constants **********************/

const DELTA_THETA: f32 = 0.01;
const DELTA_PHI: f32 = 0.01;

const R1: f32 = 1.0;
const R2: f32 = 2.0;
const K1: f32 = (SCREEN_WIDTH as f32) * K2 * 3.0 / (8.0 * (R1 + R2));
const K2: f32 = 5.0;

const SCREEN_WIDTH: usize = 36;
const SCREEN_HEIGHT: usize = 36;

const SYMBOLS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];
const NIGHT_COLORS: [u8; 12] = [26, 27, 62, 63, 98, 99, 134, 135, 170, 171, 206, 207];
const DAY_COLORS: [u8; 12] = [198, 199, 204, 205, 210, 211, 216, 217, 222, 223, 228, 229];

const A0: f32 = 1.0;
const DELTA_A: f32 = 0.04;
const B0: f32 = 1.0;
const DELTA_B: f32 = 0.015;

/*******************************************************/

struct Frame {
    symbols_output: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT],
    colors_output: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

struct FrameElement {
    symbol: char,
    color: u8,
}

impl FrameElement {
    fn new(color_variant: &ColorVariant, luminance_index: usize) -> FrameElement {
        let symbol = SYMBOLS[luminance_index];
        let color = match color_variant {
            ColorVariant::DAY => DAY_COLORS[luminance_index],
            ColorVariant::NIGHT => NIGHT_COLORS[luminance_index],
        };
        return FrameElement { symbol, color };
    }
}

pub enum ColorVariant {
    DAY,
    NIGHT,
}

pub struct Config {
    pub variant: ColorVariant,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // program name so skip it

        let variant = match args.next() {
            Some(arg) => match arg.as_str() {
                "day" => ColorVariant::DAY,
                "night" => ColorVariant::NIGHT,
                _ => return Err("Invalid variant"),
            },
            None => return Err("Color variant is required"),
        };

        Ok(Config { variant })
    }
}

pub fn simulate(config: &Config) {
    let mut a = A0;
    let mut b = B0;
    print!("\x1b[2J");
    loop {
        render_frame(&config.variant, a, b);
        a += DELTA_A;
        b += DELTA_B;
    }
}

fn render_frame(color_variant: &ColorVariant, a: f32, b: f32) {
    let Frame {
        symbols_output,
        colors_output,
    } = compute_frame(&color_variant, a, b);
    print!("\x1b[H");
    for i in 0..SCREEN_HEIGHT {
        for j in 0..SCREEN_WIDTH {
            let symbol = Fixed(colors_output[i][j])
                .on(Fixed(16))
                .bold()
                .paint(symbols_output[i][j].to_string());
            print!("{}", symbol);
        }
        println!();
    }
}

fn compute_frame(color_variant: &ColorVariant, a: f32, b: f32) -> Frame {
    let mut symbols_output = [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut colors_output = [[0u8; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut z_buffer = [[0.0f32; SCREEN_WIDTH]; SCREEN_HEIGHT];

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
                let FrameElement { symbol, color } =
                    compute_frame_element(&color_variant, luminance);
                symbols_output[xp][yp] = symbol;
                colors_output[xp][yp] = color;
            }
        }
    }

    Frame {
        symbols_output,
        colors_output,
    }
}

fn compute_frame_element(color_variant: &ColorVariant, luminance: f32) -> FrameElement {
    let luminance_index = (luminance * 8.0) as usize;
    return FrameElement::new(&color_variant, luminance_index);
}
