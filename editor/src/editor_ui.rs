use bevy::{
    camera::{Viewport, visibility::RenderLayers},
    window::PrimaryWindow,
};
use bevy_egui::{EguiContext, EguiContextSettings, EguiGlobalSettings, EguiPrimaryContextPass};
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_inspector::{ui_for_entity_with_children, ui_for_world},
};
use egui::LayerId;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use crate::{and_exit, prelude::*, save};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(bevy_egui::EguiPlugin::default());
    app.add_plugins(DefaultInspectorConfigPlugin);

    app.insert_resource(UiState::new());

    app.add_systems(OnEnter(State::Editor), setup);
    app.add_systems(
        EguiPrimaryContextPass,
        show_ui_system.run_if(in_state(State::Editor)),
    );
    app.add_systems(
        PostUpdate,
        set_camera_viewport
            .after(show_ui_system)
            .run_if(in_state(State::Editor)),
    );
}

fn setup(mut commands: Commands, mut egui_global_settings: ResMut<EguiGlobalSettings>) {
    egui_global_settings.auto_create_primary_context = false;

    // egui camera
    commands.spawn((
        Name::new("Egui Camera"),
        Camera2d,
        Pickable::IGNORE,
        PrimaryEguiContext,
        RenderLayers::none(),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));
}

fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}

// Make camera only render to view not obstructed by UI
fn set_camera_viewport(
    ui_state: Res<UiState>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut cam: Single<&mut Camera, Without<PrimaryEguiContext>>,
    egui_settings: Single<&EguiContextSettings>,
) {
    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor;

    let physical_position = UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32);
    let physical_size = UVec2::new(viewport_size.x as u32, viewport_size.y as u32);

    let rect = physical_position + physical_size;

    let window_size = window.physical_size();
    if rect.x <= window_size.x && rect.y <= window_size.y {
        cam.viewport = Some(Viewport {
            physical_position,
            physical_size,
            depth: 0.0..1.0,
        });
    }
}

#[derive(Resource)]
struct UiState {
    state: DockState<EguiWindow>,
    viewport_rect: egui::Rect,
    pointer_in_viewport: bool,
}

impl UiState {
    pub fn new() -> Self {
        let mut state = DockState::new(vec![EguiWindow::GameView]);
        let tree = state.main_surface_mut();
        let [_game, sidebar_menu] = tree.split_left(
            NodeIndex::root(),
            0.3,
            vec![
                EguiWindow::SidebarMenu,
                EguiWindow::Towers,
                EguiWindow::Projectiles,
                EguiWindow::Enemies,
                EguiWindow::WorldInspector,
            ],
        );
        let [_sidebar_menu, _options] =
            tree.split_below(sidebar_menu, 0.9, vec![EguiWindow::Options]);

        Self {
            state,
            viewport_rect: egui::Rect::NOTHING,
            pointer_in_viewport: false,
        }
    }

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            pointer_in_viewport: &mut self.pointer_in_viewport,
        };
        DockArea::new(&mut self.state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }
}

#[derive(Debug)]
enum EguiWindow {
    GameView,
    SidebarMenu,
    Towers,
    Projectiles,
    Enemies,
    WorldInspector,
    Options,
}

struct TabViewer<'a> {
    world: &'a mut World,
    viewport_rect: &'a mut egui::Rect,
    pointer_in_viewport: &'a mut bool,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        match window {
            EguiWindow::GameView => *self.viewport_rect = ui.clip_rect(),
            EguiWindow::SidebarMenu => {
                ui.vertical(|ui| {
                    ui.label("Selected entities:");
                    let focused = self.world.resource::<FocusedEntities>();
                    for entity in focused.entities.clone() {
                        let name = self
                            .world
                            .get::<Name>(entity)
                            .map(|n| n.as_str())
                            .unwrap_or_default();
                        ui.collapsing(format!("{name} ({entity})"), |ui| {
                            ui_for_entity_with_children(self.world, entity, ui);
                        });
                    }

                    ui.separator();

                    ui.label("Paths");
                    crate::map::path_edit_ui(self.world, ui);

                    ui.separator();
                    ui.add_space(50.);
                });
            }
            EguiWindow::Towers => {
                crate::tower::tower_tab_ui(self.world, ui);
            }
            EguiWindow::Projectiles => {
                crate::projectile::projectile_tab_ui(self.world, ui);
            }
            EguiWindow::Enemies => {
                crate::enemy::enemy_tab_ui(self.world, ui);
            }
            EguiWindow::WorldInspector => {
                ui_for_world(self.world, ui);
            }
            EguiWindow::Options => {
                ui.vertical(|ui| {
                    if ui.button("Save").clicked() {
                        save(self.world);
                    }
                    if ui.button("Quit editor without saving").clicked() {
                        self.world.write_message(AppExit::Success);
                    }
                    if ui.button("Close editor after saving").clicked() {
                        save(self.world);
                        and_exit(self.world);
                    }
                });
            }
        }

        *self.pointer_in_viewport = ui
            .ctx()
            .rect_contains_pointer(LayerId::background(), self.viewport_rect.shrink(16.));
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::GameView)
    }
}
