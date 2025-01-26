use spark_std::align::{Alignment, space::CrystalAxis};

#[test]
fn test_alignment_bytes() {
    assert_eq!(Alignment::Crystal16.as_bytes(), 16);
    assert_eq!(Alignment::Vector32.as_bytes(), 32);
    assert_eq!(Alignment::Vector64.as_bytes(), 64);
    assert_eq!(Alignment::Parallel128.as_bytes(), 128);
    assert_eq!(Alignment::Parallel256.as_bytes(), 256);
    assert_eq!(Alignment::Custom(512).as_bytes(), 512);
}

#[test]
fn test_alignment_from_bytes() {
    assert_eq!(Alignment::from_bytes(16), Alignment::Crystal16);
    assert_eq!(Alignment::from_bytes(32), Alignment::Vector32);
    assert_eq!(Alignment::from_bytes(64), Alignment::Vector64);
    assert_eq!(Alignment::from_bytes(128), Alignment::Parallel128);
    assert_eq!(Alignment::from_bytes(256), Alignment::Parallel256);
    assert_eq!(Alignment::from_bytes(512), Alignment::Custom(512));
}

#[test]
fn test_crystal_axis() {
    let sizes = [16, 32, 64, 128, 256];
    let axes: Vec<CrystalAxis> = sizes.iter()
        .map(|size| CrystalAxis::new(*size, Alignment::Parallel256))
        .collect();

    for (axis, &size) in axes.iter().zip(sizes.iter()) {
        assert_eq!(axis.size(), size);
        assert_eq!(axis.alignment(), Alignment::Parallel256);
    }
}
