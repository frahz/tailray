use std::sync::Arc;

use ksni::Icon;
use resvg::{
    self,
    tiny_skia::{Pixmap, Transform},
    usvg::{fontdb, Options, Tree},
};

const SVG_DATA: &str = include_str!("assets/tailscale.svg");

pub struct Resvg<'a> {
    options: Options<'a>,
    transform: Transform,
}

impl Resvg<'_> {
    pub fn to_icon(&self, svg_str: &str) -> Icon {
        let rtree = Tree::from_str(svg_str, &self.options).unwrap_or_else(|e| {
            panic!("Failed to parse SVG: {e}");
        });

        let size = rtree.size();
        let width = size.width() as u32;
        let height = size.height() as u32;

        let mut pixmap = Pixmap::new(width, height).unwrap();

        resvg::render(&rtree, self.transform, &mut pixmap.as_mut());

        let argb_data: Vec<u8> = pixmap
            .take()
            .chunks(4)
            .flat_map(|rgba| [rgba[3], rgba[0], rgba[1], rgba[2]])
            .collect();

        Icon {
            width: size.width() as i32,
            height: size.height() as i32,
            data: argb_data,
        }
    }

    pub fn load_icon(enabled: bool) -> Vec<Icon> {
        let options = Options {
            fontdb: Arc::new(fontdb::Database::new()),
            ..Default::default()
        };
        let renderer = Self {
            options,
            transform: Transform::default(),
        };

        if enabled {
            log::debug!("icon: Tailscale is enabled");
            vec![renderer.to_icon(SVG_DATA)]
        } else {
            log::debug!("icon: Tailscale is not enabled");
            vec![renderer.to_icon(&SVG_DATA.replace("1.0", "0.4"))]
        }
    }
}
