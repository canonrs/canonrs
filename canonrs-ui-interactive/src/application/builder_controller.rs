use leptos::prelude::*;
use uuid::Uuid;
use rs_canonrs::application::Command;
use crate::ui::layout_builder::domain::node::Node;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;

#[derive(Clone, Copy)]
pub struct BuilderController {
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    selected_id: RwSignal<Option<Uuid>>,
    is_dirty: RwSignal<bool>,
}

impl BuilderController {
    pub fn new(
        engine: RwSignal<BuilderEngine>,
        tree: RwSignal<Vec<Node>>,
        selected_id: RwSignal<Option<Uuid>>,
        is_dirty: RwSignal<bool>,
    ) -> Self {
        Self { engine, tree, selected_id, is_dirty }
    }

    fn sync(&self) {
        let flat = self.engine.get_untracked().sync_flat();
        self.tree.set(flat);
        self.is_dirty.set(true);
    }

    pub fn undo(&self) {
        self.engine.update(|e| { e.undo(); });
        self.sync();
    }

    pub fn redo(&self) {
        self.engine.update(|e| { e.redo(); });
        self.sync();
    }

    pub fn execute(&self, cmd: Command) {
        self.engine.update(|e| { let _ = e.execute(cmd); });
        self.sync();
    }

    pub fn delete_selected(&self) {
        if let Some(id) = self.selected_id.get_untracked() {
            self.execute(Command::Remove { node_id: id });
            self.selected_id.set(None);
        }
    }

    pub fn load_document(&self, doc: rs_canonrs::domain::CanonDocument) {
        self.engine.update(|e| { e.load_document(doc.clone()); });
        self.tree.set(self.engine.get_untracked().sync_flat());
        self.is_dirty.set(false);
    }

    pub fn has_document(&self) -> bool {
        !self.tree.get().is_empty()
    }

    pub fn can_undo(&self) -> bool { self.engine.get_untracked().can_undo() }
    pub fn can_redo(&self) -> bool { self.engine.get_untracked().can_redo() }
}
