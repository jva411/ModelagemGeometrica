use std::f32;

use glam::Vec3;
use crate::objects::brep::winged_edge_object::{WingedEdgeObject, Vertex, Edge, Face};

impl WingedEdgeObject {
  pub fn new_cylinder(name: String, subdivisions: usize, height: f32, radius: f32) -> Self {
    let mut object = WingedEdgeObject::new(name);

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut faces = Vec::new();

    let top_face = Face { id: 0, edge: 0 };
    let bottom_face = Face { id: 1, edge: subdivisions };
    faces.push(top_face);
    faces.push(bottom_face);

    for i in 0..subdivisions {
      let angle = (i as f32) / (subdivisions as f32) * f32::consts::PI * 2.0;
      let x = radius * angle.cos();
      let z = radius * angle.sin();

      let top_vertex = Vertex { id: i, position: Vec3::new(x, height / 2.0, z), _edge: i };
      let bottom_vertex = Vertex { id: i + subdivisions, position: Vec3::new(x, -height / 2.0, z), _edge: i + subdivisions };
      vertices.push(top_vertex);
      vertices.push(bottom_vertex);

      let top_edge = Edge {
        id: i,
        vertex_start: i,
        vertex_end: (i + 1) % subdivisions,
        face_clockwise: i + 2,
        face_counterclockwise: top_face.id,
        next_edge_clockwise: ((i + 1) % subdivisions) + 2*subdivisions,
        next_edge_counterclockwise: if i == 0 { subdivisions - 1 } else { i - 1 },
      };
      let bottom_edge = Edge {
        id: i + subdivisions,
        vertex_start: i + subdivisions,
        vertex_end: (i + 1) % subdivisions + subdivisions,
        face_clockwise: bottom_face.id,
        face_counterclockwise: i + 2,
        next_edge_clockwise: ((i + 1) % subdivisions) + subdivisions,
        next_edge_counterclockwise: i + 2*subdivisions,
      };
      edges.push(top_edge);
      edges.push(bottom_edge);

      let lateral_edge = Edge {
        id: i + 2*subdivisions,
        vertex_start: top_vertex.id,
        vertex_end: bottom_vertex.id,
        face_clockwise: if i == 0 { subdivisions + 1 } else { i + 1 },
        face_counterclockwise: i + 2,
        next_edge_clockwise: if i == 0 { 2*subdivisions - 1 } else { subdivisions + i - 1 },
        next_edge_counterclockwise: top_edge.id,
      };
      edges.push(lateral_edge);

      let lateral_face = Face { id: i + 2, edge: lateral_edge.id };
      faces.push(lateral_face);
    }

    vertices.sort_by(|v1, v2| v1.id.cmp(&v2.id));
    edges.sort_by(|e1, e2| e1.id.cmp(&e2.id));
    faces.sort_by(|f1, f2| f1.id.cmp(&f2.id));

    object.vertices = vertices;
    object.edges = edges;
    object.faces = faces;

    return object;
  }
}
