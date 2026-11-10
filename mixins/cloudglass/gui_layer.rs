// Runs at the head of `Game::native_gui_layer_pre` (API 139), before each
// vanilla HUD layer: tint the frame when the eye is inside a cloud-glass block.
fn observe_gui_layer_pre(
    game: &mut crate::game::Game<'_>,
    layer: &str,
    batch: &mut horizon_client::hud::HudBatch,
    canceled: &mut bool,
) {
    crate::game::cloudglass_state::on_gui_layer_pre(game, layer, batch, canceled);
}
