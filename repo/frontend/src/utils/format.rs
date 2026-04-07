pub fn format_datetime(iso: &str) -> String {
    // Format ISO datetime to human-readable
    if iso.len() >= 16 {
        let date = &iso[..10];
        let time = &iso[11..16];
        format!("{} {}", date, format_time_12h(time))
    } else {
        iso.to_string()
    }
}

pub fn format_time_12h(time_24h: &str) -> String {
    let parts: Vec<&str> = time_24h.split(':').collect();
    if parts.len() >= 2 {
        let hour: u32 = parts[0].parse().unwrap_or(0);
        let min = parts[1];
        let (h, period) = if hour == 0 { (12, "AM") } else if hour < 12 { (hour, "AM") } else if hour == 12 { (12, "PM") } else { (hour - 12, "PM") };
        format!("{}:{} {}", h, min, period)
    } else {
        time_24h.to_string()
    }
}

pub fn format_mileage(miles: i64) -> String {
    // Simple thousands separator
    let s = miles.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { result.push(','); }
        result.push(c);
    }
    result.chars().rev().collect::<String>() + " mi"
}
