use spark_std::array::{CrystalArray, ArrayOps};
use spark_std::align::Alignment;

#[test]
fn test_array_creation() {
    let array: CrystalArray<i32> = CrystalArray::new(Alignment::Crystal16);
    assert!(array.is_empty());
    assert_eq!(array.len(), 0);
}

#[test]
fn test_array_push_pop() {
    let mut array = CrystalArray::new(Alignment::Crystal16);

    array.push(1);
    array.push(2);
    array.push(3);

    assert_eq!(array.len(), 3);
    assert_eq!(array.pop(), Some(3));
    assert_eq!(array.pop(), Some(2));
    assert_eq!(array.pop(), Some(1));
    assert_eq!(array.pop(), None);
}

#[test]
fn test_array_get() {
    let mut array = CrystalArray::new(Alignment::Crystal16);

    array.push(1);
    array.push(2);

    assert_eq!(array.get(0), Some(&1));
    assert_eq!(array.get(1), Some(&2));
    assert_eq!(array.get(2), None);
}

#[test]
fn test_array_alignment() {
    let array: CrystalArray<f32> = CrystalArray::new(Alignment::Vector32);
    assert!(array.is_simd_aligned());
    assert_eq!(array.alignment(), Alignment::Vector32);
}

#[test]
fn test_array_operations() {
    let mut a = CrystalArray::new(Alignment::Crystal16);
    let mut b = CrystalArray::new(Alignment::Crystal16);

    for i in 0..4 {
        a.push(i as f32);
        b.push((i * 2) as f32);
    }

    let c = a.add(&b);
    let d = a.mul(&b);
    let dot = a.dot(&b);

    for i in 0..4 {
        assert_eq!(c.get(i), Some(&(i as f32 + (i * 2) as f32)));
        assert_eq!(d.get(i), Some(&(i as f32 * (i * 2) as f32)));
    }

    assert_eq!(dot, 28.0); // 0*0 + 1*2 + 2*4 + 3*6 = 28
}

#[test]
fn test_optimal_alignment() {
    let shard = spark_std::shard::arch::Shard::new();
    let array: CrystalArray<f32> = CrystalArray::new(CrystalArray::<f32>::optimal_alignment());

    match shard.architecture() {
        spark_std::shard::arch::Architecture::X86_64 => {
            if shard.has_feature(spark_std::shard::arch::CpuFeature::AVX512F) {
                assert_eq!(array.alignment(), Alignment::Vector64);
            } else if shard.has_feature(spark_std::shard::arch::CpuFeature::AVX2) {
                assert_eq!(array.alignment(), Alignment::Vector32);
            }
        }
        spark_std::shard::arch::Architecture::AArch64 => {
            if shard.has_feature(spark_std::shard::arch::CpuFeature::SVE) {
                assert_eq!(array.alignment(), Alignment::Vector64);
            } else {
                assert_eq!(array.alignment(), Alignment::Vector16);
            }
        }
        _ => assert_eq!(array.alignment(), Alignment::Crystal16),
    }
}

#[test]
fn test_iterator() {
    let mut array = CrystalArray::new(Alignment::Crystal16);
    for i in 0..5 {
        array.push(i);
    }

    let sum: i32 = array.into_iter().sum();
    assert_eq!(sum, 10); // 0 + 1 + 2 + 3 + 4 = 10
}

#[test]
fn test_from_iterator() {
    let vec = vec![1, 2, 3, 4, 5];
    let array: CrystalArray<i32> = vec.into_iter().collect();

    assert_eq!(array.len(), 5);
    for i in 0..5 {
        assert_eq!(array.get(i), Some(&(i as i32 + 1)));
    }
}
