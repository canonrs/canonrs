use leptos::prelude::*;
use super::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableCaption};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Table>
            <TableCaption>"User Directory"</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead>"Name"</TableHead>
                    <TableHead>"Email"</TableHead>
                    <TableHead>"Role"</TableHead>
                    <TableHead>"Status"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell>"Alice Johnson"</TableCell>
                    <TableCell>"alice@example.com"</TableCell>
                    <TableCell>"Admin"</TableCell>
                    <TableCell>"Active"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Bob Smith"</TableCell>
                    <TableCell>"bob@example.com"</TableCell>
                    <TableCell>"Developer"</TableCell>
                    <TableCell>"Active"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Carol White"</TableCell>
                    <TableCell>"carol@example.com"</TableCell>
                    <TableCell>"Designer"</TableCell>
                    <TableCell>"Away"</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    }
}
