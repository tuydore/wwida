pub(crate) mod date_specifier;
pub(crate) mod duration;

pub(crate) const DATE_FORMAT: &str = "%a, %0e-%b-%Y";

pub(crate) fn today() -> chrono::NaiveDate {
    #[cfg(not(test))]
    return chrono::Local::today().naive_local();

    #[cfg(test)] // in test configuration, it's always Wed, 05-Jan-2022
    return chrono::NaiveDate::from_isoywd(2022, 1, chrono::Weekday::Wed);
}
