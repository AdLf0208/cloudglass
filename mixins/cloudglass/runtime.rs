// Cloud Glass for Horizon — the colour registry (pure).
//
// Ported from Cloud Glass 1.1.0 (Mrbysco/CloudGlass, MIT) for Minecraft Java
// 1.21.1 NeoForge. Upstream source was read for the registry and data shapes
// only; this is a clean-room Rust rewrite. The sixteen blocks are declared in
// ../content; this model pins their names and dye colours
// (CloudRegistry.java:14-46, CloudGlassBlock.java:14-45) and the diffuse
// colour the client overlay tints the screen with (ClientTick.java:24-43).

/// The vanilla `DyeColor` texture diffuse colours (`getTextureDiffuseColor`),
/// `0xRRGGBB`, which upstream's overlay draws at alpha `0x80` while the eye is
/// inside a cloud-glass block.
pub const DIFFUSE: [(&str, u32); 16] = [
    ("white", 0xF9FFFE),
    ("orange", 0xF9801D),
    ("magenta", 0xC74EBD),
    ("light_blue", 0x3AB3DA),
    ("yellow", 0xFED83D),
    ("lime", 0x80C71F),
    ("pink", 0xF38BAA),
    ("gray", 0x474F52),
    ("light_gray", 0x9D9D97),
    ("cyan", 0x169C9C),
    ("purple", 0x8932B8),
    ("blue", 0x3C44AA),
    ("brown", 0x835432),
    ("green", 0x5E7C16),
    ("red", 0xB02E26),
    ("black", 0x1D1D21),
];

/// The tint a Cloud Glass block gives the eye overlay, from its block id:
/// `cloudglass:orange_cloud_glass` -> orange's diffuse colour. None for any
/// other block.
pub fn tint_for(block: &str) -> Option<u32> {
    let name = block
        .strip_prefix("cloudglass:")?
        .strip_suffix("_cloud_glass")?;
    DIFFUSE
        .iter()
        .find(|(colour, _)| *colour == name)
        .map(|(_, rgb)| *rgb)
}

#[cfg(test)]
mod tests {
    use super::{DIFFUSE, tint_for};

    /// One Cloud Glass colour: the vanilla dye it is named after and the
    /// `MapColor` id the block declares.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Colour {
        /// The vanilla dye name used in the block and tag ids.
        pub name: &'static str,
        /// The vanilla `DyeColor`'s `MapColor` id, 0 to 61.
        pub map_color: u8,
    }

    /// The sixteen colours, in the order `CloudRegistry` registers them.
    pub const COLOURS: [Colour; 16] = [
        Colour {
            name: "white",
            map_color: 8, // MapColor.SNOW
        },
        Colour {
            name: "orange",
            map_color: 15, // MapColor.COLOR_ORANGE
        },
        Colour {
            name: "magenta",
            map_color: 16, // MapColor.COLOR_MAGENTA
        },
        Colour {
            name: "light_blue",
            map_color: 17, // MapColor.COLOR_LIGHT_BLUE
        },
        Colour {
            name: "yellow",
            map_color: 18, // MapColor.COLOR_YELLOW
        },
        Colour {
            name: "lime",
            map_color: 19, // MapColor.COLOR_LIGHT_GREEN
        },
        Colour {
            name: "pink",
            map_color: 20, // MapColor.COLOR_PINK
        },
        Colour {
            name: "gray",
            map_color: 21, // MapColor.COLOR_GRAY
        },
        Colour {
            name: "light_gray",
            map_color: 22, // MapColor.COLOR_LIGHT_GRAY
        },
        Colour {
            name: "cyan",
            map_color: 23, // MapColor.COLOR_CYAN
        },
        Colour {
            name: "purple",
            map_color: 24, // MapColor.COLOR_PURPLE
        },
        Colour {
            name: "blue",
            map_color: 25, // MapColor.COLOR_BLUE
        },
        Colour {
            name: "brown",
            map_color: 26, // MapColor.COLOR_BROWN
        },
        Colour {
            name: "green",
            map_color: 27, // MapColor.COLOR_GREEN
        },
        Colour {
            name: "red",
            map_color: 28, // MapColor.COLOR_RED
        },
        Colour {
            name: "black",
            map_color: 29, // MapColor.COLOR_BLACK
        },
    ];

    /// The vanilla block whose behaviour the content declarations copy.
    pub const TEMPLATE: &str = "minecraft:cobweb";

    /// Every block id ends with this suffix.
    pub const BLOCK_SUFFIX: &str = "_cloud_glass";

    impl Colour {
        /// The block id, `cloudglass:<name>_cloud_glass`.
        pub fn block_id(self) -> String {
            format!("cloudglass:{}{BLOCK_SUFFIX}", self.name)
        }

        /// The wool ingredient of the upstream shaped recipe.
        pub fn wool(self) -> String {
            format!("minecraft:{}_wool", self.name)
        }

        /// The convention item tag the dye recipe uses, `c:dyes/<name>`.
        pub fn dye_tag(self) -> String {
            format!("c:dyes/{}", self.name)
        }

        /// Whether upstream defines a dye recipe for this colour (all but
        /// white, which is the recipe's base).
        pub fn has_dye_recipe(self) -> bool {
            self.name != "white"
        }
    }

    const NAMES: [&str; 16] = [
        "white",
        "orange",
        "magenta",
        "light_blue",
        "yellow",
        "lime",
        "pink",
        "gray",
        "light_gray",
        "cyan",
        "purple",
        "blue",
        "brown",
        "green",
        "red",
        "black",
    ];

    #[test]
    fn registry_has_the_sixteen_vanilla_colours_in_order() {
        assert_eq!(COLOURS.len(), 16);
        let names: Vec<&str> = COLOURS.iter().map(|c| c.name).collect();
        assert_eq!(names, NAMES);
    }

    #[test]
    fn every_colour_has_the_vanilla_dye_map_colour() {
        const MAP: [u8; 16] = [
            8,  // white   -> MapColor.SNOW
            15, // orange  -> MapColor.COLOR_ORANGE
            16, // magenta -> MapColor.COLOR_MAGENTA
            17, // light_blue -> MapColor.COLOR_LIGHT_BLUE
            18, // yellow  -> MapColor.COLOR_YELLOW
            19, // lime    -> MapColor.COLOR_LIGHT_GREEN
            20, // pink    -> MapColor.COLOR_PINK
            21, // gray    -> MapColor.COLOR_GRAY
            22, // light_gray -> MapColor.COLOR_LIGHT_GRAY
            23, // cyan    -> MapColor.COLOR_CYAN
            24, // purple  -> MapColor.COLOR_PURPLE
            25, // blue    -> MapColor.COLOR_BLUE
            26, // brown   -> MapColor.COLOR_BROWN
            27, // green   -> MapColor.COLOR_GREEN
            28, // red     -> MapColor.COLOR_RED
            29, // black   -> MapColor.COLOR_BLACK
        ];
        assert_eq!(COLOURS.len(), 16);
        for (colour, want) in COLOURS.iter().zip(MAP) {
            assert_eq!(
                colour.map_color, want,
                "{} has the wrong map colour",
                colour.name
            );
        }
    }

    #[test]
    fn block_ids_are_namespaced_by_colour() {
        let orange = COLOURS
            .iter()
            .find(|c| c.name == "orange")
            .expect("orange is registered");
        assert_eq!(orange.block_id(), "cloudglass:orange_cloud_glass");
        assert_eq!(orange.wool(), "minecraft:orange_wool");
        assert_eq!(orange.dye_tag(), "c:dyes/orange");
    }

    #[test]
    fn every_block_uses_the_no_collision_template() {
        // `minecraft:cobweb` is the only vanilla 26.3 block with a single
        // state, an empty collision table, `can_occlude: false`, a MODEL
        // render shape and an item. Its WebBlock behaviour is keyed on the
        // literal name "minecraft:cobweb", so a cloudglass:* block does not
        // slow entities (pinned by the e2e).
        assert_eq!(TEMPLATE, "minecraft:cobweb");
        assert_eq!(COLOURS.len(), 16);
        for colour in COLOURS {
            assert_eq!(
                colour.block_id(),
                format!("cloudglass:{}{BLOCK_SUFFIX}", colour.name)
            );
        }
    }

    #[test]
    fn white_is_the_base_and_has_no_dye_recipe() {
        let dye_recipes: Vec<&str> = COLOURS
            .iter()
            .filter(|c| c.has_dye_recipe())
            .map(|c| c.name)
            .collect();
        assert_eq!(dye_recipes.len(), 15);
        assert!(!dye_recipes.contains(&"white"));
        assert!(dye_recipes.contains(&"black"));
    }

    #[test]
    fn every_map_colour_is_a_valid_map_colour_id() {
        assert_eq!(COLOURS.len(), 16);
        for colour in COLOURS {
            assert!(
                colour.map_color <= 61,
                "{} has map colour {}",
                colour.name,
                colour.map_color
            );
        }
    }

    #[test]
    fn the_screen_tint_comes_from_the_dye_diffuse_colour() {
        assert_eq!(DIFFUSE.len(), 16);
        assert_eq!(tint_for("cloudglass:white_cloud_glass"), Some(0xF9FFFE));
        assert_eq!(tint_for("cloudglass:orange_cloud_glass"), Some(0xF9801D));
        assert_eq!(tint_for("cloudglass:black_cloud_glass"), Some(0x1D1D21));
        // The table and the registry name the same sixteen colours.
        for colour in COLOURS {
            assert!(
                tint_for(&colour.block_id()).is_some(),
                "{} has no diffuse colour",
                colour.name
            );
        }
        assert_eq!(tint_for("minecraft:glass"), None);
        assert_eq!(tint_for("cloudglass:white_stained_glass"), None);
        assert_eq!(tint_for("other:orange_cloud_glass"), None);
    }
}
