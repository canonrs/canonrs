use leptos::prelude::*;
use super::carousel_ui::*;
use super::carousel_item_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Carousel>
            <CarouselItem>"Slide 1"</CarouselItem>
            <CarouselItem>"Slide 2"</CarouselItem>
            <CarouselItem>"Slide 3"</CarouselItem>
        </Carousel>
    }
}
