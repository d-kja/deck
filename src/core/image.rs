use std::sync::Once;

use magick_rust::{Image, MagickError, MagickWand, magick_wand_genesis, magick_wand_terminus};
use tracing::info;

static START: Once = Once::new();

pub struct DeckImage {
    columns: usize,
    rows: usize,
    gap: usize,
}

impl Default for DeckImage {
    fn default() -> Self {
        Self {
            gap: 40,
            rows: 3,
            columns: 6,
        }
    }
}

impl DeckImage {
    pub fn new() -> Self {
        START.call_once(|| {
            magick_wand_genesis();
        });

        Self::default()
    }

    pub fn crop_grid<'a>(&self, source: &str) -> Result<Vec<Image<'a>>, MagickError> {
        let probe = MagickWand::new();
        probe.read_image(source)?;

        let (width, height) = (probe.get_image_width(), probe.get_image_height());
        let (tile_width, tile_height) = (
            (width - self.gap * (self.columns - 1)) / self.columns,
            (height - self.gap * (self.rows - 1)) / self.rows,
        );

        let mut idx = 0;
        let mut images: Vec<Image<'a>> = vec![];
        for row in 0..self.rows {
            for col in 0..self.columns {
                let (x, y) = (
                    (col * (tile_width + self.gap)) as isize,
                    (row * (tile_height + self.gap)) as isize,
                );

                let tile = MagickWand::new();

                tile.read_image(source)?;
                tile.crop_image(tile_width, tile_height, x, y);
                tile.write_image_blob(&format!("assets/icons/tiles/tile_{}.png", idx));

                info!("Cropped image {:?}", idx);

                // let image: Image<'a> = tile.get_image()?;
                idx += 1;
            }
        }

        Ok(images)
    }

    pub fn shutdown(&self) {
        START.call_once(|| {
            magick_wand_terminus();
        });
    }
}
