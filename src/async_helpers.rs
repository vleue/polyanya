#[cfg(feature = "stats")]
use std::time::Instant;
use std::{fmt, future::Future, task::Poll};

use glam::Vec2;

use crate::{
    instance::{InstanceStep, SearchInstance},
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
    pub(crate) ending_polygon: isize,
}

impl<'m> fmt::Debug for FuturePath<'m> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FuturePath")
            .field("from", &self.from)
            .field("to", &self.to)
            .finish()
    }
}

impl<'m> Future for FuturePath<'m> {
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

            let starting_polygon_index = self.mesh.get_point_location(self.from);
            if starting_polygon_index == u32::MAX {
                return Poll::Ready(None);
            }
            let ending_polygon = self.mesh.get_point_location(self.to);
            if ending_polygon == u32::MAX {
                return Poll::Ready(None);
            }
            if let Some(islands) = self.mesh.islands.as_ref() {
                let start_island = islands.get(starting_polygon_index as usize);
                let end_island = islands.get(ending_polygon as usize);
                if start_island.is_some() && end_island.is_some() && start_island != end_island {
                    return Poll::Ready(None);
                }
            }

            if starting_polygon_index == ending_polygon {
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
                }));
            }

            self.instance = Some(SearchInstance::setup(
                self.mesh,
                (self.from, starting_polygon_index),
                (self.to, ending_polygon),
                #[cfg(feature = "stats")]
                start,
            ));
            self.ending_polygon = ending_polygon as isize;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
