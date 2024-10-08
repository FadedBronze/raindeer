use cgmath::{Vector2, Zero};

use crate::path_builder::RDStroke;

fn intersect_lines(a: Vector2<f32>, b: Vector2<f32>, c: Vector2<f32>, d: Vector2<f32>) -> Option<Vector2<f32>> {
    let slope_ab = (b.y - a.y) / (b.x - a.x);
    let slope_cd = (d.y - c.y) / (d.x - c.x);

    if slope_ab == slope_cd {
        return None;
    }
    
    let intercept_ab = a.y - slope_ab * a.x;
    let intercept_cd = c.y - slope_cd * c.x;    
    
    if slope_ab == f32::INFINITY {
        let y = slope_cd * a.x + intercept_cd;
        return Some(Vector2::new(a.x, y));
    }
    
    if slope_cd == f32::INFINITY {
        let y = slope_ab * c.x + intercept_ab;
        return Some(Vector2::new(c.x, y));
    }

    let x = (intercept_cd - intercept_ab) / (slope_ab - slope_cd);
    let y = slope_ab * x + intercept_ab;

    Some(Vector2::new(x, y))
}

pub(crate) fn triangulate_stroke(vertices: &[Vector2<f32>], stroke: &RDStroke) -> (Vec<Vector2<f32>>, Vec<u32>) {
    let mut vertices = vec![];
    let mut indicies = vec![];


    (vertices, indicies)
}

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
    fn test_intersect_lines() {
        assert_eq!(intersect_lines(Vector2::new(5.0, 5.0), Vector2::new(-5.0, 5.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 10.0)), Some(Vector2::new(0.0, 5.0)));
        assert_eq!(intersect_lines(Vector2::new(0.0, 0.0), Vector2::new(5.0, 5.0), Vector2::new(10.0, 0.0), Vector2::new(10.0, 20.0)), Some(Vector2::new(10.0, 10.0)));
        assert_eq!(intersect_lines(Vector2::new(0.0, 0.0), Vector2::new(2.5, 2.5), Vector2::new(10.0, 0.0), Vector2::new(7.5, 2.5)), Some(Vector2::new(5.0, 5.0)));
        assert_eq!(intersect_lines(Vector2::new(0.0, 0.0), Vector2::new(5.0, 5.0), Vector2::new(10.0, 0.0), Vector2::new(5.0, 5.0)), Some(Vector2::new(5.0, 5.0)));
        assert_eq!(intersect_lines(Vector2::new(0.0, 0.0), Vector2::new(0.0, 5.0), Vector2::new(10.0, 0.0), Vector2::new(10.0, 5.0)), None);
    }

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
