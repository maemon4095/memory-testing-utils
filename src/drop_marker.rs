use std::{cell::Ref, fmt::Debug};

use crate::drop_watcher::DropWatcher;
pub struct DropMarker<'a, T> {
    pub(crate) id: usize,
    pub(crate) watcher: Option<&'a DropWatcher<T>>,
}

impl<'a, T> Drop for DropMarker<'a, T> {
    fn drop(&mut self) {
        if self.watcher.is_none() {
            panic!("uninitialized dropwatcher is dropped");
        }
        self.watcher.unwrap().notify_drop(self.id);
    }
}

impl<'a, T> DropMarker<'a, T> {
    pub fn props(&self) -> Ref<T> {
        Ref::map(self.watcher.unwrap().watch(self.id), |s| &s.props)
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl<'a, T: PartialEq> PartialEq for DropMarker<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a, T: Debug> Debug for DropMarker<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DropMarker")
            .field("id", &self.id)
            .field("props", &self.props())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct DropMarkerState<T> {
    pub(crate) drop_count: usize,
    pub(crate) props: T,
}
impl<T> DropMarkerState<T> {
    pub fn drop_count(&self) -> usize {
        self.drop_count
    }
    pub fn props(&self) -> &T {
        &self.props
    }

    pub fn is_properly_dropped(&self) -> bool {
        self.drop_count == 1
    }

    pub fn is_illegally_dropped(&self) -> bool {
        self.drop_count > 1
    }

    pub fn is_leaked(&self) -> bool {
        self.drop_count == 0
    }
}
