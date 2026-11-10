// Game-side glue for Cloud Glass' eye overlay (upstream `ClientTick.java:24-43`).
// Injected into `crate::game` as `cloudglass_state`.
//
// The eye is inside a cloud-glass block: fill the frame with the dye's diffuse
// colour at alpha 0x80. `gui_layer_pre` runs before each vanilla HUD layer;
// drawing before `minecraft:crosshair` puts the tint under the whole HUD, as
// upstream's overlay sits over the world and under the HUD.

use horizon_client::hud::HudBatch;

use crate::game::cloudglass::tint_for;

/// The layer to draw before: the first one, so every HUD layer stays visible.
const FIRST_LAYER: &str = "minecraft:crosshair";

/// The alpha upstream's overlay uses (`0x80` + the texture diffuse colour).
const ALPHA: u32 = 0x80;


/// `native_gui_layer_pre` (API 139): draw the eye overlay when the camera is
/// inside a Cloud Glass block. `canceled` is left alone: the layer still draws.
#[allow(private_interfaces)]
pub fn on_gui_layer_pre(
    game: &mut crate::game::Game<'_>,
    layer: &str,
    batch: &mut HudBatch,
    _canceled: &mut bool,
) {
    if layer != FIRST_LAYER {
        return;
    }
    let Some(camera) = game.native_camera() else {
        return;
    };
    let at = camera.position.map(|value| value.floor() as i32);
    let Some(rgb) = tint_for(&game.native_block(at)) else {
        return;
    };
    let rect = [0.0, 0.0, batch.width, batch.height];
    game.native_fill(batch, rect, (ALPHA << 24) | rgb);
}
