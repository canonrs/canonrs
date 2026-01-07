use super::TreeNode;
use crate::ui::command::{Command, BatchCommand};
use crate::ui::selection::SelectionContext;

/// Exemplo: Bulk delete usando SelectionContext
pub fn bulk_delete_selected(
    selection_ctx: &SelectionContext<String>,
    command_history: &mut crate::ui::command::CommandHistory,
) {
    // 1. Pegar itens selecionados
    let selected = selection_ctx.get_selected_items();
    
    if selected.is_empty() {
        return;
    }
    
    // 2. Criar batch command
    let mut batch = BatchCommand::new(
        format!("Delete {} items", selected.len())
    );
    
    // 3. Adicionar comando para cada item
    for item_id in selected {
        // batch.add(Box::new(DeleteTreeNodeCommand::new(item_id)));
        // TODO: Implementar DeleteTreeNodeCommand
    }
    
    // 4. Executar batch
    // command_history.execute(batch);
}
