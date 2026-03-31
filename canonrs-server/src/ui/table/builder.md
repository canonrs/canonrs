# Table

id: table
label: Table
family: data_display
category: Data
intent: Display tabular data
description: HTML table component
composable: true
capabilities: Selected
required_parts: TableHeader, TableBody, TableRow, TableHead, TableCell
optional_parts: TableFooter, TableCaption, TableWrapper
tags: table, tabular, rows, columns, data
keywords: 
pain: Tables lack consistent state handling and accessibility attributes
promise: Table state, sorting and selection enforced structurally
why: TablePrimitive encodes state, striped and hoverable behavior. Rows and headers derive selection and sort semantics. This guarantees consistent data table behavior.
rules: CR-001, CR-004
use_cases: data grids, reports
related: data_table, virtual_list, empty_table, tree, list_item

## before
// ❌ Typical
view! {
  <table>
    <tr><td>"Data"</td></tr>
  </table>
}

## after
// ✅ CanonRS
view! {
  <Table>
    <TableBody>
      <TableRow>
        <TableCell>"Data"</TableCell>
      </TableRow>
    </TableBody>
  </Table>
}
