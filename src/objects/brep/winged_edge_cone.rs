use std::f32;

use glam::Vec3;
use crate::objects::brep::winged_edge_object::{WingedEdgeObject, Vertex, Edge, Face};

impl WingedEdgeObject {
  pub fn new_cone(name: String, subdivisions: usize, height: f32, radius: f32) -> Self {
    let mut object = WingedEdgeObject::new(name);

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut faces = Vec::new();

    let tip_vertex = Vertex { id: 0, position: Vec3::new(0.0, height / 2.0, 0.0), _edge: 0 };
    vertices.push(tip_vertex);


    let angle = 0.0_f32;
    let x = radius * angle.cos();
    let z = radius * angle.sin();
    let first_base_vertex = Vertex { id: 1, position: Vec3::new(x, -height / 2.0, z), _edge: 1 };
    vertices.push(first_base_vertex);

    let first_edge = Edge {
      id: 0,
      vertex_start: 0,
      vertex_end: 1,
      face_clockwise: subdivisions,
      face_counterclockwise: 1,
      next_edge_clockwise: 2*subdivisions - 1,
      next_edge_counterclockwise: 2,
    };
    edges.push(first_edge);

    let base_face = Face { id: 0, edge: 1 };
    faces.push(base_face);

    for i in 1..=subdivisions {
      let angle = (i as f32 / subdivisions as f32) * f32::consts::PI * -2.0;
      let x = radius * angle.cos();
      let z = radius * angle.sin();

      let vertex = if i == subdivisions {
        vertices[1]
      } else {
        let vertex = Vertex { id: i + 1, position: Vec3::new(x, -height / 2.0, z), _edge: ((i % subdivisions) + 1)*2 - 1 };
        vertices.push(vertex);
        vertex
      };

      let base_edge = Edge {
        id: i*2 - 1,
        vertex_start: i,
        vertex_end: vertex.id,
        face_clockwise: 0,
        face_counterclockwise: i,
        next_edge_clockwise: (i % subdivisions)*2 + 1,
        next_edge_counterclockwise: i*2 - 2,
      };
      edges.push(base_edge);

      let lateral_edge = if i == subdivisions {
        first_edge
      } else {
        let edge = Edge {
          id: i*2,
          vertex_start: 0,
          vertex_end: vertex.id,
          face_clockwise: i,
          face_counterclockwise: (i % subdivisions) + 1,
          next_edge_clockwise: i*2 - 1,
          next_edge_counterclockwise: ((i + 1) % subdivisions)*2,
        };
        edges.push(edge);
        edge
      };

      let latera_face = Face { id: i, edge: lateral_edge.id };
      faces.push(latera_face);
    }

    object.vertices = vertices;
    object.edges = edges;
    object.faces = faces;

    return object;
  }
}
