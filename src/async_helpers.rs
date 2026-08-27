#[cfg(feature = "stats")]
use std::time::Instant;
use std::{collections::HashSet, fmt, future::Future, task::Poll};

use glam::Vec2;

use crate::{
    instance::{InstanceStep, SearchInstance, U32Layer},
    Mesh, Path,
};

/// A future that will resolve to a [`Option<Path>`].
///
/// This will be a [`Path`] if a path is found, or `None` if not. Returned by [`Mesh::get_path`].
pub struct FuturePath<'m> {
    pub(crate) from: Vec2,
    pub(crate) to: Vec2,
    pub(crate) mesh: &'m Mesh,
    pub(crate) instance: Option<SearchInstance<'m>>,
    pub(crate) ending_polygon: u32,
}

impl fmt::Debug for FuturePath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FuturePath")
            .field("from", &self.from)
            .field("to", &self.to)
            .finish()
    }
}

impl Future for FuturePath<'_> {
    type Output = Option<Path>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        if let Some(search_instance) = self.instance.as_mut() {
            for _i in 0..3 {
                match search_instance.next() {
                    InstanceStep::Found(path) => return Poll::Ready(Some(path)),
                    InstanceStep::NotFound => return Poll::Ready(None),
                    InstanceStep::Continue => {}
                }
            }
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            #[cfg(feature = "stats")]
            let start = Instant::now();

            // A point over overlapping polygons has more than one reading, and every one of
            // them is searched: see `Mesh::path_on_layers`.
            let starting_polygons = self
                .mesh
                .candidate_polygons(self.from.into(), &HashSet::default());
            if starting_polygons.is_empty() {
                return Poll::Ready(None);
            }
            let ending_polygons = self
                .mesh
                .candidate_polygons(self.to.into(), &HashSet::default());
            if ending_polygons.is_empty() {
                return Poll::Ready(None);
            }

            if self.mesh.layers.len() == 1 {
                if let Some(islands) = self.mesh.layers[0].islands.as_ref() {
                    let connected = starting_polygons.iter().any(|starting_polygon| {
                        ending_polygons.iter().any(|ending_polygon| {
                            let start_island = islands.get(starting_polygon.polygon() as usize);
                            let end_island = islands.get(ending_polygon.polygon() as usize);
                            start_island.is_none()
                                || end_island.is_none()
                                || start_island == end_island
                        })
                    });
                    if !connected {
                        return Poll::Ready(None);
                    }
                }
            }

            if let Some(ending_polygon) = starting_polygons
                .iter()
                .find(|starting_polygon| ending_polygons.contains(starting_polygon))
            {
                #[cfg(feature = "stats")]
                {
                    if self.mesh.scenarios.get() == 0 {
                        eprintln!(
                        "index;micros;successor_calls;generated;pushed;popped;pruned_post_pop;length",
                    );
                    }
                    eprintln!(
                        "{};{};0;0;0;0;0;{}",
                        self.mesh.scenarios.get(),
                        start.elapsed().as_secs_f32() * 1_000_000.0,
                        self.from.distance(self.to),
                    );
                    self.mesh.scenarios.set(self.mesh.scenarios.get() + 1);
                }
                return Poll::Ready(Some(Path {
                    length: self.from.distance(self.to),
                    path: vec![self.to],
                    #[cfg(feature = "detailed-layers")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "detailed-layers")))]
                    path_with_layers: vec![(self.to, ending_polygon.layer())],
                    path_through_polygons: vec![*ending_polygon],
                }));
            }

            self.ending_polygon = ending_polygons[0];
            self.instance = Some(SearchInstance::setup(
                self.mesh,
                (self.from, &starting_polygons),
                (self.to, &ending_polygons),
                HashSet::default(),
                #[cfg(feature = "stats")]
                start,
            ));
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
