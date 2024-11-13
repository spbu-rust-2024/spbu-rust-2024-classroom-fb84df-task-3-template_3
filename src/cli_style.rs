use crate::modules::{calculations::*, standart_deviation::standart_deviation};

fn count_decimal_places(epsilon: f64) -> usize {
    let epsilon_str = format!("{:.10}", epsilon);
    if let Some(pos) = epsilon_str.find('.') {
        epsilon_str[pos + 1..]
            .chars()
            .take_while(|&c| c == '0')
            .count()
            + 1
    } else {
        0
    }
}

use clap::Parser;
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long = "a", help = "Calculates arifmetic mean\n")]
    pub a: bool,
    #[arg(long = "g", help = "Calculates geometric mean\n")]
    pub g: bool,
    #[arg(long = "p", help = "Calculates power mean\n")]
    pub p: bool,
    #[arg(
        long = "ag",
        help = "Calculates arifmetic-geometric mean. \nRequired parameter: 0.0 < accuracy < 1.0\n"
    )]
    pub ag: Option<f64>,
    #[arg(long = "median", help = "Calculates median of array\n")]
    pub median: bool,
    #[arg(long = "mode", help = "Calculates mode of array\n")]
    pub mode: bool,
    #[arg(long = "cv", help = "Calculates coefficient variation\n")]
    pub cv: bool,
    #[arg(long = "lcv", help = "Calculates linear coefficient variation\n")]
    pub lcv: bool,
    #[arg(long = "dsp", help = "Calculates dispertion\n")]
    pub dsp: bool,
    #[arg(
        long = "klg",
        help = "Calculates Kolmogorov-Smirnov mean.\nRequired parameter: 0.0 < accuracy < 1.0\n"
    )]
    pub klg: Option<f64>,
    #[arg(long = "dv", help = "Calculates deviation\n")]
    pub dv: bool,
    #[arg(long = "ldv", help = "Calculates linear deviation\n")]
    pub ldv: bool,
    #[arg(
        long = "mag",
        help = "Calculates modified arifmetic-geometric mean. \nRequired parameter: 0.0 < accuracy < 1.0\n"
    )]
    pub mag: Option<f64>,
    #[arg(
        long = "wn",
        help = "Calculates winsorized mean.  \nRequired parameter: percent of elements to replace (0-100)\n"
    )]
    pub wn: Option<u8>,
    #[arg(
        long = "tr",
        help = "Calculates trimmed mean. \nRequired parameter: percent of elements to trim (0-100)\n"
    )]
    pub tr: Option<u8>,
}

pub fn print_info(vec: Vec<f64>, args: &Args) -> Vec<f64> {
    let mut info: Vec<f64> = Vec::new();

    if args.a {
        let am = arifmetic_mean(&vec);
        println!("Arifmetic mean: {:.3}\n", am);
        info.push(am);
    }
    if args.g {
        let g = geometric_mean(&vec);
        println!("Geometric mean: {:.3}\n", g);
        info.push(g);
    }
    if args.p {
        let p = power_mean(&vec, &5);
        println!("Powered: {:.3}\n", p);
        info.push(p);
    }
    if let Some(ag_val) = args.ag {
        if ag_val > 1.0 || ag_val < 0.0 {
            panic!("Incorrect arifmetic-geometric argument. Check --help!");
        }
        println!(
            "A{}: {:.3}, B{}: {:.3}\n",
            ag_val,
            arifmetic_geometric_mean(&vec, &ag_val)[0],
            ag_val,
            arifmetic_geometric_mean(&vec, &ag_val)[1]
        );
        info.push(arifmetic_geometric_mean(&vec, &ag_val)[0]);
    }

    if args.median {
        let median = median(&vec);
        println!("Median: {:.3}\n", median);
        info.push(median);
    }
    if args.mode {
        let mode = mode(&vec);
        println!("Mode: {:.3}\n", mode);
        info.push(mode);
    }
    if args.cv {
        let cv = coefficient_variations(&vec);
        println!("Cv: {:.3}\n", cv);
        info.push(cv);
    }
    if args.lcv {
        let lcv = linear_coefficient_variations(&vec);
        println!("Linear Cv: {:.3}\n", lcv);
        info.push(lcv);
    }
    if args.dsp {
        let dsp = dispertion(&vec);
        println!("Dispertion: {:.3}\n", dsp);
        info.push(dsp);
    }
    if let Some(ag_val) = args.klg {
        if ag_val > 1.0 || ag_val < 0.0 {
            panic!("Incorrect Kalmogorov-Smirnov argument. Check --help!");
        }
        let ks = kolmogorov_smirnov(&vec, ag_val);
        if ks.is_infinite() || ks.is_nan() {
            println!("Kolmogorov mean with eps: Can't calculate\n");
        } else {
            let precition = count_decimal_places(ag_val);
            println!("Kolmogorov mean with eps: {:.1$}\n", ks, precition);
        }
        info.push(ks);
    }
    if args.dv {
        let dv = standart_deviation(&vec);
        println!("Deviation: {:.3}\n", dv);
        info.push(dv);
    }
    if args.ldv {
        let ldv = linear_deviation(&vec);
        println!("Deviation: {:.3}\n", ldv);
        info.push(ldv);
    }
    if let Some(ag_val) = args.mag {
        if ag_val > 1.0 || ag_val < 0.0 {
            panic!("Incorrect modified arifmetic-geometric argument. Check --help!");
        }
        let precision = count_decimal_places(ag_val);
        let tmp_tuple = modified_arifmetic_geometric_mean(&vec, &ag_val);
        println!(
            "Modified arifmetic-geometric mean: A(eps) = {:.pr$}, B(eps) = {:.pr$}, Z(eps) = {:.pr$}\n",
            tmp_tuple[0], tmp_tuple[1], tmp_tuple[2], pr = precision
        );
        info.push(tmp_tuple[0]);
    }
    if let Some(ag_val) = args.wn {
        let wn = winsorized_mean(&vec, ag_val);
        if ag_val > 100 {
            panic!("Incorrect winsorized argument. Check --help!");
        }
        println!("Winsorized mean: {:.3}\n", wn);
        info.push(wn);
    }
    if let Some(ag_val) = args.tr {
        let tr = trimmed_mean(&vec, ag_val);
        if ag_val > 100 {
            panic!("Incorrect trimmed argument. Check --help!");
        }
        println!("Trimmed mean: {:.3}\n", tr);
        info.push(tr);
    }
    info
}
