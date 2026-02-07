// selection/hit_test.rs - Screen-space vertex hit testing
//
// Pure Rust function for finding the closest vertex to a mouse click.
// Uses projected 2D screen positions (None = behind camera).

use godot::prelude::Vector2;

/// Find the closest projected vertex to the mouse position within a pixel threshold.
///
/// Returns the index of the closest vertex, or None if no vertex is within range.
/// Vertices behind the camera should be passed as `None` in `screen_positions`.
pub fn find_closest_vertex(
    screen_positions: &[Option<Vector2>],
    mouse_pos: Vector2,
    threshold: f32,
) -> Option<usize> {
    let mut best_index = None;
    let mut best_dist_sq = threshold * threshold;

    for (i, pos) in screen_positions.iter().enumerate() {
        if let Some(p) = pos {
            let dist_sq = (p.x - mouse_pos.x).powi(2) + (p.y - mouse_pos.y).powi(2);
            if dist_sq < best_dist_sq {
                best_dist_sq = dist_sq;
                best_index = Some(i);
            }
        }
    }

    best_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_vertex_found() {
        let positions = vec![
            Some(Vector2::new(100.0, 100.0)),
            Some(Vector2::new(200.0, 200.0)),
            Some(Vector2::new(300.0, 300.0)),
        ];
        let mouse = Vector2::new(105.0, 105.0);

        let result = find_closest_vertex(&positions, mouse, 15.0);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_find_closest_vertex_none_in_range() {
        let positions = vec![
            Some(Vector2::new(100.0, 100.0)),
            Some(Vector2::new(200.0, 200.0)),
        ];
        let mouse = Vector2::new(500.0, 500.0);

        let result = find_closest_vertex(&positions, mouse, 15.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_closest_vertex_closest_wins() {
        let positions = vec![
            Some(Vector2::new(100.0, 100.0)), // distance ~14.14
            Some(Vector2::new(105.0, 105.0)), // distance ~7.07 (closer)
            Some(Vector2::new(200.0, 200.0)),
        ];
        let mouse = Vector2::new(110.0, 110.0);

        let result = find_closest_vertex(&positions, mouse, 15.0);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_find_closest_vertex_behind_camera_skipped() {
        let positions = vec![
            None, // behind camera
            Some(Vector2::new(100.0, 100.0)),
            None, // behind camera
        ];
        let mouse = Vector2::new(105.0, 105.0);

        let result = find_closest_vertex(&positions, mouse, 15.0);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_find_closest_vertex_empty_list() {
        let positions: Vec<Option<Vector2>> = vec![];
        let mouse = Vector2::new(100.0, 100.0);

        let result = find_closest_vertex(&positions, mouse, 15.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_closest_vertex_all_behind_camera() {
        let positions = vec![None, None, None];
        let mouse = Vector2::new(100.0, 100.0);

        let result = find_closest_vertex(&positions, mouse, 15.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_closest_vertex_exact_threshold_boundary() {
        // Vertex exactly at threshold distance should NOT be selected (strict < comparison)
        let positions = vec![Some(Vector2::new(115.0, 100.0))];
        let mouse = Vector2::new(100.0, 100.0);

        let result = find_closest_vertex(&positions, mouse, 15.0);
        assert_eq!(result, None);
    }
}
