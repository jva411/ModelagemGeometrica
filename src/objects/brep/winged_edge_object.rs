use std::collections::HashSet;

use glam::Vec3;
use uuid::Uuid;

use crate::{mesh_implement_partial_Object, objects::{instanced::instanced_cube::SIZE_F32, object::Object, octree::octree_object::OctreeObject}, opengl::{ebo::EBO, program::Program, vao::VAO, vbo::VBO}, utils::{material::Material, transform::Transform}};


macro_rules! append_opengl_vertex {
  ($array:expr, $vertex:expr, $normal:expr) => {
    $array.push($vertex.position.x);
    $array.push($vertex.position.y);
    $array.push($vertex.position.z);
    $array.push($normal.x);
    $array.push($normal.y);
    $array.push($normal.z);
  };
}

pub struct Neighbors {
  pub vertices: Vec<usize>,
  pub edges: Vec<usize>,
  pub faces: Vec<usize>,
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
  pub id: usize,
  pub position: Vec3,
  pub _edge: usize,
}

impl Vertex {
  pub fn get_neighbors(&self, object: &WingedEdgeObject) -> Neighbors {
    let neigh_edges = object.edges
      .iter()
      .filter(|e| e.vertex_start == self.id || e.vertex_end == self.id)
      .collect::<Vec<_>>();

    let mut neigh_vertices = Vec::new();
    let mut neigh_faces = HashSet::new();
    for edge in neigh_edges.iter() {
      neigh_vertices.push(if edge.vertex_start == self.id { edge.vertex_end } else { edge.vertex_start });
      neigh_faces.insert(edge.face_clockwise);
      neigh_faces.insert(edge.face_counterclockwise);
    }

    return Neighbors {
      vertices: neigh_vertices,
      edges: neigh_edges.iter().map(|e| e.id).collect(),
      faces: neigh_faces.into_iter().collect(),
    };
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Edge {
  pub id: usize,

  pub vertex_start: usize,
  pub vertex_end: usize,

  pub face_clockwise: usize,
  pub face_counterclockwise: usize,

  pub next_edge_clockwise: usize,
  pub next_edge_counterclockwise: usize,
}

impl Edge {
  pub fn get_neighbors(&self, object: &WingedEdgeObject) -> Neighbors {
    let neigh_vertices = vec![self.vertex_start, self.vertex_end];
    let neigh_faces = vec![self.face_clockwise, self.face_counterclockwise];
    let neigh_edges = object.edges
      .iter()
      .filter(|e| e.id != self.id && (
        e.vertex_start == self.vertex_start
        || e.vertex_start == self.vertex_end
        || e.vertex_end == self.vertex_start
        || e.vertex_end == self.vertex_end))
      .map(|e| e.id)
      .collect::<Vec<_>>();

    return Neighbors {
      vertices: neigh_vertices,
      edges: neigh_edges,
      faces: neigh_faces,
    };
  }
}


#[derive(Debug, Copy, Clone)]
pub struct Face {
  pub id: usize,
  pub edge: usize,
}

impl Face {
  pub fn get_neighbors(&self, object: &WingedEdgeObject) -> Neighbors {
    let neigh_edges = object.edges
      .iter()
      .filter(|e| e.face_clockwise == self.id || e.face_counterclockwise == self.id)
      .collect::<Vec<_>>();

    let mut neigh_vertices = HashSet::new();
    let mut neigh_faces = HashSet::new();
    for edge in neigh_edges.iter() {
      neigh_vertices.insert(edge.vertex_start);
      neigh_vertices.insert(edge.vertex_end);
      neigh_faces.insert(if edge.face_clockwise == self.id { edge.face_counterclockwise } else { edge.face_clockwise });
    }

    return Neighbors {
      vertices: neigh_vertices.into_iter().collect(),
      edges: neigh_edges.iter().map(|e| e.id).collect(),
      faces: neigh_faces.into_iter().collect(),
    };
  }

  pub fn get_neighbors_vertices_in_order(&self, object: &WingedEdgeObject) -> Vec<usize> {
    let mut face_vertices_ids = Vec::new();
    let start_edge_id = self.edge;
    let mut current_edge_id = start_edge_id;

    loop {
      let edge = &object.edges[current_edge_id];

      if edge.face_clockwise == self.id {
        face_vertices_ids.push(edge.vertex_start);
        current_edge_id = edge.next_edge_clockwise;
      } else {
        face_vertices_ids.push(edge.vertex_end);
        current_edge_id = edge.next_edge_counterclockwise;
      }

      if current_edge_id == start_edge_id { break; }
    }

    return face_vertices_ids;
  }

  pub fn get_neighbors_edges_in_order(&self, object: &WingedEdgeObject) -> Vec<usize> {
    let mut face_edges_ids = Vec::new();
    let start_edge_id = self.edge;
    let mut current_edge_id = start_edge_id;

    loop {
      let edge = &object.edges[current_edge_id];

      if edge.face_clockwise == self.id {
        face_edges_ids.push(edge.id);
        current_edge_id = edge.next_edge_clockwise;
      } else {
        face_edges_ids.push(edge.id);
        current_edge_id = edge.next_edge_counterclockwise;
      }

      if current_edge_id == start_edge_id { break; }
    }

    return face_edges_ids;
  }

  pub fn calc_center_position(&self, object: &WingedEdgeObject) -> Vec3 {
    let neigh_vertices = self.get_neighbors_vertices_in_order(object);
    let mut center_position = Vec3::ZERO;
    for &v_id in &neigh_vertices {
      center_position += object.vertices[v_id].position;
    }
    center_position /= neigh_vertices.len() as f32;

    return center_position;
  }
}


pub struct WingedEdgeObject {
  pub id: Uuid,
  pub name: String,
  pub transform: Transform,
  pub material: Material,

  pub vertices: Vec<Vertex>,
  pub edges: Vec<Edge>,
  pub faces: Vec<Face>,

  pub vao: VAO,
  pub vbo: VBO,
  pub ebo: EBO,
  pub opengl_vertices: Vec<f32>,
  pub opengl_indices: Vec<u32>,

  pub highlighted_vertices: HashSet<usize>,
  pub highlighted_edges: HashSet<usize>,
  pub highlighted_faces: HashSet<usize>,
}

impl WingedEdgeObject {
  pub fn new(name: String) -> Self {
    return WingedEdgeObject {
      id: Uuid::new_v4(),
      name,
      transform: Transform::default(),
      material: Material::default(),
      vertices: Vec::new(),
      edges: Vec::new(),
      faces: Vec::new(),

      vao: VAO::new(),
      vbo: VBO::new(),
      ebo: EBO::new(),
      opengl_vertices: Vec::new(),
      opengl_indices: Vec::new(),

      highlighted_vertices: HashSet::new(),
      highlighted_edges: HashSet::new(),
      highlighted_faces: HashSet::new(),
    };
  }

  pub fn build_opengl(&mut self) {
    self.vao.delete();
    self.vbo.delete();
    self.ebo.delete();

    self.vao = VAO::new();
    self.vbo = VBO::new();
    self.ebo = EBO::new();

    self.vao.bind();
    self.vbo.bind();
    self.ebo.bind();

    self.opengl_vertices = Vec::new();
    self.opengl_indices = Vec::new();

    for face in self.faces.iter() {
      let face_vertices_ids = face.get_neighbors_vertices_in_order(&self);

      let mut center_position = Vec3::ZERO;
      for &v_id in &face_vertices_ids {
        center_position += self.vertices[v_id].position;
      }
      center_position /= face_vertices_ids.len() as f32;

      let v0 = self.vertices[face_vertices_ids[0]].position;
      let v1 = self.vertices[face_vertices_ids[1]].position;
      let v2 = self.vertices[face_vertices_ids[2]].position;
      let normal = (v2 - v0).cross(v1 - v0).normalize();

      let center_idx = (self.opengl_vertices.len() / 6) as u32;
      self.opengl_vertices.push(center_position.x);
      self.opengl_vertices.push(center_position.y);
      self.opengl_vertices.push(center_position.z);
      self.opengl_vertices.push(normal.x);
      self.opengl_vertices.push(normal.y);
      self.opengl_vertices.push(normal.z);

      append_opengl_vertex!(self.opengl_vertices, &self.vertices[face_vertices_ids[0]], normal);
      for i in 0..face_vertices_ids.len() {
        let start_vertex_index = (self.opengl_vertices.len() / 6) as u32 - 1;

        let end_vertex_id = face_vertices_ids[(i + 1) % face_vertices_ids.len()];
        let end_vertex_index = start_vertex_index + 1;
        let end_vertex = &self.vertices[end_vertex_id];
        append_opengl_vertex!(self.opengl_vertices, end_vertex, normal);

        self.opengl_indices.push(center_idx);
        self.opengl_indices.push(end_vertex_index);
        self.opengl_indices.push(start_vertex_index);
      }
    }

    self.vao.add_attribute(0, 6 * SIZE_F32, 0);
    self.vao.add_attribute(1, 6 * SIZE_F32, 3 * SIZE_F32);
    self.vbo.send_data(&self.opengl_vertices);
    self.ebo.send_data(&self.opengl_indices);
  }
}

impl Object for WingedEdgeObject {
  mesh_implement_partial_Object!();

  fn draw(&self, program: &Program, base_transform: Option<Transform>) {
    self.vao.bind();
    self.vbo.bind();
    self.ebo.bind();

    let model_transform = match base_transform {
      Some(t) => &self.transform.concat(&t),
      None => &self.transform,
    };
    model_transform.send_to_program(&program);
    self.material.send_to_program(&program);

    unsafe {
      gl::DrawElements(gl::TRIANGLES, self.opengl_indices.len() as i32, gl::UNSIGNED_INT, 0 as *const _);
    }
  }
}
