use fleetreserve_backend::security::masking::{mask_license_plate, mask_vin};

#[test]
fn api_vehicle_masking_vin() {
    assert_eq!(mask_vin("1HGCM82633A123456"), "*************3456");
}

#[test]
fn api_vehicle_masking_plate() {
    assert_eq!(mask_license_plate("ABC1234"), "*****34");
}
