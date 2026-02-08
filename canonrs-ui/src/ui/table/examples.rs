use leptos::prelude::*;
use super::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableCaption};

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHead>"Name"</TableHead>
                    <TableHead>"Email"</TableHead>
                    <TableHead>"Role"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell>"Alice"</TableCell>
                    <TableCell>"alice@example.com"</TableCell>
                    <TableCell>"Admin"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Bob"</TableCell>
                    <TableCell>"bob@example.com"</TableCell>
                    <TableCell>"User"</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    }
}

#[component]
pub fn with_caption_example() -> impl IntoView {
    view! {
        <Table>
            <TableCaption>"User List"</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead>"ID"</TableHead>
                    <TableHead>"Name"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell>"1"</TableCell>
                    <TableCell>"Alice"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"2"</TableCell>
                    <TableCell>"Bob"</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    }
}

#[component]
pub fn styled_example() -> impl IntoView {
    view! {
        <Table class="border border-gray-200".to_string()>
            <TableHeader>
                <TableRow>
                    <TableHead class="bg-gray-100".to_string()>"Product"</TableHead>
                    <TableHead class="bg-gray-100".to_string()>"Price"</TableHead>
                    <TableHead class="bg-gray-100".to_string()>"Stock"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell>"Widget"</TableCell>
                    <TableCell>"$10"</TableCell>
                    <TableCell>"50"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Gadget"</TableCell>
                    <TableCell>"$20"</TableCell>
                    <TableCell>"30"</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    }
}
