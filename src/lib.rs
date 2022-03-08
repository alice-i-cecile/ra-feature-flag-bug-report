use bevy_ecs::prelude::{Bundle, Component};

#[derive(Component)]
struct A;

#[derive(Component)]
struct B;

// Error highlighted here
#[derive(Bundle)]
struct ABundle {
    a: A,
    // Produces RA error: "no such field"
    #[cfg(my_feature)]
    b: B,
}
