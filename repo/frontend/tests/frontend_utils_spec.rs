#[path = "../src/utils/format.rs"]
mod format;
#[path = "../src/utils/time.rs"]
mod time;

#[test]
fn frontend_utils_time_slot_generation_is_deterministic() {
    let slots = time::generate_time_slots(8, 10, 15);
    assert_eq!(
        slots,
        vec!["08:00", "08:15", "08:30", "08:45", "09:00", "09:15", "09:30", "09:45"]
    );
}

#[test]
fn frontend_utils_datetime_and_mileage_formatting_are_user_friendly() {
    assert_eq!(format::format_datetime("2026-01-01T13:05:00"), "2026-01-01 1:05 PM");
    assert_eq!(format::format_mileage(123456), "123,456 mi");
}
