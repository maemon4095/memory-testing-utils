use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
};

use crate::drop_marker::DropMarker;
use crate::drop_marker::DropMarkerState;

#[derive(Debug)]
pub(crate) struct DropWatcherProps<T> {
    markers: Vec<DropMarkerState<T>>,
}

#[derive(Debug)]
pub struct DropWatcher<T> {
    props: RefCell<DropWatcherProps<T>>,
}

impl<T> DropWatcher<T> {
    pub fn new() -> Self {
        Self {
            props: RefCell::new(DropWatcherProps {
                markers: Vec::new(),
            }),
        }
    }

    pub(crate) fn notify_drop(&self, id: usize) {
        self.props.borrow_mut().markers[id].drop_count += 1;
    }

    pub fn alloc(&self, props: T) -> DropMarker<T> {
        let id = self.props.borrow().markers.len();
        self.props.borrow_mut().markers.push(DropMarkerState {
            drop_count: 0,
            props,
        });
        DropMarker {
            id,
            watcher: Some(self),
        }
    }

    pub fn watch(&self, id: usize) -> Ref<DropMarkerState<T>> {
        Ref::map(self.props.borrow(), |w| &w.markers[id])
    }

    pub fn markers(&self) -> Ref<[DropMarkerState<T>]> {
        Ref::map(self.props.borrow(), |w| w.markers.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::DropWatcher;

    #[test]
    fn notify_drop() {
        let watcher = DropWatcher::new();
        let marker = watcher.alloc(());

        assert!(watcher.markers()[0].drop_count() == 0);
        assert!(watcher.markers()[0].is_leaked());

        watcher.notify_drop(marker.id());
        assert!(watcher.markers()[0].drop_count() == 1);
        assert!(watcher.markers()[0].is_properly_dropped());

        watcher.notify_drop(marker.id());
        assert!(watcher.markers()[0].drop_count() == 2);
        assert!(watcher.markers()[0].is_illegally_dropped());
    }
}
