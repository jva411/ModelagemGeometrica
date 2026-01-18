use glam::Vec3;

use crate::objects::brep::winged_edge_object::{Edge, Face, Vertex, WingedEdgeObject};

impl WingedEdgeObject {
  pub fn new_cube(name: String) -> Self {
    let mut object = WingedEdgeObject::new(name);

    let vertices_positions = vec![
      Vec3::new(-0.5, -0.5, -0.5),
      Vec3::new( 0.5, -0.5, -0.5),
      Vec3::new( 0.5,  0.5, -0.5),
      Vec3::new(-0.5,  0.5, -0.5),
      Vec3::new(-0.5, -0.5,  0.5),
      Vec3::new(-0.5,  0.5,  0.5),
      Vec3::new( 0.5,  0.5,  0.5),
      Vec3::new( 0.5, -0.5,  0.5),
    ];

    let vertices = vertices_positions
      .iter()
      .enumerate()
      .map(|(index, position)| Vertex { id: index, position: *position, _edge: index })
      .collect::<Vec<_>>();

    let back_face = Face { id: 0, edge: 0 };
    let front_face = Face { id: 1, edge: 4 };
    let left_face = Face { id: 2, edge: 3 };
    let right_face = Face { id: 3, edge: 1 };
    let top_face = Face { id: 4, edge: 10 };
    let bottom_face = Face { id: 5, edge: 11 };

    let edges = vec![
      // Back face edges
      Edge {
        id: 0,
        vertex_start: 0,
        vertex_end: 1,
        face_clockwise: back_face.id,
        face_counterclockwise: bottom_face.id,
        next_edge_clockwise: 1,
        next_edge_counterclockwise: 8,
      },
      Edge {
        id: 1,
        vertex_start: 1,
        vertex_end: 2,
        face_clockwise: back_face.id,
        face_counterclockwise: right_face.id,
        next_edge_clockwise: 2,
        next_edge_counterclockwise: 11,
      },
      Edge {
        id: 2,
        vertex_start: 2,
        vertex_end: 3,
        face_clockwise: back_face.id,
        face_counterclockwise: top_face.id,
        next_edge_clockwise: 3,
        next_edge_counterclockwise: 10,
      },
      Edge {
        id: 3,
        vertex_start: 3,
        vertex_end: 0,
        face_clockwise: back_face.id,
        face_counterclockwise: left_face.id,
        next_edge_clockwise: 0,
        next_edge_counterclockwise: 9,
      },

      // Front face edges
      Edge {
        id: 4,
        vertex_start: 4,
        vertex_end: 5,
        face_clockwise: front_face.id,
        face_counterclockwise: left_face.id,
        next_edge_clockwise: 5,
        next_edge_counterclockwise: 8,
      },
      Edge {
        id: 5,
        vertex_start: 5,
        vertex_end: 6,
        face_clockwise: front_face.id,
        face_counterclockwise: top_face.id,
        next_edge_clockwise: 6,
        next_edge_counterclockwise: 9,
      },
      Edge {
        id: 6,
        vertex_start: 6,
        vertex_end: 7,
        face_clockwise: front_face.id,
        face_counterclockwise: right_face.id,
        next_edge_clockwise: 7,
        next_edge_counterclockwise: 10,
      },
      Edge {
        id: 7,
        vertex_start: 7,
        vertex_end: 4,
        face_clockwise: front_face.id,
        face_counterclockwise: bottom_face.id,
        next_edge_clockwise: 4,
        next_edge_counterclockwise: 11,
      },

      // Lateral faces edges
      Edge {
        id: 8,
        vertex_start: 4,
        vertex_end: 0,
        face_clockwise: left_face.id,
        face_counterclockwise: bottom_face.id,
        next_edge_clockwise: 3,
        next_edge_counterclockwise: 7,
      },
      Edge {
        id: 9,
        vertex_start: 3,
        vertex_end: 5,
        face_clockwise: left_face.id,
        face_counterclockwise: top_face.id,
        next_edge_clockwise: 4,
        next_edge_counterclockwise: 2,
      },
      Edge {
        id: 10,
        vertex_start: 6,
        vertex_end: 2,
        face_clockwise: right_face.id,
        face_counterclockwise: top_face.id,
        next_edge_clockwise: 1,
        next_edge_counterclockwise: 5,
      },
      Edge {
        id: 11,
        vertex_start: 1,
        vertex_end: 7,
        face_clockwise: right_face.id,
        face_counterclockwise: bottom_face.id,
        next_edge_clockwise: 6,
        next_edge_counterclockwise: 0,
      },
    ];


    object.vertices = vertices;
    object.edges = edges;
    object.faces = vec![
      back_face,
      front_face,
      left_face,
      right_face,
      top_face,
      bottom_face,
    ];

    return object;
  }
}
