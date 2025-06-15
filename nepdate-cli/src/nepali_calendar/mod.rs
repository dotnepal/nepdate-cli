/// Return Human friendly month names from index
pub fn human_month(m: i32) -> &'static str {
    match m {
        1 => "Baisakh",
        2 => "Jestha",
        3 => "Ashad",
        4 => "Shrawan",
        5 => "Bhadra",
        6 => "Ashoj",
        7 => "Kartik",
        8 => "Mangshir",
        9 => "Poush",
        10 => "Magh",
        11 => "Falgun",
        12 => "Chaitra",
        _ => "",
    }
}
