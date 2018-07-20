#![windows_subsystem = "windows"]

extern crate azul;

use azul::prelude::*;
use azul::widgets::*;
use azul::dialogs::*;
use std::fs;

#[derive(Debug)]
pub struct MyAppData {
    pub map: Option<Map>,
}

// need: (Style, VertexBuffer<SvgVert>, IndexBuffer<u32>)
// TODO: This will be slow at first if we don't cache this

#[derive(Debug)]
pub struct Map {
    pub cache: SvgCache<MyAppData>,
    pub layers: Vec<SvgLayerId>,
    pub font_cache: VectorizedFontCache,
    pub zoom: f64,
    pub pan_horz: f64,
    pub pan_vert: f64,
}

impl Layout for MyAppData {
    fn layout(&self, info: WindowInfo)
    -> Dom<MyAppData>
    {
        if let Some(map) = &self.map {
            Svg::with_layers(build_layers(&map.layers, &map.font_cache, &info.resources))
                .with_pan(map.pan_horz as f32, map.pan_vert as f32)
                .with_zoom(map.zoom as f32)
                .dom(&info.window, &map.cache)
                .with_callback(On::Scroll, Callback(scroll_map_contents))
        } else {
            // TODO: If this is changed to Label::new(), the text is cut off at the top
            // because of the (offset_top / 2.0) - see text_layout.rs file
            Button::with_label("Open SVG file...").dom()
               .with_callback(On::LeftMouseUp, Callback(my_button_click_handler))
        }
    }
}

const FONT_ID: FontId = FontId::BuiltinFont("sans-serif");

fn build_layers(existing_layers: &[SvgLayerId], vector_font_cache: &VectorizedFontCache, resources: &AppResources)
-> Vec<SvgLayerResource>
{
    let mut layers: Vec<SvgLayerResource> = existing_layers.iter().map(|e| SvgLayerResource::Reference(*e)).collect();

    // layout the texts
    use azul::text_layout::*;

    let cur_string = "HelloWorld";

    let font = resources.get_font(&FONT_ID).unwrap();
    let vectorized_font = vector_font_cache.get_font(&FONT_ID).unwrap();

    // let font_metrics = FontMetrics::new(&font.0, &FontSize::px(10.0), None);
    // let layout = layout_text(&cur_string, &font.0, &font_metrics);

    let style = SvgStyle::filled(ColorU { r: 0, g: 0, b: 0, a: 255 });
    let (fill, stroke) = {

        let mut fill = None;
        let mut stroke = None;

        // TODO: calculate the offset of each glyph vertex and apply it here
        // NOTE: cant use spaces here, this will panic!

        if style.fill.is_some() {
            let mut fill_buf = Vec::new();
            for gid in cur_string.chars().map(|g| font.0.glyph(g).id()) {
                if let Some(s) = vectorized_font.get_fill_vertices(&gid){ fill_buf.push(s); }
            }
            fill = Some(join_vertex_buffers(&fill_buf));
        }

        if style.stroke.is_some() {
            let mut stroke_buf = Vec::new();
            for gid in cur_string.chars().map(|g| font.0.glyph(g).id()) {
                if let Some(s) = vectorized_font.get_stroke_vertices(&gid){ stroke_buf.push(s); }
            }
            stroke = Some(join_vertex_buffers(&stroke_buf));
        }

        (fill.and_then(|s| Some(VerticesIndicesBuffer { vertices: s.0, indices: s.1 })),
         stroke.and_then(|s| Some(VerticesIndicesBuffer { vertices: s.0, indices: s.1 })))
    };

    layers.push(SvgLayerResource::Direct { style, fill, stroke });

    layers
}

fn scroll_map_contents(app_state: &mut AppState<MyAppData>, event: WindowEvent) -> UpdateScreen {
    app_state.data.modify(|data| {
        if let Some(map) = data.map.as_mut() {

            let mouse_state = app_state.windows[event.window].get_mouse_state();
            let keyboard_state = app_state.windows[event.window].get_keyboard_state();

            if keyboard_state.shift_down {
                map.pan_horz += mouse_state.scroll_y;
            } else if keyboard_state.ctrl_down {
                if mouse_state.scroll_y.is_sign_positive() {
                    map.zoom /= 2.0;
                } else {
                    map.zoom *= 2.0;
                }
            } else {
                map.pan_vert += mouse_state.scroll_y;
            }
        }
    });

    UpdateScreen::Redraw
}

fn my_button_click_handler(app_state: &mut AppState<MyAppData>, _event: WindowEvent) -> UpdateScreen {
    open_file_dialog(None, None)
        .and_then(|path| fs::read_to_string(path.clone()).ok())
        .and_then(|contents| {

            let mut svg_cache = SvgCache::empty();
            let svg_layers = svg_cache.add_svg(&contents).ok()?;

            // Pre-vectorize the glyphs of the font into vertex buffers
            let (font, _) = app_state.get_font(&FONT_ID)?;
            let mut vectorized_font_cache = VectorizedFontCache::new();
            vectorized_font_cache.insert_if_not_exist(FONT_ID, font);

            app_state.data.modify(|data| data.map = Some(Map {
                cache: svg_cache,
                font_cache: vectorized_font_cache,
                layers: svg_layers,
                zoom: 1.0,
                pan_horz: 0.0,
                pan_vert: 0.0,
            }));

            Some(UpdateScreen::Redraw)
        })
        .unwrap_or_else(|| {
            UpdateScreen::DontRedraw
        })
}

fn main() {
    let mut app = App::new(MyAppData { map: None }, AppConfig::default());
    app.create_window(WindowCreateOptions::default(), Css::native()).unwrap();
    app.run().unwrap();
}