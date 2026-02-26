use rs_canonrs::domain::{CanonDocument, CanonNode};
use rs_canonrs::application::{CanonEngine, CanonError, Command};
use crate::ui::layout_builder::application::tree_mapper::flatten_tree;
use crate::ui::layout_builder::domain::node::Node;

#[derive(Clone)]
pub struct BuilderEngine {
    engine: CanonEngine,
    pub flat_cache: Vec<Node>,
}

impl BuilderEngine {
    pub fn new(document: CanonDocument) -> Self {
        let flat_cache = flatten_tree(&document.nodes);
        Self { engine: CanonEngine::new(document), flat_cache }
    }

    pub fn execute_batch(&mut self, commands: Vec<Command>) -> Result<(), CanonError> {
        self.engine.execute_batch(commands)?;
        self.rebuild_cache();
        Ok(())
    }

    pub fn execute(&mut self, cmd: Command) -> Result<(), CanonError> {
        self.engine.execute(cmd)?;
        self.rebuild_cache();
        Ok(())
    }

    pub fn undo(&mut self) -> bool {
        let changed = self.engine.undo();
        if changed { self.rebuild_cache(); }
        changed
    }

    pub fn redo(&mut self) -> bool {
        let changed = self.engine.redo();
        if changed { self.rebuild_cache(); }
        changed
    }

    pub fn can_undo(&self) -> bool { self.engine.can_undo() }
    pub fn can_redo(&self) -> bool { self.engine.can_redo() }

    pub fn document(&self) -> &CanonDocument { &self.engine.document }
    pub fn flat(&self) -> &[Node] { &self.flat_cache }
    pub fn version(&self) -> u32 { self.engine.document.version }
    pub fn load_document(&mut self, doc: CanonDocument) {
        self.engine.load_document(doc);
        self.rebuild_cache();
    }

    pub fn sync_flat(&self) -> Vec<Node> { self.flat_cache.clone() }

    /// Troca o documento preservando o histórico de undo
    pub fn replace_document_preserving_history(&mut self, doc: CanonDocument) {
        let snapshot = self.engine.document.clone();
        self.engine.replace_document(doc);
        self.rebuild_cache();
    }

    fn rebuild_cache(&mut self) {
        self.flat_cache = flatten_tree(&self.engine.document.nodes);
    }
}
