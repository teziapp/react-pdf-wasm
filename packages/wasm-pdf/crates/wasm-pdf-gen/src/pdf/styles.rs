#![allow(dead_code)]

use super::json::{get_bool_from_js, get_number_from_js, JsContent, JsParamValue};

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

impl HorizontalAlign {
    pub fn from_content(content: &JsContent) -> HorizontalAlign {
        if let Some(text_align) = content.params.get("align") {
            if let JsParamValue::Text(text_align) = text_align {
                match text_align.as_str() {
                    "right" => HorizontalAlign::Right,
                    "center" => HorizontalAlign::Center,
                    _ => HorizontalAlign::Left,
                }
            } else {
                HorizontalAlign::Left
            }
        } else {
            HorizontalAlign::Left
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TableStyle {
    pub grid_visible: bool,
    pub grid_width: f32,
    pub grid_color: Color,
    pub padding_top: f32,
    pub padding_left: f32,
    pub padding_bottom: f32,
    pub padding_right: f32,
    pub vertical_align: VerticalAlign,
}

impl TableStyle {
    pub fn new() -> TableStyle {
        TableStyle {
            grid_visible: false,
            grid_width: 1.0,
            grid_color: Color::new(0.0, 0.0, 0.0),
            padding_top: 0.0,
            padding_left: 0.0,
            padding_bottom: 0.0,
            padding_right: 0.0,
            vertical_align: VerticalAlign::Top,
        }
    }
    pub fn from_content(content: &JsContent) -> TableStyle {
        let mut table_style = TableStyle::new();
        if let Some(style) = content.params.get("style") {
            if let JsParamValue::Object(style) = style {
                //json_out(&JsValue::from_serde(style).unwrap());
                if let Some(grid) = style.get("grid") {
                    TableStyle::get_grid(&mut table_style, grid);
                }
                if let Some(padding) = style.get("padding") {
                    TableStyle::get_table_padding(&mut table_style, padding);
                }
                if let Some(align) = style.get("align") {
                    TableStyle::get_align(&mut table_style, align);
                }
            }
        }
        table_style
    }
    fn get_table_padding(table_style: &mut TableStyle, padding: &JsParamValue) {
        if let JsParamValue::Object(padding) = padding {
            if let Some(top) = padding.get("top") {
                if let JsParamValue::Number(top) = top {
                    table_style.padding_top = *top;
                }
            }
            if let Some(left) = padding.get("left") {
                if let JsParamValue::Number(left) = left {
                    table_style.padding_left = *left;
                }
            }
            if let Some(bottom) = padding.get("bottom") {
                if let JsParamValue::Number(bottom) = bottom {
                    table_style.padding_bottom = *bottom;
                }
            }
            if let Some(right) = padding.get("right") {
                if let JsParamValue::Number(right) = right {
                    table_style.padding_right = *right;
                }
            }
        }
    }

    fn get_align(table_style: &mut TableStyle, align: &JsParamValue) {
        if let JsParamValue::Object(align) = align {
            if let Some(vertical) = align.get("vertical") {
                if let JsParamValue::Text(vertical) = vertical {
                    match vertical.as_str() {
                        "bottom" => {
                            table_style.vertical_align = VerticalAlign::Bottom;
                        }
                        "middle" => {
                            table_style.vertical_align = VerticalAlign::Middle;
                        }
                        _ => {
                            table_style.vertical_align = VerticalAlign::Top;
                        }
                    }
                }
            }
        }
    }

    fn get_grid(table_style: &mut TableStyle, grid: &JsParamValue) {
        table_style.grid_visible = true;
        if let JsParamValue::Object(grid) = grid {
            if let Some(width) = grid.get("width") {
                if let JsParamValue::Number(width) = width {
                    table_style.grid_width = *width;
                    //log(&format!("Table grid width is {}", width));
                }
            }
            if let Some(color) = grid.get("color") {
                if let Some(rgb_color) = Color::from_param(color) {
                    table_style.grid_color = rgb_color;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImageStyle {
    pub horizontal_align: HorizontalAlign,
}

impl ImageStyle {
    pub fn new() -> ImageStyle {
        ImageStyle {
            horizontal_align: HorizontalAlign::Center,
        }
    }
    pub fn from_content(content: &JsContent) -> ImageStyle {
        let mut image_style = ImageStyle::new();
        if let Some(align) = content.params.get("align") {
            image_style.horizontal_align = if let JsParamValue::Text(image_align) = align {
                match image_align.as_str() {
                    "right" => HorizontalAlign::Right,
                    "center" => HorizontalAlign::Center,
                    _ => HorizontalAlign::Left,
                }
            } else {
                HorizontalAlign::Left
            }
        }
        image_style
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PathStyle {
    pub horizontal_align: HorizontalAlign,
}

impl PathStyle {
    pub fn new() -> PathStyle {
        PathStyle {
            horizontal_align: HorizontalAlign::Center,
        }
    }
    pub fn from_content(content: &JsContent) -> PathStyle {
        let mut path_style = PathStyle::new();
        if let Some(align) = content.params.get("align") {
            path_style.horizontal_align = if let JsParamValue::Text(path_align) = align {
                match path_align.as_str() {
                    "right" => HorizontalAlign::Right,
                    "center" => HorizontalAlign::Center,
                    _ => HorizontalAlign::Left,
                }
            } else {
                HorizontalAlign::Left
            }
        }
        path_style
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CellStyle {
    pub background_color: Option<Color>,
}

impl CellStyle {
    pub fn new() -> CellStyle {
        CellStyle {
            background_color: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
    pub fn from_param_or_default(value: Option<&JsParamValue>, default: Color) -> Color {
        if let Some(value) = value {
            if let Some(color) = Color::from_param(value) {
                return color;
            }
        }
        default
    }
    pub fn from_param(color_arr: &JsParamValue) -> Option<Color> {
        let mut rgb_color = Color::new(0.0, 0.0, 0.0);
        if let JsParamValue::Array(color_arr) = color_arr {
            if color_arr.len() != 3 {
                return None;
            }
            for (index, color) in color_arr.iter().enumerate() {
                if let JsParamValue::Number(color) = color {
                    match index {
                        0 => {
                            rgb_color.r = *color;
                        }
                        1 => {
                            rgb_color.g = *color;
                        }
                        2 => {
                            rgb_color.b = *color;
                        }
                        _ => (),
                    }
                }
            }
            return Some(rgb_color);
        }
        None
    }
}

pub struct ParagraphStyle {
    pub leading: f32,
    pub align: HorizontalAlign,
    pub bullet: Option<String>,
    pub bullet_indent: f32,
    pub padding: (f32, f32, f32, f32),
    pub wrap: bool,
    pub color: Color,
    pub link_color: Color,
}

impl ParagraphStyle {
    pub fn new(
        leading: f32,
        align: HorizontalAlign,
        padding: (f32, f32, f32, f32),
    ) -> ParagraphStyle {
        ParagraphStyle {
            leading,
            align,
            bullet: None,
            bullet_indent: 0.0,
            padding,
            wrap: true,
            color: Color::new(0.0, 0.0, 0.0),
            link_color: Color::new(1.0, 0.0, 0.0),
        }
    }
    pub fn from_content(content: &JsContent, p_font_size: f32) -> ParagraphStyle {
        let p_leading = get_number_from_js(content.params.get("leading"), p_font_size + 2.0);
        let p_padding = ParagraphStyle::get_paragraph_padding(&content, p_font_size);
        let p_align = HorizontalAlign::from_content(&content);
        let p_bullet: Option<String> = if let Some(bullet) = content.params.get("bullet") {
            match bullet {
                JsParamValue::Text(s) => Some(s.to_string()),
                _ => None,
            }
        } else {
            None
        };
        let p_bullet_indent = get_number_from_js(content.params.get("bullet_indent"), 0.0);
        let p_color = if let Some(color) = content.params.get("color") {
            if let Some(rgb_color) = Color::from_param(color) {
                rgb_color
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        } else {
            Color::new(0.0, 0.0, 0.0)
        };
        let link_color = if let Some(color) = content.params.get("link_color") {
            if let Some(rgb_color) = Color::from_param(color) {
                rgb_color
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        } else {
            Color::new(0.0, 0.0, 0.0)
        };
        let wrap = get_bool_from_js(content.params.get("wrap"), true);
        ParagraphStyle {
            leading: p_leading,
            align: p_align,
            bullet: p_bullet,
            bullet_indent: p_bullet_indent,
            padding: p_padding,
            wrap,
            color: p_color,
            link_color,
        }
    }
    fn get_paragraph_padding(content: &JsContent, font_size: f32) -> (f32, f32, f32, f32) {
        let mut padding_top = font_size / 2.0;
        let mut padding_left = 0.0;
        let mut padding_bottom = font_size / 2.0;
        let mut padding_right = 0.0;
        if let Some(padding) = content.params.get("padding") {
            if let JsParamValue::Object(padding) = padding {
                if let Some(top) = padding.get("top") {
                    if let JsParamValue::Number(top) = top {
                        padding_top = *top;
                    }
                }
                if let Some(left) = padding.get("left") {
                    if let JsParamValue::Number(left) = left {
                        padding_left = *left;
                    }
                }
                if let Some(bottom) = padding.get("bottom") {
                    if let JsParamValue::Number(bottom) = bottom {
                        padding_bottom = *bottom;
                    }
                }
                if let Some(right) = padding.get("right") {
                    if let JsParamValue::Number(right) = right {
                        padding_right = *right;
                    }
                }
            } else if let JsParamValue::Array(padding_arr) = padding {
                // Also allow padding to be in [top, left, bottom, right] format.
                if padding_arr.len() == 4 {
                    for (index, p) in padding_arr.iter().enumerate() {
                        if let JsParamValue::Number(p) = p {
                            match index {
                                0 => {
                                    padding_top = *p;
                                }
                                1 => {
                                    padding_left = *p;
                                }
                                2 => {
                                    padding_bottom = *p;
                                }
                                3 => {
                                    padding_right = *p;
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        (padding_top, padding_left, padding_bottom, padding_right)
    }
}
