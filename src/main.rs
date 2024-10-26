use dialoguer::Select;
use std::collections::HashMap;
mod constants;
mod handler_task;
mod math_mods;
mod small_logic;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(short, long, value_enum)]
    func: Option<Func>,
    #[arg(short = 'd', long)]
    depth: Option<f64>,
    number_array: Vec<String>,
    // #[arg(long)]
    // depth: String,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Func {
    ArthMean,
    GeoMean,
    DegMean,
    ArthGeoMean,
    ModifArthGeoMean,
    KolmogorMean,
    TrimMean,
    VinzorMean,
    Median,
    Moda,
    MeanDeviation,
    MeanSquareDev,
    LinearCoeffDeviation,
    SquareCoeffDeviation,
    Dispersion,
}

fn main() {
    let cli = Cli::parse();
    small_logic::clear_terminal();
    let items = vec![
        "Среднее арифметическое",
        "Среднее геометрическое",
        "Среднее степенное",
        "Среднее арефметико-геометрическое",
        "Модифицированное арифметико-геометрическое среднее",
        "Среднее по Колмогорову",
        "Среднее усеченное",
        "Винсоризованное среднее",
        "Медиана",
        "Мода",
        "Среднее линейное отклонение",
        "Cреднее квадратическое отклонение",
        "Линейный коэффициент вариации",
        "Квадратический коэффициент вариации",
        "Дисперсия",
    ];

    let mut action: HashMap<&str, &str> = HashMap::new();
    action.insert("Среднее арифметическое", "arth_mean");
    action.insert("Среднее геометрическое", "geo_mean");
    action.insert("Среднее степенное", "deg_mean");
    action.insert("Среднее арефметико-геометрическое", "arth_geo_mean");
    action.insert(
        "Модифицированное арифметико-геометрическое среднее",
        "modif_arth_geo_mean",
    );
    action.insert("Среднее по Колмогорову", "kolmogor_mean");
    action.insert("Среднее усеченное", "trim_mean");
    action.insert("Винсоризованное среднее", "vinzor_mean");
    action.insert("Медиана", "median");
    action.insert("Мода", "moda");
    action.insert("Среднее линейное отклонение", "mean_deviation");
    action.insert("Cреднее квадратическое отклонение", "mean_square_dev");
    action.insert("Линейный коэффициент вариации", "linear_coeff_deviation");
    action.insert(
        "Квадратический коэффициент вариации",
        "square_coeff_deviation",
    );
    action.insert("Дисперсия", "dispersion");

    println!("{}", constants::PROG_NAME);

    if cli.func.is_none() {
        let selection = Select::new()
            .with_prompt("Что будем делать?")
            .default(0)
            .items(&items)
            .interact()
            .unwrap();

        let func_code = action.get(&items[selection]).unwrap();
        let res = handler_task::get_command(func_code, vec![], None);
        if res == 1 {
            main();
        }
    } else {
        let func_code = match cli.func.unwrap() {
            Func::ArthGeoMean => "arth_geo_mean",
            Func::ArthMean => "arth_mean",
            Func::DegMean => "deg_mean",
            Func::Dispersion => "dispersion",
            Func::KolmogorMean => "kolmogor_mean",
            Func::LinearCoeffDeviation => "linear_coeff_deviation",
            Func::MeanDeviation => "mean_deviation",
            Func::MeanSquareDev => "mean_square_dev",
            Func::Median => "median",
            Func::Moda => "moda",
            Func::ModifArthGeoMean => "modif_arth_geo_mean",
            Func::SquareCoeffDeviation => "square_coeff_deviation",
            Func::TrimMean => "trim_mean",
            Func::VinzorMean => "vinzor_mean",
            _ => "none",
        };
        let mut num_arr: Vec<i128> = Vec::new();
        if !cli.number_array.is_empty() {
            for i in cli.number_array.iter() {
                num_arr.push(i.trim().parse::<i128>().expect("Не число"));
            }
        }
        let res = handler_task::get_command(&func_code, num_arr, cli.depth);
        if res == 1 {
            main();
        }
    }
}
