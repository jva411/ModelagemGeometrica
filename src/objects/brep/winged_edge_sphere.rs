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

    let mut meridian_edges = vec![vec![0; n_long]; n_lat];
    let mut parallel_edges = vec![vec![0; n_long]; rings_count];

    for row in 0..n_lat {
      for col in 0..n_long {
        let v_start = if row == 0 { 0 } else { get_ring_vertex(row - 1, col) };
        let v_end = if row == n_lat - 1 { 1 } else { get_ring_vertex(row, col) };

        let edge_id = object.edges.len();
        meridian_edges[row][col] = edge_id;

        object.edges.push(Edge {
          id: edge_id,
          vertex_start: v_start,
          vertex_end: v_end,
          face_clockwise: 0, face_counterclockwise: 0,
          next_edge_clockwise: 0, next_edge_counterclockwise: 0,
        });
        object.vertices[v_start]._edge = edge_id;
      }
    }

    for row in 0..rings_count {
      for col in 0..n_long {
        let v_start = get_ring_vertex(row, col);
        let v_end = get_ring_vertex(row, col + 1);

        let edge_id = object.edges.len();
        parallel_edges[row][col] = edge_id;

        object.edges.push(Edge {
          id: edge_id,
          vertex_start: v_start,
          vertex_end: v_end,
          face_clockwise: 0, face_counterclockwise: 0,
          next_edge_clockwise: 0, next_edge_counterclockwise: 0,
        });
      }
    }

    for i in 0..n_long {
      let face_id = object.faces.len();

      let e_right = meridian_edges[0][(i + 1) % n_long];
      let e_bottom = parallel_edges[0][i];
      let e_left = meridian_edges[0][i];

      object.faces.push(Face { id: face_id, edge: e_left });

      let e_r_obj = &mut object.edges[e_right];
      e_r_obj.face_clockwise = face_id;
      e_r_obj.next_edge_clockwise = e_bottom;

      let e_b_obj = &mut object.edges[e_bottom];
      e_b_obj.face_counterclockwise = face_id;
      e_b_obj.next_edge_counterclockwise = e_left;

      let e_l_obj = &mut object.edges[e_left];
      e_l_obj.face_counterclockwise = 0;

      object.edges[e_left].face_counterclockwise = face_id;
      object.edges[e_left].next_edge_counterclockwise = e_right;

      object.edges[e_right].face_clockwise = face_id;
      object.edges[e_right].next_edge_clockwise = e_bottom;

      object.edges[e_bottom].face_counterclockwise = face_id;
      object.edges[e_bottom].next_edge_counterclockwise = e_left;
    }

    for j in 0..rings_count - 1 {
      for i in 0..n_long {
        let face_id = object.faces.len();

        let e_top = parallel_edges[j][i];
        let e_right = meridian_edges[j + 1][(i + 1) % n_long];
        let e_bottom = parallel_edges[j + 1][i];
        let e_left = meridian_edges[j + 1][i];

        object.faces.push(Face { id: face_id, edge: e_top });

        object.edges[e_top].face_clockwise = face_id;
        object.edges[e_top].next_edge_clockwise = e_right;

        object.edges[e_right].face_clockwise = face_id;
        object.edges[e_right].next_edge_clockwise = e_bottom;

        object.edges[e_bottom].face_counterclockwise = face_id;
        object.edges[e_bottom].next_edge_counterclockwise = e_left;

        object.edges[e_left].face_counterclockwise = face_id;
        object.edges[e_left].next_edge_counterclockwise = e_top;
      }
    }

    let last_ring_idx = rings_count - 1;
    for i in 0..n_long {
      let face_id = object.faces.len();

      let e_top = parallel_edges[last_ring_idx][i];
      let e_right = meridian_edges[n_lat - 1][(i + 1) % n_long];
      let e_left = meridian_edges[n_lat - 1][i];

      object.faces.push(Face { id: face_id, edge: e_top });

      object.edges[e_top].face_clockwise = face_id;
      object.edges[e_top].next_edge_clockwise = e_right;

      object.edges[e_right].face_clockwise = face_id;
      object.edges[e_right].next_edge_clockwise = e_left;

      object.edges[e_left].face_counterclockwise = face_id;
      object.edges[e_left].next_edge_counterclockwise = e_top;
    }

    object.vertices.sort_by(|v1, v2| v1.id.cmp(&v2.id));
    object.edges.sort_by(|e1, e2| e1.id.cmp(&e2.id));
    object.faces.sort_by(|f1, f2| f1.id.cmp(&f2.id));

    return object;
  }
}
