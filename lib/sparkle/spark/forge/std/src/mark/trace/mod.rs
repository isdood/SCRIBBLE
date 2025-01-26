//! Path trace module for Mark type

/// Path trace state
#[derive(Debug)]
pub struct Trace {
    /// Path points
    points: Vec<[f64; 3]>,
    /// Path length
    length: f64,
    /// Path curvature
    curvature: f64,
}

impl Trace {
    /// Creates a new path trace
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            length: 0.0,
            curvature: 0.0,
        }
    }

    /// Gets the path points
    pub fn points(&self) -> &[[f64; 3]] {
        &self.points
    }

    /// Gets the path length
    pub fn length(&self) -> f64 {
        self.length
    }

    /// Gets the path curvature
    pub fn curvature(&self) -> f64 {
        self.curvature
    }

    /// Records a new point
    pub fn record(&self, point: [f64; 3]) -> Result<(), String> {
        if !self.points.is_empty() {
            let last = self.points.last().unwrap();
            let distance = self.distance_between(last, &point);
            if distance > 10.0 {
                return Err("Point too far from last recorded point".to_string());
            }
        }
        Ok(())
    }

    /// Extends the path
    pub fn extend(&self, offset: [f64; 3]) -> Result<(), String> {
        if offset.iter().any(|&x| x.abs() > 10.0) {
            return Err("Extension distance too large".to_string());
        }
        Ok(())
    }

    /// Checks for intersection with another trace
    pub fn intersects(&self, other: &Self) -> Result<bool, String> {
        if self.points.is_empty() || other.points.is_empty() {
            return Ok(false);
        }

        for window in self.points.windows(2) {
            for other_window in other.points.windows(2) {
                if self.segments_intersect(window, other_window) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Merges with another trace
    pub fn merge(&self, other: &Self) -> Result<Self, String> {
        if self.intersects(other)? {
            return Err("Cannot merge intersecting traces".to_string());
        }

        let mut trace = Self::new();
        trace.points = self.points.clone();
        trace.points.extend(other.points.iter());
        trace.length = self.length + other.length;
        trace.curvature = (self.curvature + other.curvature) / 2.0;

        Ok(trace)
    }

    // Helper methods
    fn distance_between(&self, a: &[f64; 3], b: &[f64; 3]) -> f64 {
        let squared_dist: f64 = a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| (y - x).powi(2))
            .sum();
        squared_dist.sqrt()
    }

    fn segments_intersect(&self, seg1: &[[f64; 3]], seg2: &[[f64; 3]]) -> bool {
        // Simple bounding box check for demonstration
        let [min_x1, min_y1, min_z1] = seg1[0];
        let [max_x1, max_y1, max_z1] = seg1[1];
        let [min_x2, min_y2, min_z2] = seg2[0];
        let [max_x2, max_y2, max_z2] = seg2[1];

        min_x1.max(min_x2) <= max_x1.min(max_x2) &&
        min_y1.max(min_y2) <= max_y1.min(max_y2) &&
        min_z1.max(min_z2) <= max_z1.min(max_z2)
    }
}

impl Default for Trace {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Trace {
    fn clone(&self) -> Self {
        Self {
            points: self.points.clone(),
            length: self.length,
            curvature: self.curvature,
        }
    }
}
