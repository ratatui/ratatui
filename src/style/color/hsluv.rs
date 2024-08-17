use std::{cmp::Ordering, f64::consts::PI};

const M: [[f64; 3]; 3] = [
    [3.240969941904521, -1.537383177570093, -0.498610760293],
    [-0.96924363628087, 1.87596750150772, 0.041555057407175],
    [0.055630079696993, -0.20397695888897, 1.056971514242878],
];

const REF_Y: f64 = 1.0;
const REF_U: f64 = 0.19783000664283;
const REF_V: f64 = 0.46831999493879;
// CIE LUV constants
const KAPPA: f64 = 903.2962962;
const EPSILON: f64 = 0.0088564516;

#[allow(clippy::many_single_char_names)]
pub(super) fn hsluv_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    // Convert HSLuv to LCH
    let (l, c, h) = match l {
        l if l > 99.9999999 => (100.0, 0.0, h),
        l if l < 0.00000001 => (0.0, 0.0, h),
        _ => {
            let mx = max_chroma_for(l, h);
            let c = mx / 100.0 * s;
            (l, c, h)
        }
    };

    // Convert LCH to LUV
    let h_radians = h * PI / 180.0;
    let u = h_radians.cos() * c;
    let v = h_radians.sin() * c;

    // Convert LUV to XYZ
    if l == 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let var_y = if l > 8.0 {
        REF_Y * ((l + 16.0) / 116.0).powf(3.0)
    } else {
        REF_Y * l / KAPPA
    };
    let var_u = u / (13.0 * l) + REF_U;
    let var_v = v / (13.0 * l) + REF_V;

    let y = var_y * REF_Y;
    let x = 0.0 - (9.0 * y * var_u) / ((var_u - 4.0) * var_v - var_u * var_v);
    let z = (9.0 * y - (15.0 * var_v * y) - (var_v * x)) / (3.0 * var_v);

    // Convert XYZ to RGB
    let rgb: Vec<f64> = M
        .iter()
        .map(|i| {
            let dot_product: f64 = i.iter().zip([x, y, z].iter()).map(|(i, j)| i * j).sum();
            if dot_product <= 0.0031308 {
                12.92 * dot_product
            } else {
                1.055 * (dot_product.powf(1.0 / 2.4)) - 0.055
            }
        })
        .collect();

    (rgb[0], rgb[1], rgb[2])
}

fn max_chroma_for(l: f64, h: f64) -> f64 {
    let hrad = h / 360.0 * PI * 2.0;

    let mut lengths: Vec<f64> = get_bounds(l)
        .iter()
        .map(|line| length_of_ray_until_intersect(hrad, line))
        .filter(|length| length > &0.0)
        .collect::<Vec<f64>>();

    lengths.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    lengths[0]
}

fn get_bounds(l: f64) -> Vec<(f64, f64)> {
    let sub1 = ((l + 16.0).powi(3)) / 1560896.0;
    let sub2 = match sub1 {
        s if s > EPSILON => s,
        _ => l / KAPPA,
    };

    let mut bounds = Vec::new();

    for ms in &M {
        let (m1, m2, m3) = (ms[0], ms[1], ms[2]);
        for t in 0..2 {
            let top1 = (284517.0 * m1 - 94839.0 * m3) * sub2;
            let top2 = (838422.0 * m3 + 769860.0 * m2 + 731718.0 * m1) * l * sub2
                - 769860.0 * f64::from(t) * l;
            let bottom = (632260.0 * m3 - 126452.0 * m2) * sub2 + 126452.0 * f64::from(t);

            bounds.push((top1 / bottom, top2 / bottom));
        }
    }
    bounds
}

fn length_of_ray_until_intersect(theta: f64, line: &(f64, f64)) -> f64 {
    let (m1, b1) = *line;
    let length = b1 / (theta.sin() - m1 * theta.cos());
    if length < 0.0 {
        -0.0001
    } else {
        length
    }
}
