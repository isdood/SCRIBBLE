use spark_std::mark::{Mark, MarkResult};

#[test]
fn test_mark_creation() {
    let mark = Mark::new([1, 2, 3]);
    assert!(mark.id() != 0);
    assert!(!mark.data().is_empty());
}

#[test]
fn test_mark_placement() -> MarkResult<()> {
    let mark = Mark::new([1, 2, 3]);
    mark.place([0.0, 0.0, 0.0])?;
    assert!(mark.crystal().coherence() > 0.0);
    Ok(())
}

#[test]
fn test_mark_shifting() -> MarkResult<()> {
    let mark = Mark::new([1, 2, 3]);
    mark.shift([0.5, 0.5, 0.5])?;
    assert!(mark.trace().length() >= 0.0);
    Ok(())
}

#[test]
fn test_mark_collision() -> MarkResult<()> {
    let mark1 = Mark::new([1, 2, 3]);
    let mark2 = Mark::new([4, 5, 6]);

    mark1.place([0.0, 0.0, 0.0])?;
    mark2.place([2.0, 2.0, 2.0])?;

    assert!(!mark1.collides_with(&mark2)?);
    Ok(())
}

#[test]
fn test_mark_merging() -> MarkResult<()> {
    let mark1 = Mark::new([1, 2, 3]);
    let mark2 = Mark::new([4, 5, 6]);

    mark1.place([0.0, 0.0, 0.0])?;
    mark2.place([5.0, 5.0, 5.0])?;

    let merged = mark1.merge(&mark2)?;
    assert_eq!(merged.data().len(), mark1.data().len() + mark2.data().len());
    assert!(merged.crystal().strength() > 0.0);
    assert!(merged.trace().length() >= 0.0);

    Ok(())
}

#[test]
fn test_mark_identity() {
    let mark1 = Mark::new([1, 2, 3]);
    let mark2 = Mark::new([1, 2, 3]);
    let mark3 = Mark::new([4, 5, 6]);

    assert_eq!(mark1, mark2);
    assert_ne!(mark1, mark3);
    assert_ne!(mark2, mark3);
}

#[test]
fn test_mark_trace_extension() -> MarkResult<()> {
    let mark = Mark::new([1, 2, 3]);

    mark.place([0.0, 0.0, 0.0])?;
    mark.shift([1.0, 0.0, 0.0])?;
    mark.shift([0.0, 1.0, 0.0])?;
    mark.shift([0.0, 0.0, 1.0])?;

    assert!(mark.trace().points().len() > 0);
    assert!(mark.trace().length() > 0.0);
    assert!(mark.trace().curvature() >= 0.0);

    Ok(())
}

#[test]
fn test_mark_crystal_field() -> MarkResult<()> {
    let mark = Mark::new([1, 2, 3]);

    mark.place([0.0, 0.0, 0.0])?;
    assert_eq!(mark.crystal().center(), [0.0, 0.0, 0.0]);
    assert!(mark.crystal().radius() > 0.0);
    assert!(mark.crystal().strength() > 0.0);
    assert!(mark.crystal().coherence() > 0.0);

    Ok(())
}

#[test]
fn test_mark_error_handling() {
    let mark = Mark::new([1, 2, 3]);

    // Test out-of-range placement
    assert!(mark.place([100.0, 100.0, 100.0]).is_err());

    // Test excessive shift
    assert!(mark.shift([20.0, 20.0, 20.0]).is_err());

    // Test invalid merge
    let mark2 = Mark::new([4, 5, 6]);
    mark.place([0.0, 0.0, 0.0]).unwrap();
    mark2.place([0.1, 0.1, 0.1]).unwrap();
    assert!(mark.merge(&mark2).is_err());
}
