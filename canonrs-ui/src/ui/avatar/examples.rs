use leptos::prelude::*;
use super::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarShape, AvatarStatus};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 3rem;">
            // Sizes
            <div>
                <h4>"Sizes"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Avatar size=AvatarSize::Xs>
                        <AvatarFallback>"XS"</AvatarFallback>
                    </Avatar>
                    <Avatar size=AvatarSize::Sm>
                        <AvatarFallback>"SM"</AvatarFallback>
                    </Avatar>
                    <Avatar size=AvatarSize::Md>
                        <AvatarFallback>"MD"</AvatarFallback>
                    </Avatar>
                    <Avatar size=AvatarSize::Lg>
                        <AvatarFallback>"LG"</AvatarFallback>
                    </Avatar>
                    <Avatar size=AvatarSize::Xl>
                        <AvatarFallback>"XL"</AvatarFallback>
                    </Avatar>
                </div>
            </div>

            // Shapes
            <div>
                <h4>"Shapes"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Avatar shape=AvatarShape::Circle>
                        <AvatarFallback>"CR"</AvatarFallback>
                    </Avatar>
                    <Avatar shape=AvatarShape::Square>
                        <AvatarFallback>"SQ"</AvatarFallback>
                    </Avatar>
                    <Avatar shape=AvatarShape::Rounded>
                        <AvatarFallback>"RD"</AvatarFallback>
                    </Avatar>
                </div>
            </div>

            // Status
            <div>
                <h4>"Status Indicators"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Avatar status=AvatarStatus::Online>
                        <AvatarImage 
                            src="https://i.pravatar.cc/150?img=1".to_string()
                            alt="Online user".to_string()
                        />
                        <AvatarFallback>"ON"</AvatarFallback>
                    </Avatar>
                    <Avatar status=AvatarStatus::Busy>
                        <AvatarImage 
                            src="https://i.pravatar.cc/150?img=2".to_string()
                            alt="Busy user".to_string()
                        />
                        <AvatarFallback>"BS"</AvatarFallback>
                    </Avatar>
                    <Avatar status=AvatarStatus::Away>
                        <AvatarImage 
                            src="https://i.pravatar.cc/150?img=3".to_string()
                            alt="Away user".to_string()
                        />
                        <AvatarFallback>"AW"</AvatarFallback>
                    </Avatar>
                    <Avatar status=AvatarStatus::Offline>
                        <AvatarImage 
                            src="https://i.pravatar.cc/150?img=4".to_string()
                            alt="Offline user".to_string()
                        />
                        <AvatarFallback>"OF"</AvatarFallback>
                    </Avatar>
                </div>
            </div>

            // Badges
            <div>
                <h4>"Notification Badges"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Avatar badge=3>
                        <AvatarFallback>"JD"</AvatarFallback>
                    </Avatar>
                    <Avatar badge=99>
                        <AvatarFallback>"AB"</AvatarFallback>
                    </Avatar>
                    <Avatar size=AvatarSize::Lg badge=5>
                        <AvatarImage 
                            src="https://i.pravatar.cc/150?img=5".to_string()
                            alt="User with badge".to_string()
                        />
                        <AvatarFallback>"CD"</AvatarFallback>
                    </Avatar>
                </div>
            </div>

            // Combined
            <div>
                <h4>"Combined Features"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Avatar 
                        size=AvatarSize::Lg 
                        shape=AvatarShape::Rounded 
                        status=AvatarStatus::Online 
                        badge=12
                    >
                        <AvatarImage 
                            src="https://i.pravatar.cc/150?img=6".to_string()
                            alt="User".to_string()
                        />
                        <AvatarFallback>"EF"</AvatarFallback>
                    </Avatar>
                    <Avatar 
                        size=AvatarSize::Xl 
                        shape=AvatarShape::Square 
                        status=AvatarStatus::Busy
                    >
                        <AvatarFallback>"GH"</AvatarFallback>
                    </Avatar>
                </div>
            </div>

            // Fallback only
            <div>
                <h4>"Fallback (Initials)"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Avatar size=AvatarSize::Sm>
                        <AvatarFallback>"JD"</AvatarFallback>
                    </Avatar>
                    <Avatar>
                        <AvatarFallback>"AB"</AvatarFallback>
                    </Avatar>
                    <Avatar size=AvatarSize::Lg>
                        <AvatarFallback>"CD"</AvatarFallback>
                    </Avatar>
                </div>
            </div>
        </div>
    }
}
