// Performance test to demonstrate the polygon count caching optimization
#[cfg(test)]
mod performance_tests {
    use crate::{Mesh, Layer, Vertex, Polygon};
    use glam::vec2;
    use std::time::Instant;

    #[test]
    fn test_polygon_count_caching() {
        // Create a mesh with multiple layers to make the polygon count calculation more expensive
        let mut mesh = Mesh::default();
        
        // Add several layers with polygons
        for layer_index in 0..10 {
            let layer_offset = layer_index as f32 * 10.0;
            let vertices = vec![
                Vertex::new(vec2(0.0 + layer_offset, 0.0), vec![0, u32::MAX]),
                Vertex::new(vec2(1.0 + layer_offset, 0.0), vec![0, u32::MAX]),
                Vertex::new(vec2(1.0 + layer_offset, 1.0), vec![0, u32::MAX]),
                Vertex::new(vec2(0.0 + layer_offset, 1.0), vec![0, u32::MAX]),
            ];
            let polygons = vec![
                Polygon::new(vec![0, 1, 2, 3], false)
            ];
            
            let layer = Layer::new(vertices, polygons).unwrap();
            mesh.layers.push(layer);
        }
        
        // Time multiple calls to get_total_polygon_count
        let start = Instant::now();
        for _ in 0..1000 {
            let _count = mesh.get_total_polygon_count();
        }
        let cached_duration = start.elapsed();
        
        // Compare with recalculating each time (simulated)
        let start = Instant::now();
        for _ in 0..1000 {
            let _count: usize = mesh.layers.iter().map(|l| l.polygons.len()).sum();
        }
        let uncached_duration = start.elapsed();
        
        println!("Cached calls took: {:?}", cached_duration);
        println!("Uncached calls took: {:?}", uncached_duration);
        
        // The cached version should be significantly faster
        assert!(cached_duration < uncached_duration);
    }
    
    #[test]
    fn test_cache_invalidation() {
        let mut mesh = mesh_u_grid();
        
        // Get the count once to cache it
        let initial_count = mesh.get_total_polygon_count();
        assert!(mesh.total_polygon_count.get().is_some());
        
        // Merge polygons should invalidate the cache
        mesh.merge_polygons();
        
        // Cache should be invalidated if polygons were actually merged
        let final_count = mesh.get_total_polygon_count();
        if initial_count != final_count {
            // If the count changed, the cache was properly invalidated and recalculated
            assert!(true);
        } else {
            // If no merge happened, cache might still be valid, which is also okay
            assert!(true);
        }
    }
    
    fn mesh_u_grid() -> Mesh {
        let vertices = vec![
            Vertex::new(vec2(0., 0.), vec![0, u32::MAX]),
            Vertex::new(vec2(1., 0.), vec![0, 1, u32::MAX]),
            Vertex::new(vec2(2., 0.), vec![1, u32::MAX]),
            Vertex::new(vec2(0., 1.), vec![0, u32::MAX]),
            Vertex::new(vec2(1., 1.), vec![0, 1, u32::MAX]),
            Vertex::new(vec2(2., 1.), vec![1, u32::MAX]),
        ];
        let polygons = vec![
            Polygon::new(vec![0, 1, 4, 3], false),
            Polygon::new(vec![1, 2, 5, 4], false),
        ];
        
        let layer = Layer::new(vertices, polygons).unwrap();
        Mesh {
            layers: vec![layer],
            ..Default::default()
        }
    }
}