use bevy::{
    ecs::system::SystemId, input_focus::tab_navigation::TabIndex, prelude::*,
    window::SystemCursorIcon, winit::cursor::CursorIcon,
};
use bevy_core_widgets::{Checked, hover::Hovering};

use bevy_additional_core_widgets::CoreSwitch;

use super::{
    SwitchSize,
    components::{AccessibleName, StyledSwitch, SwitchVariant},
};
use crate::themes::ThemeManager;

#[derive(Default)]
pub struct SwitchBuilder {
    pub variant: SwitchVariant,
    pub state: bool, // ON or OFF by default
    pub on_switch: Option<SystemId<In<(Entity, bool)>>>,
    pub on_color: Option<Color>,
    pub off_color: Option<Color>,
    pub knob_color: Option<Color>,
    pub border_color: Option<Color>,
    pub hover_color: Option<Color>,
    pub text_color: Option<Color>,
    pub on_label: Option<String>,
    pub off_label: Option<String>,
    pub size: Option<SwitchSize>,
    pub disabled: bool, // Disable switch interaction
    pub name: Option<String>,
}

#[derive(Component, Default)]
pub struct RootComponent;

impl SwitchBuilder {
    pub fn variant(mut self, variant: SwitchVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn state(mut self, state: bool) -> Self {
        self.state = state;
        self
    }

    pub fn on_switch(mut self, system_id: SystemId<In<(Entity, bool)>>) -> Self {
        self.on_switch = Some(system_id);
        self
    }

    pub fn on_color(mut self, color: Color) -> Self {
        self.on_color = Some(color);
        self
    }

    pub fn off_color(mut self, color: Color) -> Self {
        self.off_color = Some(color);
        self
    }

    pub fn knob_color(mut self, color: Color) -> Self {
        self.knob_color = Some(color);
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn hover_color(mut self, color: Color) -> Self {
        self.hover_color = Some(color);
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn on_label<S: Into<String>>(mut self, label: S) -> Self {
        self.on_label = Some(label.into());
        self
    }

    pub fn off_label<S: Into<String>>(mut self, label: S) -> Self {
        self.off_label = Some(label.into());
        self
    }

    pub fn size(mut self, size: SwitchSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn build(self) -> impl Bundle {
        let theme_manager = ThemeManager::default();

        let switch_size_styles = theme_manager.styles.switch_sizes.clone();
        //Update size styles
        let switch_size_style = match self.size.unwrap_or_default() {
            SwitchSize::XSmall => switch_size_styles.xsmall,
            SwitchSize::Small => switch_size_styles.small,
            SwitchSize::Medium => switch_size_styles.medium,
            SwitchSize::Large => switch_size_styles.large,
            SwitchSize::XLarge => switch_size_styles.xlarge,
        };

        let is_on = self.state;
        let is_disabled = self.disabled;

        let cursor_icon = if is_disabled {
            SystemCursorIcon::NotAllowed
        } else {
            SystemCursorIcon::Pointer
        };

        let child_nodes = match self.variant {
            SwitchVariant::Rounded => {
                let track_color = if is_disabled {
                    theme_manager.styles.switches.rounded.off_background.with_alpha(10.0)
                } else if is_on {
                    self.on_color
                        .unwrap_or(theme_manager.styles.switches.rounded.on_background)
                } else {
                    self.off_color
                        .unwrap_or(theme_manager.styles.switches.rounded.off_background)
                };
        
                let knob_color = if is_disabled {
                    theme_manager.styles.switches.rounded.knob_color.with_alpha(10.0)
                } else {
                    self.knob_color
                        .unwrap_or(theme_manager.styles.switches.rounded.knob_color)
                };

                Children::spawn((Spawn((
                    Node {
                        width: Val::Px(switch_size_style.track_width),
                        height: Val::Px(switch_size_style.track_height),
                        border: UiRect::all(Val::Px(switch_size_style.track_border_width)),
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(switch_size_style.track_corner_radius)),
                    BackgroundColor(track_color),
                    Children::spawn((
                        Spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Px(switch_size_style.knob_width),
                                height: Val::Px(switch_size_style.knob_height),
                                left: Val::Px(if is_on {
                                    switch_size_style.knob_offset_x_on
                                } else {
                                    switch_size_style.knob_offset_x
                                }),
                                ..default()
                            },
                            BackgroundColor(knob_color),
                            BorderRadius::all(Val::Px(switch_size_style.knob_corner_radius)),
                        )),
                        Spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                ..default()
                            },
                            Text::new(""),
                            TextFont {
                                font_size: switch_size_style.label_font_size,
                                ..default()
                            },
                            TextColor(
                                self.text_color.unwrap_or(
                                    theme_manager
                                        .styles
                                        .switches
                                        .rounded
                                        .text_color,
                                    ),
                                ),
                            )),
                        )),
                    )),
                ))
            },            

            SwitchVariant::RectangularWithText => {
               
                let track_color = if is_disabled {
                    theme_manager.styles.switches.rectangular_with_text.off_background.with_alpha(0.2)
                } else if is_on {
                    self.on_color
                        .unwrap_or(theme_manager.styles.switches.rectangular_with_text.on_background)
                } else {
                    self.off_color
                        .unwrap_or(theme_manager.styles.switches.rectangular_with_text.off_background)
                };
        
                let knob_color = if is_disabled {
                    theme_manager.styles.switches.rectangular_with_text.knob_color.with_alpha(10.0)
                } else {
                    self.knob_color
                        .unwrap_or(theme_manager.styles.switches.rectangular_with_text.knob_color)
                };

                let text = if is_on {
                    self.on_label.clone().unwrap_or("ON".into())
                } else {
                    self.off_label.clone().unwrap_or("OFF".into())
                };

                Children::spawn((Spawn((
                    Node {
                        width: Val::Px(switch_size_style.track_width),
                        height: Val::Px(switch_size_style.track_height),
                        border: UiRect::all(Val::Px(switch_size_style.track_border_width)),
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative, // IMPORTANT for absolute children!
                        ..default()
                    },
                    BorderRadius::all(Val::Px(0.0)),
                    BackgroundColor(track_color), // unwrap Option<Color>
                    Children::spawn((
                        // Knob
                        Spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Px(switch_size_style.knob_width),
                                height: Val::Px(switch_size_style.knob_height),
                                left: Val::Px(if is_on {
                                    switch_size_style.knob_offset_x_on
                                } else {
                                    switch_size_style.knob_offset_x
                                }),
                                ..default()
                            },
                            BackgroundColor(knob_color),
                            BorderRadius::all(Val::Px(0.0)),
                        )),
                        // Label
                        Spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(if is_on {
                                    switch_size_style.label_offset_on
                                } else {
                                    switch_size_style.label_offset
                                }),
                                ..default()
                            },
                            Text::new(text.clone()),
                            TextFont {
                                font_size: switch_size_style.label_font_size,
                                ..default()
                            },
                            TextColor(
                                self.text_color.unwrap_or(
                                    theme_manager
                                        .styles
                                        .switches
                                        .rectangular_with_text
                                        .text_color,
                                    ),
                                ),
                            )),
                        )),
                    )),
                ))
            } 
        };

        (
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            Name::new("Switch"),
            Hovering::default(),
            CursorIcon::System(cursor_icon),
            StyledSwitch {
                variant: self.variant,
                state: self.state,
                on_switch: self.on_switch,
                on_color: self.on_color,
                off_color: self.off_color,
                border_color: self.border_color,
                hover_color: self.hover_color,
                text_color: self.text_color,
                on_label: self.on_label.clone(),
                off_label: self.off_label.clone(),
                size: self.size,
                disabled: self.disabled,
                knob_color: self.knob_color,
            },
            CoreSwitch {
                on_switch: self.on_switch,
            },
            Checked(self.state),
            RootComponent,
            AccessibleName(self.name.clone().unwrap_or_else(|| "Switch".to_string())),
            TabIndex(0),
            child_nodes,
        )
    }
}
