use bevy::prelude::*;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub enum ViewCubeSegment {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
    TopLeft,
    TopRight,
    TopFront,
    TopBack,
    TopFrontLeft,
    TopFrontRight,
    TopBackLeft,
    TopBackRight,
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight,
    BottomLeft,
    BottomRight,
    BottomFront,
    BottomBack,
    BottomFrontLeft,
    BottomFrontRight,
    BottomBackLeft,
    BottomBackRight,
}

impl ViewCubeSegment {
    pub fn get_rotation(&self) -> Quat {
        match self {
            ViewCubeSegment::Top => Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
            ViewCubeSegment::Bottom => Quat::from_euler(
                EulerRot::XYZ,
                0.0f32.to_radians(),
                0.,
                180.0f32.to_radians(),
            ),
            ViewCubeSegment::Left => Quat::from_euler(
                EulerRot::XYZ,
                0.0f32.to_radians(),
                90.0f32.to_radians(),
                -90.0f32.to_radians(),
            ),
            ViewCubeSegment::Right => Quat::from_euler(
                EulerRot::XYZ,
                0.0f32.to_radians(),
                -90.0f32.to_radians(),
                90.0f32.to_radians(),
            ),
            ViewCubeSegment::Front => Quat::from_euler(
                EulerRot::XYZ,
                -90.0f32.to_radians(),
                0.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
            ViewCubeSegment::Back => Quat::from_euler(
                EulerRot::XYZ,
                90.0f32.to_radians(),
                0.0f32.to_radians(),
                180.0f32.to_radians(),
            ),
            ViewCubeSegment::TopLeft => Quat::from_euler(
                EulerRot::XYZ,
                0.0f32.to_radians(),
                0.0f32.to_radians(),
                -45.0f32.to_radians(),
            ),
            ViewCubeSegment::TopRight => Quat::from_euler(
                EulerRot::XYZ,
                0.0f32.to_radians(),
                0.0f32.to_radians(),
                45.0f32.to_radians(),
            ),
            ViewCubeSegment::TopFront => Quat::from_euler(
                EulerRot::XYZ,
                -45.0f32.to_radians(),
                0.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
            ViewCubeSegment::TopBack => Quat::from_euler(
                EulerRot::XYZ,
                45.0f32.to_radians(),
                0.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
            ViewCubeSegment::TopFrontLeft => Quat::from_euler(
                EulerRot::XYZ,
                -45.0f32.to_radians(),
                45.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
            ViewCubeSegment::TopFrontRight => Quat::from_euler(
                EulerRot::XYZ,
                -45.0f32.to_radians(),
                -45.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
            ViewCubeSegment::TopBackLeft => Quat::from_euler(
                EulerRot::XYZ,
                -45.0f32.to_radians(),
                -135.0f32.to_radians(),
                45.0f32.to_radians(),
            ),
            ViewCubeSegment::TopBackRight => Quat::from_euler(
                EulerRot::XYZ,
                -45.0f32.to_radians(),
                -135.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
            ViewCubeSegment::FrontLeft => Quat::from_euler(
                EulerRot::XYZ,
                -90.0f32.to_radians(),
                45.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
            ViewCubeSegment::FrontRight => Quat::from_euler(
                EulerRot::XYZ,
                -90.0f32.to_radians(),
                -45.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
            ViewCubeSegment::BackLeft => Quat::from_euler(
                EulerRot::XYZ,
                90.0f32.to_radians(),
                45.0f32.to_radians(),
                180.0f32.to_radians(),
            ),
            ViewCubeSegment::BackRight => Quat::from_euler(
                EulerRot::XYZ,
                90.0f32.to_radians(),
                -45.0f32.to_radians(),
                180.0f32.to_radians(),
            ),
            ViewCubeSegment::BottomLeft => Quat::from_euler(
                EulerRot::XYZ,
                0.0f32.to_radians(),
                0.0f32.to_radians(),
                225.0f32.to_radians(),
            ),
            ViewCubeSegment::BottomRight => Quat::from_euler(
                EulerRot::XYZ,
                0.0f32.to_radians(),
                0.0f32.to_radians(),
                135.0f32.to_radians(),
            ),
            ViewCubeSegment::BottomFront => Quat::from_euler(
                EulerRot::XYZ,
                -135.0f32.to_radians(),
                0.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
            ViewCubeSegment::BottomBack => Quat::from_euler(
                EulerRot::XYZ,
                45.0f32.to_radians(),
                0.0f32.to_radians(),
                180.0f32.to_radians(),
            ),
            ViewCubeSegment::BottomFrontLeft => {
                warn!("Not Done");
                Quat::from_euler(
                    EulerRot::XYZ,
                    45.0f32.to_radians(),
                    0.0f32.to_radians(),
                    0.0f32.to_radians(),
                )
            }
            ViewCubeSegment::BottomFrontRight => {
                warn!("Not Done");
                Quat::from_euler(
                    EulerRot::XYZ,
                    45.0f32.to_radians(),
                    0.0f32.to_radians(),
                    0.0f32.to_radians(),
                )
            }
            ViewCubeSegment::BottomBackLeft => {
                warn!("Not Done");
                Quat::from_euler(
                    EulerRot::XYZ,
                    45.0f32.to_radians(),
                    0.0f32.to_radians(),
                    0.0f32.to_radians(),
                )
            }
            ViewCubeSegment::BottomBackRight => {
                warn!("Not Done");
                Quat::from_euler(
                    EulerRot::XYZ,
                    45.0f32.to_radians(),
                    0.0f32.to_radians(),
                    0.0f32.to_radians(),
                )
            }
        }
    }
}
