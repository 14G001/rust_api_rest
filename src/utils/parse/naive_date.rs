use {serde::Deserialize,
    std::num::NonZeroI32};

/// Estructura que permite copiar
/// datos de Json con formato
/// NaiveDate utilizando Serde.
#[derive(Deserialize, Debug, Clone)]
pub struct CustomNaiveDate {
    pub yof: NonZeroI32
}