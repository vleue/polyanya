use crate::vendored_spade::RemovalResult;

use super::dcel::{Dcel, EdgeEntry, FaceEntry, HalfEdgeEntry, VertexEntry};
use super::handles::*;

use smallvec::SmallVec;

use std::{vec, vec::Vec};

pub const OUTER_FACE_HANDLE: FixedFaceHandle<PossiblyOuterTag> = new_fixed_face_handle(0);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IsolateVertexResult {
    pub new_edges: Vec<FixedUndirectedEdgeHandle>,
    pub smallest_new_edge: FixedUndirectedEdgeHandle,
    pub edges_to_remove: Vec<FixedUndirectedEdgeHandle>,
    pub faces_to_remove: Vec<FixedFaceHandle<InnerTag>>,
}

impl IsolateVertexResult {
    pub fn is_new_edge(&self, edge: FixedUndirectedEdgeHandle) -> bool {
        edge >= self.smallest_new_edge
    }
}

pub fn disconnect_edge_strip<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    edge_strip: Vec<FixedDirectedEdgeHandle>,
) -> IsolateVertexResult
where
    DE: Default,
    UE: Default,
    F: Default,
{
    let mut edges_to_remove = Vec::new();
    let mut faces_to_remove = Vec::new();

    for edge in edge_strip {
        let edge_handle = dcel.directed_edge(edge);
        edges_to_remove.push(edge_handle.prev().as_undirected().fix());
        faces_to_remove.extend(edge_handle.face().as_inner().map(|e| e.fix()));

        let from = edge_handle.from().fix();

        let prev = edge_handle.ccw().prev().fix();
        dcel.half_edge_mut(prev).next = edge;
        dcel.half_edge_mut(edge).prev = prev;

        dcel.half_edge_mut(edge).face = OUTER_FACE_HANDLE;

        dcel.faces[OUTER_FACE_HANDLE.index()].adjacent_edge = Some(edge);
        dcel.vertices[from.index()].out_edge = Some(edge);
    }

    IsolateVertexResult {
        new_edges: Vec::new(),
        smallest_new_edge: FixedUndirectedEdgeHandle::new(0),
        edges_to_remove,
        faces_to_remove,
    }
}

pub fn isolate_vertex_and_fill_hole<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    border_loop: Vec<FixedDirectedEdgeHandle>,
    vertex_to_remove: FixedVertexHandle,
) -> IsolateVertexResult
where
    DE: Default,
    UE: Default,
    F: Default,
{
    let vertex = dcel.vertex(vertex_to_remove);

    let mut edges_to_remove = Vec::new();
    let mut faces_to_remove = Vec::new();

    for edge in vertex.out_edges() {
        edges_to_remove.push(edge.as_undirected().fix());
        faces_to_remove.extend(edge.face().fix().as_inner());
    }

    remesh_edge_ring(dcel, border_loop, edges_to_remove, faces_to_remove)
}

pub fn remesh_edge_ring<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    mut border_loop: Vec<FixedDirectedEdgeHandle>,
    edges_to_remove: Vec<FixedUndirectedEdgeHandle>,
    faces_to_remove: Vec<FixedFaceHandle<InnerTag>>,
) -> IsolateVertexResult
where
    DE: Default,
    UE: Default,
    F: Default,
{
    let mut new_edges = Vec::new();
    let smallest_new_edge = FixedUndirectedEdgeHandle::new(dcel.edges.len());

    // Re-mesh border loop by fanning out triangles
    let mut inner_edge = border_loop.pop().unwrap();
    let fan_origin = dcel.directed_edge(inner_edge).from().fix();
    while border_loop.len() > 2 {
        let outer_edge = border_loop.pop().unwrap();
        let outer_edge_from = dcel.directed_edge(outer_edge).from().fix();
        let outer_edge_to = dcel.directed_edge(outer_edge).to().fix();

        let new_edge_handle = FixedDirectedEdgeHandle::new_normalized(dcel.edges.len());
        let new_face_handle = FixedFaceHandle::new(dcel.faces.len());

        // This edge needs to be created
        //      |
        //      |   +
        //      |  /|
        //      | / |
        //      V/  |
        //      /   |
        //    t/   o|
        //    /e    |
        //   /   i  |
        // fo-------+
        //
        // i = inner_edge
        // o = outer_edge
        // fo = fan_origin
        // e = new_edge_handle
        // t = new_twin_handle
        let new_norm = HalfEdgeEntry {
            next: inner_edge,
            prev: outer_edge,
            origin: outer_edge_to,
            face: new_face_handle,
        };
        let new_twin = HalfEdgeEntry {
            next: FixedDirectedEdgeHandle::max(), // Will be
            prev: FixedDirectedEdgeHandle::max(), // fixed in
            face: FixedFaceHandle::max(),         // the next iteration
            origin: fan_origin,
        };

        let new_edge = EdgeEntry::new(new_norm, new_twin);

        let new_face = FaceEntry {
            adjacent_edge: Some(new_edge_handle),
            data: F::default(),
        };

        dcel.half_edge_mut(outer_edge).face = new_face_handle;
        dcel.half_edge_mut(outer_edge).next = new_edge_handle;
        dcel.half_edge_mut(outer_edge).prev = inner_edge;

        // Fix missing stuff from last iteration
        dcel.half_edge_mut(inner_edge).prev = new_edge_handle;
        dcel.half_edge_mut(inner_edge).next = outer_edge;
        dcel.half_edge_mut(inner_edge).face = new_face_handle;

        dcel.vertices[outer_edge_from.index()].out_edge = Some(outer_edge);

        dcel.edges.push(new_edge);
        new_edges.push(new_edge_handle.as_undirected());
        dcel.faces.push(new_face);

        // Prepare next iteration
        inner_edge = new_edge_handle.rev();
    }

    // Fix last triangle
    let inner_edge_next = border_loop.pop().unwrap();
    let inner_edge_prev = border_loop.pop().unwrap();

    // Create a new face for the last triangle
    let new_face_handle = FixedFaceHandle::new(dcel.faces.len());
    let new_face = FaceEntry {
        adjacent_edge: Some(inner_edge),
        data: F::default(),
    };
    dcel.half_edge_mut(inner_edge).face = new_face_handle;
    dcel.faces.push(new_face);
    dcel.half_edge_mut(inner_edge_prev).face = new_face_handle;
    dcel.half_edge_mut(inner_edge_next).face = new_face_handle;

    // Update edge links from last triangle
    dcel.half_edge_mut(inner_edge).prev = inner_edge_prev;
    dcel.half_edge_mut(inner_edge_prev).next = inner_edge;

    dcel.half_edge_mut(inner_edge).next = inner_edge_next;
    dcel.half_edge_mut(inner_edge_next).prev = inner_edge;

    dcel.half_edge_mut(inner_edge_prev).prev = inner_edge_next;
    dcel.half_edge_mut(inner_edge_next).next = inner_edge_prev;

    // Update out_edge entries for the triangle's vertices
    let prev_origin = dcel.half_edge_mut(inner_edge_prev).origin;
    dcel.vertices[prev_origin.index()].out_edge = Some(inner_edge_prev);

    let next_origin = dcel.half_edge_mut(inner_edge_next).origin;
    dcel.vertices[next_origin.index()].out_edge = Some(inner_edge_next);

    // fan_origin is the third vertex of the triangle
    dcel.vertices[fan_origin.index()].out_edge = Some(inner_edge);

    IsolateVertexResult {
        new_edges,
        smallest_new_edge,
        edges_to_remove,
        faces_to_remove,
    }
}

pub fn create_single_face_between_edge_and_next<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    edge: FixedDirectedEdgeHandle,
) -> FixedDirectedEdgeHandle
where
    DE: Default,
    UE: Default,
    F: Default,
{
    // Before:
    //
    //
    //       ^
    //         \
    //  outer   \
    //  face     \
    //            \
    // --- edge -> \
    //
    // After:
    // Added a new edge between edge.next().to() and edge.from()
    //
    // new edge (return value)
    //   |
    //   |   ^
    //   | /   \
    //   v/ new \
    //   / face  \
    //  /         \
    // /-- edge -> \

    let edge_entry = *dcel.half_edge(edge);
    let next_entry = *dcel.half_edge(edge_entry.next);
    let next_to = dcel.directed_edge(edge_entry.next).to().fix();

    let new_face_handle = FixedFaceHandle::<PossiblyOuterTag>::new(dcel.num_faces());

    let inner_edge_entry = HalfEdgeEntry {
        next: edge,
        prev: edge_entry.next,
        face: new_face_handle,
        origin: next_to,
    };

    let outer_edge_entry = HalfEdgeEntry {
        next: next_entry.next,
        prev: edge_entry.prev,
        face: OUTER_FACE_HANDLE,
        origin: edge_entry.origin,
    };

    let new_edge_entry = EdgeEntry {
        entries: [inner_edge_entry, outer_edge_entry],
        directed_data: [DE::default(), DE::default()],
        undirected_data: UE::default(),
    };

    let new_edge_handle = FixedUndirectedEdgeHandle::new(dcel.num_undirected_edges());

    let new_inner_handle = FixedDirectedEdgeHandle::new_normalized(new_edge_handle.index());
    let new_outer_handle = new_inner_handle.rev();

    dcel.half_edge_mut(edge_entry.prev).next = new_outer_handle;
    dcel.half_edge_mut(edge).prev = new_inner_handle;

    dcel.half_edge_mut(edge_entry.next).next = new_inner_handle;
    dcel.half_edge_mut(next_entry.next).prev = new_outer_handle;

    dcel.half_edge_mut(edge).face = new_face_handle;
    dcel.half_edge_mut(edge_entry.next).face = new_face_handle;

    dcel.faces[OUTER_FACE_HANDLE.index()].adjacent_edge = Some(new_outer_handle);

    dcel.edges.push(new_edge_entry);
    dcel.faces.push(FaceEntry {
        adjacent_edge: Some(new_inner_handle),
        data: F::default(),
    });

    new_outer_handle
}

pub fn create_new_face_adjacent_to_edge<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    edge: FixedDirectedEdgeHandle,
    new_vertex: V,
) -> FixedVertexHandle
where
    DE: Default,
    UE: Default,
    F: Default,
{
    // Before:
    //
    // --- edge --->
    //
    // After:
    //
    //  new_prev.rev
    //  |   ^ <---- new
    //  |  / \
    //  | /   \
    //  V/ new \<---new_next.rev
    //  / face  \
    // /         \
    // --- edge -->
    let edge_entry = *dcel.half_edge(edge);

    let edge_from = edge_entry.origin;
    let edge_to = dcel.directed_edge(edge).to().fix();

    let new_next_handle = FixedUndirectedEdgeHandle::new(dcel.num_undirected_edges());
    let new_prev_handle = FixedUndirectedEdgeHandle::new(dcel.num_undirected_edges() + 1);

    let new_face_handle = FixedFaceHandle::<PossiblyOuterTag>::new(dcel.num_faces());
    let new_vertex_handle = FixedVertexHandle::new(dcel.num_vertices());

    let new_next = HalfEdgeEntry {
        next: new_prev_handle.normalized(),
        prev: edge,
        face: new_face_handle,
        origin: edge_to,
    };

    let new_next_rev = HalfEdgeEntry {
        next: edge_entry.next,
        prev: new_prev_handle.not_normalized(),
        face: edge_entry.face,
        origin: new_vertex_handle,
    };

    let new_prev = HalfEdgeEntry {
        next: edge,
        prev: new_next_handle.normalized(),
        face: new_face_handle,
        origin: new_vertex_handle,
    };

    let new_prev_rev = HalfEdgeEntry {
        next: new_next_handle.not_normalized(),
        prev: edge_entry.prev,
        face: edge_entry.face,
        origin: edge_from,
    };

    dcel.edges.push(EdgeEntry {
        entries: [new_next, new_next_rev],
        directed_data: [DE::default(), DE::default()],
        undirected_data: UE::default(),
    });

    dcel.edges.push(EdgeEntry {
        entries: [new_prev, new_prev_rev],
        directed_data: [DE::default(), DE::default()],
        undirected_data: UE::default(),
    });

    dcel.faces.push(FaceEntry {
        adjacent_edge: Some(edge),
        data: F::default(),
    });

    dcel.vertices.push(VertexEntry {
        data: new_vertex,
        out_edge: Some(new_prev_handle.normalized()),
    });

    *dcel.half_edge_mut(edge) = HalfEdgeEntry {
        prev: new_prev_handle.normalized(),
        next: new_next_handle.normalized(),
        face: new_face_handle,
        ..edge_entry
    };

    dcel.faces[edge_entry.face.index()].adjacent_edge = Some(new_prev_handle.not_normalized());

    dcel.half_edge_mut(edge_entry.next).prev = new_next_handle.not_normalized();
    dcel.half_edge_mut(edge_entry.prev).next = new_prev_handle.not_normalized();

    new_vertex_handle
}

pub fn extend_line<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    end_vertex: FixedVertexHandle,
    new_vertex: V,
) -> FixedVertexHandle
where
    DE: Default,
    UE: Default,
{
    // Before:
    //
    // <--out_edge-- end_vertex
    //
    // After:
    // <--out_edge-- end_vertex <---new_edge--- new_vertex

    let end_vertex = dcel.vertex(end_vertex);
    let out_edge = end_vertex.out_edge().expect("end vertex must not isolated");
    let in_edge = out_edge.rev();

    assert_eq!(
        out_edge,
        in_edge.rev(),
        "end vertex is not at the end of the line"
    );

    let new_edge = FixedDirectedEdgeHandle::new_normalized(dcel.num_undirected_edges());
    let new_edge_rev = new_edge.rev();

    let new_vertex_handle = FixedVertexHandle::new(dcel.num_vertices());
    let face = out_edge.face().fix();

    let out_edge = out_edge.fix();
    let in_edge = in_edge.fix();
    let end_vertex = end_vertex.fix();

    dcel.half_edge_mut(out_edge).prev = new_edge;
    dcel.half_edge_mut(in_edge).next = new_edge_rev;

    dcel.edges.push(EdgeEntry::new(
        HalfEdgeEntry {
            next: out_edge,
            prev: new_edge_rev,
            face,
            origin: new_vertex_handle,
        },
        HalfEdgeEntry {
            next: new_edge,
            prev: in_edge,
            face,
            origin: end_vertex,
        },
    ));

    dcel.vertices.push(VertexEntry {
        data: new_vertex,
        out_edge: Some(new_edge),
    });

    new_vertex_handle
}

pub fn split_edge_when_all_vertices_on_line<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    edge: FixedDirectedEdgeHandle,
    new_vertex: V,
) -> ([FixedDirectedEdgeHandle; 2], FixedVertexHandle)
where
    DE: Default,
    UE: Default,
{
    let edge = dcel.directed_edge(edge);
    let rev = edge.rev();
    assert_eq!(edge.face(), rev.face());

    let new_edge = FixedDirectedEdgeHandle::new_normalized(dcel.num_undirected_edges());
    let new_edge_rev = new_edge.rev();

    let edge_next = edge.next().fix();
    let rev_prev = rev.prev().fix();

    let to = edge.to().fix();
    let new_vertex_handle = FixedVertexHandle::new(dcel.vertices.len());

    let face = edge.face().fix();
    let edge = edge.fix();
    let rev = rev.fix();

    let is_isolated = edge_next == rev;

    // Before:
    //
    // from ---edge---> to
    //
    // After:
    //
    // from ---edge---> new_vertex ---new_edge---> to

    dcel.half_edge_mut(edge).next = new_edge;
    dcel.half_edge_mut(rev).prev = new_edge_rev;

    dcel.half_edge_mut(rev).origin = new_vertex_handle;

    dcel.vertices[to.index()].out_edge = Some(new_edge_rev);

    let (new_edge_next, new_rev_prev) = if is_isolated {
        (new_edge_rev, new_edge)
    } else {
        dcel.half_edge_mut(edge_next).prev = new_edge;
        dcel.half_edge_mut(rev_prev).next = new_edge_rev;
        (edge_next, rev_prev)
    };

    dcel.edges.push(EdgeEntry::new(
        HalfEdgeEntry {
            next: new_edge_next,
            prev: edge,
            face,
            origin: new_vertex_handle,
        },
        HalfEdgeEntry {
            next: rev,
            prev: new_rev_prev,
            face,
            origin: to,
        },
    ));

    dcel.vertices.push(VertexEntry {
        data: new_vertex,
        out_edge: Some(new_edge),
    });

    ([edge, new_edge], new_vertex_handle)
}

pub fn split_half_edge<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    edge_handle: FixedDirectedEdgeHandle,
    new_vertex_data: V,
) -> (FixedVertexHandle, [FixedDirectedEdgeHandle; 2])
where
    DE: Default,
    UE: Default,
    F: Default,
{
    // Original face:
    //
    //      to
    //      +
    //      |\
    //      | \
    //      |  \
    //      |   \
    //      |    \
    //      |     \
    //      |     e\
    //      |en     \
    //      |  f1    \
    //      |    ep   \
    //      +----------+
    //      v         from
    //
    // en = edge next
    // ep = edge prev
    //
    // After splitting e0:
    //
    //      to
    //      +
    //      |\
    //      | \
    //      |  \
    //      | e2\
    //      |    \
    //      | nf  nv
    //      |    / \
    //      | e1/   \
    //      |  /    e\
    //      | /  f1   \
    //      +----------+
    //      v         from
    //
    // nf = new face
    // e1, e2 = new edges
    // nv = new vertex
    //
    // This would return [e, e2]
    let edge = dcel.directed_edge(edge_handle);
    let v = edge.prev().from().fix();
    let to = edge.to().fix();
    let edge_next = edge.next().fix();
    let edge_prev = edge.prev().fix();

    let edge_twin = edge.rev().fix();
    let edge_twin_prev = edge.rev().prev().fix();
    let edge_twin_face = edge.rev().face().fix();

    let f1 = edge.face().fix();
    let nf = FixedFaceHandle::new(dcel.faces.len());

    let e1 = FixedDirectedEdgeHandle::new_normalized(dcel.edges.len());
    let t1 = e1.rev();
    let e2 = FixedDirectedEdgeHandle::new_normalized(dcel.edges.len() + 1);
    let t2 = e2.rev();

    let nv = FixedVertexHandle::new(dcel.vertices.len());

    let edge1 = HalfEdgeEntry {
        next: e2,
        prev: edge_next,
        origin: v,
        face: nf,
    };

    let twin1 = HalfEdgeEntry {
        next: edge_prev,
        prev: edge_handle,
        face: f1,
        origin: nv,
    };

    let edge2 = HalfEdgeEntry {
        next: edge_next,
        prev: e1,
        origin: nv,
        face: nf,
    };

    let twin2 = HalfEdgeEntry {
        next: edge_twin,
        prev: edge_twin_prev,
        face: edge_twin_face,
        origin: to,
    };

    let new_face = FaceEntry {
        adjacent_edge: Some(e2),
        data: Default::default(),
    };

    let new_vertex_entry = VertexEntry {
        data: new_vertex_data,
        out_edge: Some(e2),
    };

    dcel.edges.push(EdgeEntry::new(edge1, twin1));
    dcel.edges.push(EdgeEntry::new(edge2, twin2));
    dcel.faces.push(new_face);
    dcel.vertices.push(new_vertex_entry);

    dcel.half_edge_mut(edge_twin_prev).next = t2;

    dcel.half_edge_mut(edge_next).prev = e2;
    dcel.half_edge_mut(edge_prev).prev = t1;
    dcel.half_edge_mut(edge_twin).prev = t2;
    dcel.half_edge_mut(edge_next).next = e1;
    dcel.half_edge_mut(edge_handle).next = t1;

    dcel.half_edge_mut(edge_next).face = nf;
    dcel.half_edge_mut(edge_twin).origin = nv;

    dcel.vertices[to.index()].out_edge = Some(e2.rev());
    dcel.faces[f1.index()].adjacent_edge = Some(edge_handle);

    (nv, [edge_handle, e2])
}

pub fn split_edge<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    edge_handle: FixedDirectedEdgeHandle,
    new_vertex: V,
) -> (FixedVertexHandle, [FixedDirectedEdgeHandle; 2])
where
    DE: Default,
    UE: Default,
    F: Default,
{
    // Original quad:
    //
    //     v1          v4
    //      +----------+
    //      |\   ep    |
    //      | \        |
    //      |  \    f0 |
    //      |   \e0    |
    //      |    \     |
    //      |tn   \  en|
    //      |      \   |
    //      |       \  |
    //      |  f1    \ |
    //      |    tp   \|
    //      +----------+
    //     v2          v3
    //
    // After splitting e0:
    //
    //      +----------+
    //      |\   ep   /|
    //      | \e0    / |
    //      |  \    /  |
    //      |   \  /e3 |
    //      |tn  \/    |
    //      |    /v0   |
    //      | e1/  \ en|
    //      |  /    \  |
    //      | /    e2\ |
    //      |/   tp   \|
    //      +----------+
    //
    // All edges are oriented counter clock wise
    // f0 .. f3 will denote the faces adjacent to e0 .. e3
    // t0 .. t3 will denote the twins of e0 .. e3
    //
    // Splitting edge e0 would return [e0, e2.rev()]
    let edge = dcel.half_edge(edge_handle);
    let twin = dcel.half_edge(edge_handle.rev());

    let f0 = edge.face;
    let f1 = twin.face;
    let f2 = FixedFaceHandle::new(dcel.faces.len());
    let f3 = FixedFaceHandle::new(dcel.faces.len() + 1);

    let e0 = edge_handle;
    let t0 = e0.rev();
    let e1 = FixedDirectedEdgeHandle::new_normalized(dcel.edges.len());
    let t1 = e1.rev();
    let e2 = FixedDirectedEdgeHandle::new_normalized(dcel.edges.len() + 1);
    let t2 = e2.rev();
    let e3 = FixedDirectedEdgeHandle::new_normalized(dcel.edges.len() + 2);
    let t3 = e3.rev();
    let ep = edge.prev;
    let en = edge.next;
    let tn = twin.next;
    let tp = twin.prev;

    let v0 = FixedVertexHandle::new(dcel.vertices.len());
    let v1 = edge.origin;
    let v2 = dcel.half_edge(tp).origin;
    let v3 = twin.origin;
    let v4 = dcel.half_edge(ep).origin;

    let edge0 = HalfEdgeEntry {
        next: t3,
        prev: ep,
        origin: v1,
        face: f0,
    };

    let twin0 = HalfEdgeEntry {
        next: tn,
        prev: e1,
        origin: v0,
        face: f1,
    };

    let edge1 = HalfEdgeEntry {
        next: t0,
        prev: tn,
        origin: v2,
        face: f1,
    };

    let twin1 = HalfEdgeEntry {
        next: tp,
        prev: e2,
        origin: v0,
        face: f2,
    };

    let edge2 = HalfEdgeEntry {
        next: t1,
        prev: tp,
        origin: v3,
        face: f2,
    };

    let twin2 = HalfEdgeEntry {
        next: en,
        prev: e3,
        origin: v0,
        face: f3,
    };

    let edge3 = HalfEdgeEntry {
        next: t2,
        prev: en,
        origin: v4,
        face: f3,
    };

    let twin3 = HalfEdgeEntry {
        next: ep,
        prev: e0,
        origin: v0,
        face: f0,
    };

    let new_vertex_entry = VertexEntry {
        out_edge: Some(t0),
        data: new_vertex,
    };

    let face2 = FaceEntry {
        adjacent_edge: Some(e2),
        data: F::default(),
    };

    let face3 = FaceEntry {
        adjacent_edge: Some(e3),
        data: F::default(),
    };

    *dcel.half_edge_mut(e0) = edge0;
    *dcel.half_edge_mut(t0) = twin0;
    dcel.edges.push(EdgeEntry::new(edge1, twin1));
    dcel.edges.push(EdgeEntry::new(edge2, twin2));
    dcel.edges.push(EdgeEntry::new(edge3, twin3));

    dcel.half_edge_mut(en).next = e3;
    dcel.half_edge_mut(en).prev = t2;
    dcel.half_edge_mut(en).face = f3;

    dcel.half_edge_mut(tp).next = e2;
    dcel.half_edge_mut(tp).prev = t1;
    dcel.half_edge_mut(tp).face = f2;

    dcel.half_edge_mut(tn).next = e1;
    dcel.half_edge_mut(ep).prev = t3;

    dcel.vertices.push(new_vertex_entry);
    dcel.vertices[v3.index()].out_edge = Some(e2);

    dcel.faces[f0.index()].adjacent_edge = Some(e0);
    dcel.faces[f1.index()].adjacent_edge = Some(e1);
    dcel.faces.push(face2);
    dcel.faces.push(face3);

    (v0.adjust_inner_outer(), [e0, e2.rev()])
}

pub fn insert_first_vertex<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    vertex: V,
) -> FixedVertexHandle {
    assert!(dcel.vertices.is_empty());
    dcel.vertices.push(VertexEntry {
        data: vertex,
        out_edge: None,
    });
    FixedVertexHandle::new(0)
}

pub fn insert_second_vertex<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    vertex: V,
) -> FixedVertexHandle
where
    DE: Default,
    UE: Default,
{
    let first_vertex = FixedVertexHandle::new(0);
    let second_vertex = FixedVertexHandle::new(1);
    let normalized = FixedDirectedEdgeHandle::new_normalized(0);
    let not_normalized = normalized.rev();

    dcel.edges.push(EdgeEntry::new(
        HalfEdgeEntry {
            next: not_normalized,
            prev: not_normalized,
            face: OUTER_FACE_HANDLE,
            origin: first_vertex,
        },
        HalfEdgeEntry {
            next: normalized,
            prev: normalized,
            face: OUTER_FACE_HANDLE,
            origin: second_vertex,
        },
    ));

    dcel.vertices.push(VertexEntry {
        data: vertex,
        out_edge: Some(not_normalized),
    });
    dcel.vertices[0].out_edge = Some(normalized);
    dcel.faces[OUTER_FACE_HANDLE.index()].adjacent_edge = Some(normalized);
    second_vertex
}

pub fn insert_into_triangle<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    vertex: V,
    f0: FixedFaceHandle<InnerTag>,
) -> FixedVertexHandle
where
    DE: Default,
    UE: Default,
    F: Default,
{
    // All edges are oriented counterclockwise
    //
    // Original triangle:
    //       v1
    //      / \
    //     /   \
    //    /e1   \
    //   /   f0  \
    //  /       e0\
    // v2__e2_____v0
    //
    // With v inserted (e0, e1 and e2 as above):
    //                 .
    //               / # \
    //             /   #   \
    //       f1  /  e4 # e3  \f0
    //         /     __v__     \
    //       /   e5_#     #__e8  \
    //     / ___#             #___ \
    //   /__#   e6          e7    #__\
    // /#_____________________________#\
    //                f2

    let e0 = dcel.faces[f0.index()].adjacent_edge.expect(
        "Face without adjacent edge should not exist when at least two vertices are present. \
        This is a bug.",
    );

    let e1 = dcel.half_edge(e0).next;
    let e2 = dcel.half_edge(e1).next;
    let e3 = FixedDirectedEdgeHandle::new_normalized(dcel.edges.len());
    let e4 = e3.rev();
    let e5 = FixedDirectedEdgeHandle::new_normalized(dcel.edges.len() + 1);
    let e6 = e5.rev();
    let e7 = FixedDirectedEdgeHandle::new_normalized(dcel.edges.len() + 2);
    let e8 = e7.rev();

    let v = FixedVertexHandle::new(dcel.vertices.len());
    let v0 = dcel.half_edge(e0).origin;
    let v1 = dcel.half_edge(e1).origin;
    let v2 = dcel.half_edge(e2).origin;

    let f1 = FixedFaceHandle::new(dcel.faces.len());
    let f2 = FixedFaceHandle::new(dcel.faces.len() + 1);

    let face1 = FaceEntry {
        adjacent_edge: Some(e1),
        data: F::default(),
    };

    let face2 = FaceEntry {
        adjacent_edge: Some(e2),
        data: F::default(),
    };

    dcel.faces.push(face1);
    dcel.faces.push(face2);

    let vertex = VertexEntry {
        out_edge: Some(e4),
        data: vertex,
    };
    dcel.vertices.push(vertex);

    dcel.half_edge_mut(e0).prev = e8;
    dcel.half_edge_mut(e0).next = e3;
    dcel.half_edge_mut(e1).prev = e4;
    dcel.half_edge_mut(e1).next = e5;
    dcel.half_edge_mut(e1).face = f1;
    dcel.half_edge_mut(e2).prev = e6;
    dcel.half_edge_mut(e2).next = e7;
    dcel.half_edge_mut(e2).face = f2;

    let edge3 = HalfEdgeEntry {
        next: e8,
        prev: e0,
        origin: v1,
        face: f0.adjust_inner_outer(),
    };

    let edge4 = HalfEdgeEntry {
        next: e1,
        prev: e5,
        origin: v,
        face: f1,
    };

    let edge5 = HalfEdgeEntry {
        next: e4,
        prev: e1,
        origin: v2,
        face: f1,
    };

    let edge6 = HalfEdgeEntry {
        next: e2,
        prev: e7,
        origin: v,
        face: f2,
    };

    let edge7 = HalfEdgeEntry {
        next: e6,
        prev: e2,
        origin: v0,
        face: f2,
    };

    let edge8 = HalfEdgeEntry {
        next: e0,
        prev: e3,
        origin: v,
        face: f0.adjust_inner_outer(),
    };

    dcel.edges.push(EdgeEntry::new(edge3, edge4));
    dcel.edges.push(EdgeEntry::new(edge5, edge6));
    dcel.edges.push(EdgeEntry::new(edge7, edge8));

    // Return inserted vertex handle
    v.adjust_inner_outer()
}

pub fn new<V, DE, UE, F>() -> Dcel<V, DE, UE, F>
where
    F: Default,
{
    let outer_face = FaceEntry {
        adjacent_edge: None,
        data: F::default(),
    };

    Dcel {
        vertices: Vec::new(),
        edges: Vec::new(),
        faces: vec![outer_face],
    }
}

pub fn flip_cw<V, DE, UE, F>(dcel: &mut Dcel<V, DE, UE, F>, e: FixedUndirectedEdgeHandle) {
    let e = e.as_directed();
    let e_entry = *dcel.half_edge(e);
    let en = e_entry.next;
    let ep = e_entry.prev;
    let e_face = e_entry.face;
    let e_origin = e_entry.origin;

    let t = e.rev();
    let t_entry = *dcel.half_edge(t);
    let tn = t_entry.next;
    let tp = t_entry.prev;
    let t_face = t_entry.face;
    let t_origin = t_entry.origin;

    dcel.half_edge_mut(en).next = e;
    dcel.half_edge_mut(en).prev = tp;
    dcel.half_edge_mut(e).next = tp;
    dcel.half_edge_mut(e).prev = en;
    dcel.half_edge_mut(e).origin = dcel.half_edge(ep).origin;
    dcel.half_edge_mut(tp).next = en;
    dcel.half_edge_mut(tp).prev = e;
    dcel.half_edge_mut(tp).face = e_face;

    dcel.half_edge_mut(tn).next = t;
    dcel.half_edge_mut(tn).prev = ep;
    dcel.half_edge_mut(t).next = ep;
    dcel.half_edge_mut(t).prev = tn;
    dcel.half_edge_mut(t).origin = dcel.half_edge(tp).origin;
    dcel.half_edge_mut(ep).next = tn;
    dcel.half_edge_mut(ep).prev = t;
    dcel.half_edge_mut(ep).face = t_face;

    dcel.vertices[e_origin.index()].out_edge = Some(tn);
    dcel.vertices[t_origin.index()].out_edge = Some(en);

    dcel.faces[e_face.index()].adjacent_edge = Some(e);
    dcel.faces[t_face.index()].adjacent_edge = Some(t);
}

pub fn cleanup_isolated_vertex<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    isolated: &mut IsolateVertexResult,
) {
    // Remove disconnected edges, faces and the vertex
    isolated.edges_to_remove.sort_unstable();
    for edge in isolated.edges_to_remove.iter().rev() {
        swap_remove_undirected_edge(dcel, *edge);
    }

    isolated.faces_to_remove.sort_unstable();
    for face in isolated.faces_to_remove.iter().rev() {
        swap_remove_face(dcel, *face);
    }
}

fn swap_remove_undirected_edge<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    edge_handle: FixedUndirectedEdgeHandle,
) {
    dcel.edges.swap_remove(edge_handle.index());
    if dcel.num_undirected_edges() > edge_handle.index() {
        let directed = edge_handle.as_directed();
        fix_handle_swap(dcel, directed.rev());
        fix_handle_swap(dcel, directed);
    }
}

fn fix_handle_swap<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    edge_handle: FixedDirectedEdgeHandle,
) {
    // An edge handle was moved to index "edge_handle".
    // Make sure to update all references to this edge
    //
    // Since this method gets only called as part of a swap_remove, the edge that got
    // swapped in always had the index dcel.num_undirected_edges .
    let old_handle = FixedUndirectedEdgeHandle::new(dcel.num_undirected_edges());
    let old_to_new = |handle: FixedDirectedEdgeHandle| {
        let undirected = handle.as_undirected();
        if undirected == old_handle {
            // handle.as_undirected is now edge_handle
            if handle.is_normalized() {
                edge_handle.as_undirected().normalized()
            } else {
                edge_handle.as_undirected().not_normalized()
            }
        } else {
            handle
        }
    };
    let edge_next = dcel.half_edge(edge_handle).next;
    let edge_prev = dcel.half_edge(edge_handle).prev;
    let edge_next = old_to_new(edge_next);
    let edge_prev = old_to_new(edge_prev);
    dcel.half_edge_mut(edge_prev).next = edge_handle;
    dcel.half_edge_mut(edge_next).prev = edge_handle;

    let edge_origin = dcel.half_edge(edge_handle).origin;
    let edge_face = dcel.half_edge(edge_handle).face;
    dcel.vertices[edge_origin.index()].out_edge = Some(edge_handle);
    dcel.faces[edge_face.index()].adjacent_edge = Some(edge_handle);
}

pub fn swap_remove_vertex<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    vertex_handle: FixedVertexHandle,
) -> RemovalResult<V> {
    let data = dcel.vertices.swap_remove(vertex_handle.index()).data;
    let mut swapped_in_vertex = None;
    if dcel.vertices.len() != vertex_handle.index() {
        // Update origin of all out edges of the swapped in vertex
        swapped_in_vertex = Some(FixedVertexHandle::new(dcel.vertices.len()));
        let to_update: SmallVec<[_; 8]> = dcel
            .vertex(vertex_handle)
            .out_edges()
            .map(|handle| handle.fix())
            .collect();
        for e in to_update {
            dcel.half_edge_mut(e).origin = vertex_handle;
        }
    };

    RemovalResult {
        removed_vertex: data,
        swapped_in_vertex,
    }
}

fn swap_remove_face<V, DE, UE, F>(dcel: &mut Dcel<V, DE, UE, F>, face: FixedFaceHandle<InnerTag>) {
    dcel.faces.swap_remove(face.index());
    if dcel.faces.len() > face.index() {
        let neighs: SmallVec<[_; 3]> = dcel
            .face(face)
            .adjacent_edges()
            .iter()
            .map(|edge| edge.fix())
            .collect();
        for n in neighs {
            dcel.half_edge_mut(n).face = face.adjust_inner_outer();
        }
    }
}

pub fn remove_when_degenerate<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    vertex_to_remove: FixedVertexHandle,
) -> RemovalResult<V> {
    match dcel.num_vertices() {
        0 => panic!("Cannot remove vertex when triangulation is empty"),
        1 => remove_when_one_vertex_left(dcel, vertex_to_remove),
        2 => remove_when_two_vertices_left(dcel, vertex_to_remove),
        _ => remove_when_all_vertices_on_line(dcel, vertex_to_remove),
    }
}

fn remove_when_one_vertex_left<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    vertex_to_remove: FixedVertexHandle,
) -> RemovalResult<V> {
    assert_eq!(
        vertex_to_remove.index(),
        0,
        "Attempting to remove invalid vertex"
    );

    RemovalResult {
        removed_vertex: dcel.vertices.pop().unwrap().data,
        swapped_in_vertex: None,
    }
}

fn remove_when_two_vertices_left<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    vertex_to_remove: FixedVertexHandle,
) -> RemovalResult<V> {
    assert_eq!(dcel.num_faces(), 1);
    assert_eq!(dcel.num_vertices(), 2);
    assert_eq!(dcel.num_directed_edges(), 2);

    let swapped_in_vertex = if vertex_to_remove.index() == 1 {
        None
    } else {
        Some(FixedVertexHandle::new(1))
    };

    let result = dcel.vertices.swap_remove(vertex_to_remove.index()).data;
    dcel.faces[OUTER_FACE_HANDLE.index()].adjacent_edge = None;
    dcel.vertices[0].out_edge = None;
    dcel.edges.clear();
    RemovalResult {
        removed_vertex: result,
        swapped_in_vertex,
    }
}

fn remove_when_all_vertices_on_line<V, DE, UE, F>(
    dcel: &mut Dcel<V, DE, UE, F>,
    vertex_to_remove: FixedVertexHandle,
) -> RemovalResult<V> {
    let out_edges: Vec<_> = dcel.vertex(vertex_to_remove).out_edges().collect();
    match &*out_edges {
        [out_edge1] => {
            // Vertex to remove is at one of the ends of the line
            //
            // Topology sketch:
            // vertex_to_remove ---out_edge1---> vertex_to_update ---o_next---->...

            let vertex_to_update = out_edge1.to().fix();
            let o_next = out_edge1.next().fix();
            let out_edge1 = out_edge1.fix();

            dcel.half_edge_mut(o_next).prev = o_next.rev();
            dcel.half_edge_mut(o_next.rev()).next = o_next;
            dcel.vertices[vertex_to_update.index()].out_edge = Some(o_next);
            dcel.faces[OUTER_FACE_HANDLE.index()].adjacent_edge = Some(o_next);

            swap_remove_undirected_edge(dcel, out_edge1.as_undirected());

            swap_remove_vertex(dcel, vertex_to_remove)
        }
        [e1, e2] => {
            // Topology sketch (with v0 = vertex_to_remove, t1 = e1.rev()):
            // v1 ---t1--> v0 ---e2--> v2
            //
            // Should become this:
            // v1 ---------t1--------> v2
            let t1 = e1.rev().fix();
            let e2_next = e2.next().fix();
            let e2_to = e2.to().fix();
            let t2_prev = e2.rev().prev().fix();
            let e2 = e2.fix();

            if e2_next == e2.rev() {
                dcel.half_edge_mut(t1).next = t1.rev();
                dcel.half_edge_mut(t1.rev()).prev = t1;
            } else {
                dcel.half_edge_mut(e2_next).prev = t1;
                dcel.half_edge_mut(t1).next = e2_next;

                dcel.half_edge_mut(t2_prev).next = t1.rev();
                dcel.half_edge_mut(t1.rev()).prev = t2_prev;
            }

            dcel.vertices[e2_to.index()].out_edge = Some(t1.rev());
            dcel.half_edge_mut(t1.rev()).origin = e2_to;

            dcel.faces[OUTER_FACE_HANDLE.index()].adjacent_edge = Some(t1);

            let result = swap_remove_vertex(dcel, vertex_to_remove);
            swap_remove_undirected_edge(dcel, e2.as_undirected());
            result
        }
        _ => panic!("Vertex with invalid out edges found. This is a bug."),
    }
}
