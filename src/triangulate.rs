use cgmath::Vector2;

//1. no colinear edges
//2. no clockwise verticies
pub(crate) fn triangulate(vertices: &[Vector2<f32>]) -> Vec<u32> {
    let mut vertex_ids = vec![];
    let mut indicies = vec![];

    for i in 0..vertices.len() {
        vertex_ids.push(i);
    }

    let mut id: usize = 0;

    'ear_clipper: while vertex_ids.len() > 2 {
        let last_id = if id == 0 { vertex_ids.len() - 1 } else { id - 1 };
        let next_id = if id + 1 == vertex_ids.len() { 0 } else { id + 1 };

        let last = vertices[vertex_ids[last_id]];
        let now = vertices[vertex_ids[id]];
        let next = vertices[vertex_ids[next_id]];

        if (last - now).perp_dot(next - now) > 0.0 {
            id += 1;
            id %= vertex_ids.len();
            continue 'ear_clipper;
        }

        for i in 0..vertex_ids.len() {
            if within_triangle(last, now, next, vertices[vertex_ids[i]]) {
                id += 1;
                id %= vertex_ids.len();
                continue 'ear_clipper;
            }
        }
         
        indicies.push(vertex_ids[last_id] as u32);
        indicies.push(vertex_ids[id] as u32);
        indicies.push(vertex_ids[next_id] as u32);
        vertex_ids.remove(id);
    }

    indicies
}

pub(crate) fn within_triangle(a: Vector2<f32>, b: Vector2<f32>, c: Vector2<f32>, p: Vector2<f32>) -> bool {
    (p - a).perp_dot(b - a) < 0.0 && (p - b).perp_dot(c - b) < 0.0 && (p - c).perp_dot(a - c) < 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_within_triangle() {
        assert_eq!( within_triangle( Vector2::new(0.0, 0.0), Vector2::new(0.0, 10.0), Vector2::new(-10.0, 10.0), Vector2::new(-5.0, 2.5)), false);
        assert_eq!( within_triangle( Vector2::new(0.0, 0.0), Vector2::new(0.0, 10.0), Vector2::new(-10.0, 10.0), Vector2::new(-2.5, 5.0)), true);
        assert_eq!( within_triangle( Vector2::new(0.0, 0.0), Vector2::new(0.0, 10.0), Vector2::new(-10.0, 10.0), Vector2::new(5.0, 2.5)), false);
        assert_eq!( within_triangle( Vector2::new(0.0, 0.0), Vector2::new(10.0, 0.0), Vector2::new(10.0, 10.0), Vector2::new(5.0, 2.5)), true);
        assert_eq!( within_triangle( Vector2::new(0.0, 0.0), Vector2::new(10.0, 0.0), Vector2::new(10.0, 10.0), Vector2::new(10.0, 0.0)), false);
        assert_eq!( within_triangle( Vector2::new(0.0, 0.0), Vector2::new(10.0, 0.0), Vector2::new(10.0, 10.0), Vector2::new(10.0, 10.0)), false);
        assert_eq!( within_triangle( Vector2::new(0.0, 0.0), Vector2::new(10.0, 0.0), Vector2::new(10.0, 10.0), Vector2::new(0.0, 0.0)), false);
    }
    
    #[test]
    fn test_triangulate() {
        assert_eq!(triangulate(&vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(10.0, 0.0),
            Vector2::new(10.0, 10.0),
            Vector2::new(0.0, 10.0),
        ]), vec![3, 0, 1, 3, 1, 2]);
         
        assert_eq!(triangulate(&vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(10.0, 0.0),
            Vector2::new(10.0, 10.0),
            Vector2::new(5.01, 5.01),
            Vector2::new(0.0, 10.0),
        ]), vec![4, 0, 1,  1, 2, 3,  1, 3, 4]);
    }
}
