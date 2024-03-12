pub trait AmountUpdater {
    fn update_amount(amount: &mut f64, difference: f64);
}
