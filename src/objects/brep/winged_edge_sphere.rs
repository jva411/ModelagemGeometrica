use std::f32::consts::PI;
use glam::Vec3;
use crate::objects::brep::winged_edge_object::{WingedEdgeObject, Vertex, Edge, Face};

impl WingedEdgeObject {
  pub fn new_sphere(name: String, subdivisions: usize) -> Self {
    let mut object = WingedEdgeObject::new(name);

    let n_long = subdivisions;
    let n_lat = subdivisions;

    let top_vertex = Vertex { id: 0, position: Vec3::new(0.0, 1.0, 0.0), _edge: 0 };
    let bottom_vertex = Vertex { id: 1, position: Vec3::new(0.0, -1.0, 0.0), _edge: 0 };
    object.vertices.push(top_vertex);
    object.vertices.push(bottom_vertex);

    let rings_count = n_lat - 1;
    for j in 1..=rings_count {
      let v = j as f32 / n_lat as f32;
      let phi = v * PI;

      for i in 0..n_long {
        let u = i as f32 / n_long as f32;
        let theta = u * 2.0 * PI;

        let x = theta.sin() * phi.sin();
        let y = phi.cos();
        let z = theta.cos() * phi.sin();

        let id = object.vertices.len();
        object.vertices.push(Vertex { id, position: Vec3::new(x, y, z), _edge: 0 });
      }
    }

    let get_ring_vertex = |ring_idx: usize, long_idx: usize| -> usize {
      let wrapped_long = long_idx % n_long;
      2 + ring_idx * n_long + wrapped_long
    };

    for i in 0..n_long {
      let v_top = top_vertex.id;
      let v_curr = get_ring_vertex(0, i);
      let v_next = get_ring_vertex(0, i + 1);

      let face_id = object.faces.len();
      let e0_id = object.edges.len();
      let e1_id = e0_id + 1;
      let e2_id = e0_id + 2;

      object.faces.push(Face { id: face_id, edge: e0_id });

      let face_next = if i == n_long - 1 { 0 } else { face_id + 1 };
      let face_prev = if i == 0 { n_long - 1 } else { face_id - 1 };

      object.edges.push(Edge {
        id: e0_id,
        vertex_start: v_top, vertex_end: v_curr,
        face_clockwise: face_id,
        face_counterclockwise: face_prev,
        next_edge_clockwise: e1_id,
        next_edge_counterclockwise: 0,
      });
      object.edges.push(Edge {
        id: e1_id,
        vertex_start: v_curr, vertex_end: v_next,
        face_clockwise: face_id,
        face_counterclockwise: n_long + i,
        next_edge_clockwise: e2_id,
        next_edge_counterclockwise: 0,
      });
      object.edges.push(Edge {
        id: e2_id,
        vertex_start: v_next, vertex_end: v_top,
        face_clockwise: face_id,
        face_counterclockwise: face_next,
        next_edge_clockwise: e0_id,
        next_edge_counterclockwise: 0,
      });

      object.vertices[v_top]._edge = e0_id;
    }

    for j in 0..rings_count - 1 {
      for i in 0..n_long {
        let v_tl = get_ring_vertex(j, i);
        let v_tr = get_ring_vertex(j, i + 1);
        let v_br = get_ring_vertex(j + 1, i + 1);
        let v_bl = get_ring_vertex(j + 1, i);

        let face_id = object.faces.len();
        let base_edge_id = object.edges.len();

        object.faces.push(Face { id: face_id, edge: base_edge_id });

        let e_top = base_edge_id;
        let e_right = base_edge_id + 1;
        let e_bottom = base_edge_id + 2;
        let e_left = base_edge_id + 3;

        object.edges.push(Edge {
          id: e_top, vertex_start: v_tl, vertex_end: v_tr,
          face_clockwise: face_id, face_counterclockwise: 0,
          next_edge_clockwise: e_right, next_edge_counterclockwise: e_left
        });
        object.edges.push(Edge {
          id: e_right, vertex_start: v_tr, vertex_end: v_br,
          face_clockwise: face_id, face_counterclockwise: 0,
          next_edge_clockwise: e_bottom, next_edge_counterclockwise: e_top
        });
        object.edges.push(Edge {
          id: e_bottom, vertex_start: v_br, vertex_end: v_bl,
          face_clockwise: face_id, face_counterclockwise: 0,
          next_edge_clockwise: e_left, next_edge_counterclockwise: e_right
        });
        object.edges.push(Edge {
          id: e_left, vertex_start: v_bl, vertex_end: v_tl,
          face_clockwise: face_id, face_counterclockwise: 0,
          next_edge_clockwise: e_top, next_edge_counterclockwise: e_bottom
        });
      }
    }

    let last_ring_idx = rings_count - 1;
    for i in 0..n_long {
      let v_top = get_ring_vertex(last_ring_idx, i);
      let v_next_top = get_ring_vertex(last_ring_idx, i + 1);
      let v_bottom = bottom_vertex.id;

      let face_id = object.faces.len();
      let base_edge = object.edges.len();

      object.faces.push(Face { id: face_id, edge: base_edge });

      object.edges.push(Edge {
        id: base_edge, vertex_start: v_top, vertex_end: v_next_top,
        face_clockwise: face_id, face_counterclockwise: 0,
        next_edge_clockwise: base_edge + 1, next_edge_counterclockwise: base_edge + 2
      });
      object.edges.push(Edge {
        id: base_edge + 1, vertex_start: v_next_top, vertex_end: v_bottom,
        face_clockwise: face_id, face_counterclockwise: 0,
        next_edge_clockwise: base_edge + 2, next_edge_counterclockwise: base_edge
      });
      object.edges.push(Edge {
        id: base_edge + 2, vertex_start: v_bottom, vertex_end: v_top,
        face_clockwise: face_id, face_counterclockwise: 0,
        next_edge_clockwise: base_edge, next_edge_counterclockwise: base_edge + 1
      });
    }

    object.build_opengl();

    return object;
  }
}
