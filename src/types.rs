#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetFormat {
    Hex,
    Rgb,
    Rgba,
    Hsl,
    Hsla,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOption {
    Spectrum,
    Luminance,
    Opacity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HueFilter {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpacityFilter {
    Opaque,
    Transparent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaturationFilter {
    Grayscale,
    Saturated,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterOptions {
    pub hues: Option<Vec<HueFilter>>,
    pub opacity: Option<OpacityFilter>,
    pub saturation: Option<SaturationFilter>,
    pub transparency_threshold: f32,
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self {
            hues: None,
            opacity: None,
            saturation: None,
            transparency_threshold: 0.999,
        }
    }
}

impl FilterOptions {
    pub fn with_hues<I: IntoIterator<Item = HueFilter>>(mut self, hues: I) -> Self {
        let hues_vec: Vec<HueFilter> = hues.into_iter().collect();
        self.hues = if hues_vec.is_empty() {
            None
        } else {
            Some(hues_vec)
        };
        self
    }

    pub fn with_opacity(mut self, opacity: OpacityFilter) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn with_saturation(mut self, saturation: SaturationFilter) -> Self {
        self.saturation = Some(saturation);
        self
    }

    pub fn with_transparency_threshold(mut self, threshold: f32) -> Self {
        self.transparency_threshold = threshold.clamp(0.0, 1.0);
        self
    }
}
