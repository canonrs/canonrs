use leptos::prelude::*;
use super::{Carousel, CarouselItem};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 3rem;">
            // Basic carousel
            <div>
                <h4>"Basic Carousel"</h4>
                <Carousel id="carousel-1">
                    <CarouselItem>
                        <div style="background: var(--color-primary); color: white; padding: 4rem; text-align: center; border-radius: var(--radius-md);">
                            "Slide 1"
                        </div>
                    </CarouselItem>
                    <CarouselItem>
                        <div style="background: var(--color-success); color: white; padding: 4rem; text-align: center; border-radius: var(--radius-md);">
                            "Slide 2"
                        </div>
                    </CarouselItem>
                    <CarouselItem>
                        <div style="background: var(--color-warning); color: white; padding: 4rem; text-align: center; border-radius: var(--radius-md);">
                            "Slide 3"
                        </div>
                    </CarouselItem>
                </Carousel>
            </div>

            // With autoplay
            <div>
                <h4>"Autoplay Carousel (5s interval)"</h4>
                <Carousel id="carousel-2" autoplay=true interval=5000>
                    <CarouselItem>
                        <div style="background: var(--theme-surface-muted); padding: 3rem; text-align: center; border-radius: var(--radius-md);">
                            "Auto Slide 1"
                        </div>
                    </CarouselItem>
                    <CarouselItem>
                        <div style="background: var(--theme-surface-muted); padding: 3rem; text-align: center; border-radius: var(--radius-md);">
                            "Auto Slide 2"
                        </div>
                    </CarouselItem>
                    <CarouselItem>
                        <div style="background: var(--theme-surface-muted); padding: 3rem; text-align: center; border-radius: var(--radius-md);">
                            "Auto Slide 3"
                        </div>
                    </CarouselItem>
                </Carousel>
            </div>

            // No loop
            <div>
                <h4>"No Loop (stops at edges)"</h4>
                <Carousel id="carousel-3" r#loop=false>
                    <CarouselItem>
                        <div style="background: var(--theme-surface-muted); padding: 2rem; border-radius: var(--radius-md);">
                            "First"
                        </div>
                    </CarouselItem>
                    <CarouselItem>
                        <div style="background: var(--theme-surface-muted); padding: 2rem; border-radius: var(--radius-md);">
                            "Middle"
                        </div>
                    </CarouselItem>
                    <CarouselItem>
                        <div style="background: var(--theme-surface-muted); padding: 2rem; border-radius: var(--radius-md);">
                            "Last"
                        </div>
                    </CarouselItem>
                </Carousel>
            </div>
        </div>
    }
}
