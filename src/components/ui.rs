use bevy::{prelude::*};
use bevy::{color::palettes::basic::*, winit::WinitSettings};

struct ButtonItemData {
    item_name : String,
    quantity: u16,
    image_texture: String,
    is_stackable: bool
}

static MAX_ITEMS : u16 = 10;

const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.1, 0.1, 0.1); // A bluish background color
const NORMAL_BUTTON_BORDER_COLOR: Color = Color::srgb(0.1, 0.5, 0.9); // A lighter bluish border color

pub fn spawn_item_grid(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(0.0)), // Remove outer margin for centering
            ..Default::default() // Use Default::default() here
        },
        ..Default::default() // Use Default::default() here
    }).with_children(|parent| {
        // Middle grid for buttons
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0), // Full width of the parent
                height: Val::Percent(100.0), // Full height of the parent
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::percent(30.0),
                    GridTrack::percent(30.0),
                    GridTrack::percent(30.0),
                ],
                grid_template_rows: vec![
                    GridTrack::auto(), // Auto height to fit the buttons
                    GridTrack::auto(),
                    GridTrack::auto(),
                ],
                margin: UiRect {
                    left: Val::Px(32.0), // Horizontal margin
                    right: Val::Px(32.0),
                    top: Val::Px(0.0), // Optional vertical margin
                    bottom: Val::Px(0.0),
                },
                ..Default::default() // Use Default::default() here
            },
            ..Default::default() // Use Default::default() here
        }).with_children(|grid_parent| {
            for _ in 0..MAX_ITEMS {
                grid_parent.spawn(ButtonBundle {
                    style: Style {
                        width: Val::Auto, // Full width for buttons
                        height: Val::Px(100.0), // Fixed height for buttons
                        margin: UiRect::all(Val::Px(16.0)), // Margin around buttons
                        ..Default::default() // Use Default::default() here
                    },
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    border_color: NORMAL_BUTTON_BORDER_COLOR.into(),
                    border_radius:BorderRadius {
                        bottom_left:Val::Percent(32.0),
                        top_right:Val::Percent(32.0),
                        top_left:Val::Percent(32.0),
                        bottom_right:Val::Percent(32.0),
                    },
                    ..Default::default() // Use Default::default() here
                });
            }
        });
    });
}

