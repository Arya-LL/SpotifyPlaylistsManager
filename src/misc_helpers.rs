pub fn convert_to_parsable_date(mut date_str: String) -> i64 {
    let date_vec: Vec<&str> = date_str.split('-').collect();
    //For simplicity, all dates are assumed to be in UTC and missing information is taken as the average of
    //possible values
    match date_vec.len() {
        3 => date_str.push_str("UTC"),
        2 => date_str.push_str("-15 UTC"),
        1 => date_str.push_str("-06-15 UTC"),
        // Set date to 0000-01-01 in case of no date information
        _ => date_str.push_str("-62167201438"),
    };

    dateparser::parse(&date_str)
        .expect("Invalid date format")
        .timestamp()
}

pub fn convert_mode_to_int(mode: rspotify::model::enums::misc::Modality) -> i32 {
    match mode {
        rspotify::model::Modality::Minor => 0,
        rspotify::model::Modality::Major => 1,
        rspotify::model::Modality::NoResult => -1,
    }
}
