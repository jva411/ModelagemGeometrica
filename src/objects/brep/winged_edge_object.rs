use std::collections::{HashMap, HashSet};

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

pub enum MemberType {
  Vertex,
  Edge,
  Face,
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

    let mut neigh_vertices = vec![self.id];
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
    let mut neigh_edges = object.edges
      .iter()
      .filter(|e| e.id != self.id && (
        e.vertex_start == self.vertex_start
        || e.vertex_start == self.vertex_end
        || e.vertex_end == self.vertex_start
        || e.vertex_end == self.vertex_end))
      .map(|e| e.id)
      .collect::<Vec<_>>();

    neigh_edges.push(self.id);

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
    let mut neigh_faces: HashSet<usize> = HashSet::new();
    for edge in neigh_edges.iter() {
      neigh_vertices.insert(edge.vertex_start);
      neigh_vertices.insert(edge.vertex_end);
      neigh_faces.insert(if edge.face_clockwise == self.id { edge.face_counterclockwise } else { edge.face_clockwise });
    }
    neigh_faces.insert(self.id);

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

  pub main_vao: VAO,
  pub main_vbo: VBO,
  pub main_ebo: EBO,
  pub opengl_vertices: Vec<f32>,
  pub opengl_indices: Vec<u32>,

  pub highlighted_vertices: Vec<usize>,
  pub highlighted_edges: Vec<usize>,
  pub highlighted_faces: Vec<usize>,

  pub face_index_ranges: HashMap<usize, (usize, i32)>,
  pub highlight_vao: VAO,
  pub highlight_vbo: VBO,
  pub highlight_count_lines: i32,
  pub highlight_count_points: i32,
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

      main_vao: VAO::new(),
      main_vbo: VBO::new(),
      main_ebo: EBO::new(),
      opengl_vertices: Vec::new(),
      opengl_indices: Vec::new(),

      highlighted_vertices: Vec::new(),
      highlighted_edges: Vec::new(),
      highlighted_faces: Vec::new(),

      face_index_ranges: HashMap::new(),
      highlight_vao: VAO::new(),
      highlight_vbo: VBO::new(),
      highlight_count_lines: 0,
      highlight_count_points: 0,
    };
  }

  pub fn build_opengl(&mut self) {
    self.main_vao.delete();
    self.main_vbo.delete();
    self.main_ebo.delete();

    self.main_vao = VAO::new();
    self.main_vbo = VBO::new();
    self.main_ebo = EBO::new();

    self.main_vao.bind();
    self.main_vbo.bind();
    self.main_ebo.bind();

    self.opengl_vertices = Vec::new();
    self.opengl_indices = Vec::new();
    self.face_index_ranges = HashMap::new();

    for face in self.faces.iter() {
      let start_index = self.opengl_indices.len();
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

      let new_faces_count = self.opengl_indices.len() - start_index;
      self.face_index_ranges.insert(face.id, (start_index, new_faces_count as i32));
    }

    self.main_vao.add_attribute(0, 6 * SIZE_F32, 0);
    self.main_vao.add_attribute(1, 6 * SIZE_F32, 3 * SIZE_F32);
    self.main_vbo.send_data(&self.opengl_vertices);
    self.main_ebo.send_data(&self.opengl_indices);
  }

  pub fn highlight_member(&mut self, _type: MemberType, member_id: usize) {
    let neighbors = match _type {
      MemberType::Vertex => self.vertices[member_id].get_neighbors(self),
      MemberType::Edge => self.edges[member_id].get_neighbors(self),
      MemberType::Face => self.faces[member_id].get_neighbors(self),
    };

    self.highlighted_vertices = neighbors.vertices;
    self.highlighted_edges = neighbors.edges;
    self.highlighted_faces = neighbors.faces;

    let mut data: Vec<f32> = Vec::new();

    self.highlight_count_lines = 0;
    for edge_id in &self.highlighted_edges {
      if let Some(edge) = self.edges.iter().find(|e| e.id == *edge_id) {
        let v_start = self.vertices[edge.vertex_start].position;
        let v_end = self.vertices[edge.vertex_end].position;

        data.push(v_start.x); data.push(v_start.y); data.push(v_start.z);
        data.push(v_end.x); data.push(v_end.y); data.push(v_end.z);

        self.highlight_count_lines += 2;
      }
    }

    self.highlight_count_points = 0;
    for vertex_id in &self.highlighted_vertices {
      if let Some(vertex) = self.vertices.iter().find(|v| v.id == *vertex_id) {
        let pos = vertex.position;
        data.push(pos.x); data.push(pos.y); data.push(pos.z);
        self.highlight_count_points += 1;
      }
    }

    if !data.is_empty() {
      self.highlight_vao.bind();
      self.highlight_vbo.bind();
      self.highlight_vbo.send_data(&data);
      self.highlight_vao.add_attribute(0, 3 * SIZE_F32, 0);
    }
  }

  fn draw_highlights(&self, program: &Program) {
    if !self.highlighted_faces.is_empty() {
      unsafe {
        gl::Enable(gl::POLYGON_OFFSET_FILL);
        gl::PolygonOffset(-1.0, -1.0);

        program.set_uniform_vec3f("uOverrideColor", Vec3::new(1.0, 0.5, 0.0)).unwrap();
      }

      for face_id in &self.highlighted_faces {
        if let Some((start_idx, count)) = self.face_index_ranges.get(face_id) {
          let offset = (*start_idx * std::mem::size_of::<u32>()) as *const std::ffi::c_void;
          unsafe {
            gl::DrawElements(gl::TRIANGLES, *count, gl::UNSIGNED_INT, offset);
          }
        }
      }

      unsafe { gl::Disable(gl::POLYGON_OFFSET_FILL); }
    }

    if self.highlight_count_lines > 0 || self.highlight_count_points > 0 {
      self.highlight_vao.bind();

      program.set_uniform_vec3f("uOverrideColor", Vec3::new(1.0, 1.0, 0.0)).unwrap();

      unsafe {
        gl::Disable(gl::DEPTH_TEST);
      }

      if self.highlight_count_lines > 0 {
        unsafe {
          gl::LineWidth(2.0);
          gl::DrawArrays(gl::LINES, 0, self.highlight_count_lines);
          gl::LineWidth(1.0);
        }
      }

      if self.highlight_count_points > 0 {
        let start_points = self.highlight_count_lines;
        unsafe {
          gl::PointSize(10.0);
          gl::DrawArrays(gl::POINTS, start_points, self.highlight_count_points);
          gl::PointSize(1.0);
        }
      }

      unsafe { gl::Enable(gl::DEPTH_TEST); }
    }
  }
}

impl Object for WingedEdgeObject {
  mesh_implement_partial_Object!();

  fn draw(&self, program: &Program, base_transform: Option<Transform>) {
    self.main_vao.bind();
    self.main_vbo.bind();
    self.main_ebo.bind();

    let model_transform = match base_transform {
      Some(t) => &self.transform.concat(&t),
      None => &self.transform,
    };
    model_transform.send_to_program(&program);
    self.material.send_to_program(&program);

    unsafe {
      gl::DrawElements(gl::TRIANGLES, self.opengl_indices.len() as i32, gl::UNSIGNED_INT, 0 as *const _);
    }

    program.set_uniform_bool("uUseOverrideColor", true).unwrap();
    self.draw_highlights(program);
    program.set_uniform_bool("uUseOverrideColor", false).unwrap();
  }
}
