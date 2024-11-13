pub mod arifmetic_geometric_mean;
pub mod arifmetic_mean;
pub mod coefficient_variation;
pub mod dispertion;
pub mod geometric_mean;
pub mod kolmogorov_smirnov;
pub mod linear_coefficient_variation;
pub mod linear_deviation;
pub mod median;
pub mod mode;
pub mod modified_arifmetic_geometric_mean;
pub mod power_mean;
pub mod standart_deviation;
pub mod trimmed_mean;
pub mod winsorized_mean;

pub mod calculations {
    pub use super::arifmetic_geometric_mean::arifmetic_geometric_mean;
    pub use super::arifmetic_mean::arifmetic_mean;
    pub use super::coefficient_variation::coefficient_variations;
    pub use super::dispertion::dispertion;
    pub use super::geometric_mean::geometric_mean;
    pub use super::kolmogorov_smirnov::kolmogorov_smirnov;
    pub use super::linear_coefficient_variation::linear_coefficient_variations;
    pub use super::linear_deviation::linear_deviation;
    pub use super::median::median;
    pub use super::mode::mode;
    pub use super::modified_arifmetic_geometric_mean::modified_arifmetic_geometric_mean;
    pub use super::power_mean::power_mean;
    pub use super::trimmed_mean::trimmed_mean;
    pub use super::winsorized_mean::winsorized_mean;
}
