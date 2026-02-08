use leptos::prelude::*;
use super::card_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>"Card Title"</CardTitle>
                <CardDescription>"Card description goes here"</CardDescription>
            </CardHeader>
            <CardContent>
                <p>"This is the main content of the card."</p>
            </CardContent>
            <CardFooter>
                <p>"Card footer"</p>
            </CardFooter>
        </Card>
    }
}
