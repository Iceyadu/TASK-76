pub fn mask_vin(vin: &str) -> String {
    if vin.len() <= 4 {
        return "*".repeat(vin.len());
    }
    let visible = &vin[vin.len() - 4..];
    format!("{}{}", "*".repeat(vin.len() - 4), visible)
}

pub fn mask_license_plate(plate: &str) -> String {
    if plate.len() <= 2 {
        return "*".repeat(plate.len());
    }
    let visible = &plate[plate.len() - 2..];
    format!("{}{}", "*".repeat(plate.len() - 2), visible)
}

pub fn mask_username(username: &str) -> String {
    if username.is_empty() {
        return String::new();
    }
    let first = &username[..1];
    format!("{}***", first)
}

pub fn mask_email(email: &str) -> String {
    if email.is_empty() {
        return String::new();
    }
    "****@****".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_vin() {
        assert_eq!(mask_vin("1HGCM82633A123456"), "*************3456");
        // Length <= 4: mask entire value
        assert_eq!(mask_vin("ABCD"), "****");
        assert_eq!(mask_vin("AB"), "**");
    }

    #[test]
    fn test_mask_license_plate() {
        assert_eq!(mask_license_plate("ABC1234"), "*****34");
        // Length <= 2: mask entire value
        assert_eq!(mask_license_plate("AB"), "**");
    }

    #[test]
    fn test_mask_username() {
        assert_eq!(mask_username("johndoe"), "j***");
        assert_eq!(mask_username("a"), "a***");
        assert_eq!(mask_username(""), "");
    }

    #[test]
    fn test_mask_email() {
        assert_eq!(mask_email("john@example.com"), "****@****");
        assert_eq!(mask_email(""), "");
    }
}
