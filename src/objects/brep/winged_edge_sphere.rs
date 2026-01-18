use glam::Vec3;
use crate::objects::brep::winged_edge_object::{WingedEdgeObject, Vertex, Edge, Face};

impl WingedEdgeObject {
    pub fn new_sphere(name: String, subdivisions: usize) -> Self {
        let mut object = WingedEdgeObject::new(name);

        let vertices = vec![
            Vec3::new( 0.0,  1.0,  0.0),
            Vec3::new(-1.0,  0.0,  0.0),
            Vec3::new( 0.0,  0.0, -1.0),
            Vec3::new( 1.0,  0.0,  0.0),
            Vec3::new( 0.0,  0.0,  1.0),
            Vec3::new( 0.0, -1.0,  0.0),
        ]
            .iter()
            .enumerate()
            .map(|(index, position) | Vertex { id: index, position: *position, _edge: index })
            .collect();

        let top_front_left_face = Face { id: 0, edge: 0 };
        let top_front_right_face = Face { id: 1, edge: 1 };
        let top_back_right_face = Face { id: 2, edge: 2 };
        let top_back_left_face = Face { id: 3, edge: 3 };
        let bottom_front_left_face = Face { id: 4, edge: 4 };
        let bottom_front_right_face = Face { id: 5, edge: 5 };
        let bottom_back_right_face = Face { id: 6, edge: 6 };
        let bottom_back_left_face = Face { id: 7, edge: 7 };
        let faces = vec![top_front_left_face, top_front_right_face, top_back_right_face, top_back_left_face, bottom_front_left_face, bottom_front_right_face, bottom_back_right_face, bottom_back_left_face];

        let edges = vec![
            // Top vertex to middle edges
            Edge {
                id: 0,
                vertex_start: 1,
                vertex_end: 0,
                face_clockwise: top_front_left_face.id,
                face_counterclockwise: top_back_left_face.id,
                next_edge_clockwise: 1,
                next_edge_counterclockwise: 11,
            },
            Edge {
                id: 1,
                vertex_start: 2,
                vertex_end: 0,
                face_clockwise: top_front_right_face.id,
                face_counterclockwise: top_front_left_face.id,
                next_edge_clockwise: 2,
                next_edge_counterclockwise: 8,
            },
            Edge {
                id: 2,
                vertex_start: 3,
                vertex_end: 0,
                face_clockwise: top_back_right_face.id,
                face_counterclockwise: top_front_right_face.id,
                next_edge_clockwise: 3,
                next_edge_counterclockwise: 9,
            },
            Edge {
                id: 3,
                vertex_start: 4,
                vertex_end: 0,
                face_clockwise: top_back_left_face.id,
                face_counterclockwise: top_back_right_face.id,
                next_edge_clockwise: 0,
                next_edge_counterclockwise: 10,
            },

            // Bottom vertex to middle edges
            Edge {
                id: 4,
                vertex_start: 5,
                vertex_end: 1,
                face_clockwise: bottom_front_left_face.id,
                face_counterclockwise: bottom_back_left_face.id,
                next_edge_clockwise: 8,
                next_edge_counterclockwise: 7,
            },
            Edge {
                id: 5,
                vertex_start: 5,
                vertex_end: 2,
                face_clockwise: bottom_front_right_face.id,
                face_counterclockwise: bottom_front_left_face.id,
                next_edge_clockwise: 9,
                next_edge_counterclockwise: 4,
            },
            Edge {
                id: 6,
                vertex_start: 5,
                vertex_end: 3,
                face_clockwise: bottom_back_right_face.id,
                face_counterclockwise: bottom_front_right_face.id,
                next_edge_clockwise: 10,
                next_edge_counterclockwise: 5,
            },
            Edge {
                id: 7,
                vertex_start: 5,
                vertex_end: 4,
                face_clockwise: bottom_back_left_face.id,
                face_counterclockwise: bottom_back_right_face.id,
                next_edge_clockwise: 11,
                next_edge_counterclockwise: 6,
            },

            // Middle edges
            Edge {
                id: 8,
                vertex_start: 2,
                vertex_end: 1,
                face_clockwise: top_front_left_face.id,
                face_counterclockwise: bottom_front_left_face.id,
                next_edge_clockwise: 0,
                next_edge_counterclockwise: 5,
            },
            Edge {
                id: 9,
                vertex_start: 3,
                vertex_end: 2,
                face_clockwise: top_front_right_face.id,
                face_counterclockwise: bottom_front_right_face.id,
                next_edge_clockwise: 1,
                next_edge_counterclockwise: 6,
            },
            Edge {
                id: 10,
                vertex_start: 4,
                vertex_end: 3,
                face_clockwise: top_back_right_face.id,
                face_counterclockwise: bottom_back_right_face.id,
                next_edge_clockwise: 2,
                next_edge_counterclockwise: 7,
            },
            Edge {
                id: 11,
                vertex_start: 1,
                vertex_end: 4,
                face_clockwise: top_back_left_face.id,
                face_counterclockwise: bottom_back_left_face.id,
                next_edge_clockwise: 3,
                next_edge_counterclockwise: 4,
            },
        ];

        object.vertices = vertices;
        object.edges = edges;
        object.faces = faces;

        for _ in 0..subdivisions {
            object.subdivide_sphere();
        }

        return object;
    }

    fn subdivide_sphere(&mut self) {
        let mut new_vertices = self.vertices.clone();
        let mut new_edges = self.edges.clone();
        let mut new_faces = Vec::new();

        for face in self.faces.iter() {
            let mut neigh_vertices = face.get_neighbors_vertices_in_order(self);
            let mut neigh_edges = face.get_neighbors_edges_in_order(self);
            let mut incoming_vertices = neigh_vertices
                .iter()
                .map(|v| neigh_edges.iter().filter(|e| new_edges[*(*e)].vertex_end == *v).count())
                .collect::<Vec<_>>();

            let desired_incoming_order = vec![1, 2, 0];
            for (i, x) in desired_incoming_order.iter().enumerate() {
                if incoming_vertices[i] != *x {
                    let pos = incoming_vertices.iter().position(|&e| e == *x).unwrap();
                    incoming_vertices.swap(i, pos);
                    neigh_vertices.swap(i, pos);
                    neigh_edges.swap(i, pos);

                    let (mut v1, mut v2) = (new_vertices[neigh_vertices[i]], new_vertices[neigh_vertices[pos]]);
                    let (mut e1, mut e2) = (new_edges[neigh_edges[i]], new_edges[neigh_edges[pos]]);
                    new_edges.swap(e1.id, e2.id);
                    new_vertices.swap(v1.id, v2.id);

                    let aux = v1.id;
                    v1.id = v2.id;
                    v2.id = aux;

                    let aux = e1.id;
                    e1.id = e2.id;
                    e2.id = aux;
                }
            }

            let center_position = face.calc_center_position(self).normalize();
            let new_edge_0_id = new_edges.len();
            let new_face_0_id = new_faces.len();
            let new_vertex = Vertex {
                id: new_vertices.len(),
                position: center_position,
                _edge: new_edge_0_id,
            };
            new_vertices.push(new_vertex);

            let new_face_0 = Face { id: new_face_0_id, edge: new_edge_0_id };
            let new_face_1 = Face { id: new_face_0_id + 1, edge: new_edge_0_id + 1 };
            let new_face_2 = Face { id: new_face_0_id + 2, edge: new_edge_0_id + 2 };
            new_faces.push(new_face_0);
            new_faces.push(new_face_1);
            new_faces.push(new_face_2);

            let new_edge_0 = Edge {
                id: new_edge_0_id,
                vertex_start: new_vertex.id,
                vertex_end: neigh_vertices[0],
                face_clockwise: new_face_2.id,
                face_counterclockwise: new_face_0_id,
                next_edge_clockwise: neigh_edges[0],
                next_edge_counterclockwise: new_edge_0_id + 1,
            };
            new_edges.push(new_edge_0);

            let new_edge_1 = Edge {
                id: new_edge_0_id + 1,
                vertex_start: neigh_vertices[2],
                vertex_end: new_vertex.id,
                face_clockwise: new_face_1.id,
                face_counterclockwise: new_face_0_id,
                next_edge_clockwise: new_edge_0_id + 2,
                next_edge_counterclockwise: neigh_edges[2],
            };
            new_edges.push(new_edge_1);

            let new_edge_2 = Edge {
                id: new_edge_0_id + 2,
                vertex_start: new_vertex.id,
                vertex_end: neigh_vertices[1],
                face_clockwise: new_face_1.id,
                face_counterclockwise: new_face_2.id,
                next_edge_clockwise: neigh_edges[1],
                next_edge_counterclockwise: new_edge_0_id,
            };
            new_edges.push(new_edge_2);

           new_edges[neigh_edges[0]].next_edge_clockwise = new_edge_2.id;
           new_edges[neigh_edges[0]].face_clockwise = new_face_2.id;

           new_edges[neigh_edges[1]].next_edge_counterclockwise = new_edge_1.id;
           new_edges[neigh_edges[1]].face_counterclockwise = new_face_1.id;

           new_edges[neigh_edges[2]].next_edge_clockwise = new_edge_0.id;
           new_edges[neigh_edges[2]].face_clockwise = new_face_0.id;
        }

        self.vertices = new_vertices;
        self.edges = new_edges;
        self.faces = new_faces;
    }
}
