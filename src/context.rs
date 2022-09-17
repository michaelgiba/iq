use image::RgbaImage;
use std::cmp::{max, min, PartialOrd};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct IqPixel {
    pub x: u32,
    pub y: u32,
    pub c: [u8; 4],
}

impl IqPixel {
    pub fn negate(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            c: [!self.c[0], !self.c[1], !self.c[2], !self.c[3]],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context<T> {
    min_y: u32,
    max_y: u32,
    min_x: u32,
    max_x: u32,
    pixels: HashMap<(u32, u32), IqPixel>,
    annotations: HashMap<IqPixel, T>,
}

pub type BasicContext = Context<()>;
pub type AnnotatedFloatContext = Context<f64>;
pub type AnnotatedPixelContext = Context<IqPixel>;

impl<T> Context<T> {
    pub fn empty() -> Self {
        Self {
            min_y: 0,
            max_y: 0,
            min_x: 0,
            max_x: 0,
            pixels: HashMap::new(),
            annotations: HashMap::new(),
        }
    }

    pub fn blank(h: u32, w: u32) -> Self {
        let mut ctx = Self::empty();
        for y in 0..h {
            for x in 0..w {
                ctx.insert(IqPixel {
                    y,
                    x,
                    c: [255, 255, 255, 255],
                });
            }
        }
        ctx
    }

    pub fn from_contexts(contexts: Vec<Self>) -> Self {
        if contexts.is_empty() {
            Self::empty()
        } else {
            let mut out = Self::empty();
            for ctx in contexts {
                for pixel in ctx.iter() {
                    out.insert(pixel.clone())
                }
            }
            out
        }
    }

    pub fn write(&self, path: &str) {
        let mut img = RgbaImage::new(self.max_y + 1, self.max_x + 1);

        for pixel in self.iter() {
            img.put_pixel(
                pixel.x - self.min_x,
                pixel.y - self.min_y,
                image::Rgba(pixel.c),
            )
        }

        img.save(path).unwrap();
    }

    pub fn from_path(path: &str) -> Self {
        let img = image::open(&Path::new(path)).unwrap().to_rgba8();
        let mut out = Self::empty();

        for (x, y, c) in img.enumerate_pixels() {
            out.insert(IqPixel {
                y,
                x,
                c: [c[0], c[1], c[2], c[3]],
            })
        }

        out
    }

    pub fn subcontext(
        &self,
        y_bounds: (Option<u32>, Option<u32>),
        x_bounds: (Option<u32>, Option<u32>),
    ) -> Self {
        let lby = y_bounds.0.unwrap_or(self.min_y);
        let uby = y_bounds.1.unwrap_or(self.max_y);
        let lbx = x_bounds.0.unwrap_or(self.min_x);
        let ubx = x_bounds.1.unwrap_or(self.max_x);

        let mut subctx = Self::empty();
        for pixel in self.iter() {
            if lby <= pixel.y && pixel.y <= uby && lbx <= pixel.x && pixel.x <= ubx {
                subctx.insert(pixel.clone())
            }
        }
        subctx
    }

    pub fn width(&self) -> u32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> u32 {
        self.max_y - self.min_y
    }

    pub fn insert(&mut self, pixel: IqPixel) {
        self.min_x = min(pixel.x, self.min_x);
        self.max_x = max(pixel.x, self.max_x);
        self.min_y = min(pixel.y, self.min_y);
        self.max_y = max(pixel.y, self.max_y);
        self.pixels.insert((pixel.y, pixel.x), pixel);
    }

    pub fn count(&self) -> usize {
        self.pixels.len()
    }

    pub fn iter(&self) -> std::collections::hash_map::Values<(u32, u32), IqPixel> {
        self.pixels.values()
    }

    pub fn select(&self, selection_ctx: Context<T>) -> Context<T> {
        let mut selected_context = Context::empty();
        for pixel in self.iter() {
            if selection_ctx.pixels.contains_key(&(pixel.y, pixel.x)) {
                selected_context.insert(pixel.clone())
            }
        }
        selected_context
    }

    pub fn center(&self) -> IqPixel {
        let y = self.min_y + (self.max_y - self.min_y) / 2;
        let x = self.min_x + (self.max_x - self.min_x) / 2;
        if let Some(p) = self.pixels.get(&(y, x)) {
            p.clone()
        } else {
            IqPixel {
                y: y,
                x: x,
                c: [255, 255, 255, 255],
            }
        }
    }

    pub fn x_bounds(&self) -> (u32, u32) {
        (self.min_x, self.max_x)
    }

    pub fn y_bounds(&self) -> (u32, u32) {
        (self.min_y, self.max_y)
    }

    pub fn describe(&self) -> String {
        String::from("<details>")
    }

    pub fn from_iter<I, P, F>(iter: P, f: F) -> Self
    where
        F: Fn(I) -> IqPixel,
        P: IntoIterator<Item = I>,
    {
        let mut out = Self::empty();
        for item in iter {
            out.insert(f(item))
        }
        out
    }

    pub fn from_iter_with_annotation<I, P, F>(iter: P, f: F) -> Self
    where
        F: Fn(I) -> (IqPixel, T),
        P: IntoIterator<Item = I>,
    {
        let mut out = Self::empty();
        for item in iter {
            let result = f(item);
            out.insert_with_annotation(result.0, result.1)
        }
        out
    }

    pub fn insert_with_annotation(&mut self, pixel: IqPixel, annotation: T) {
        self.insert(pixel.clone());
        self.annotations.insert(pixel, annotation);
    }

    pub fn get_annotation(&self, pixel: &IqPixel) -> Option<&T> {
        self.annotations.get(pixel)
    }

    pub fn get_annotation_at_loc(&self, loc: (u32, u32)) -> Option<&T> {
        if let Some(p) = self.pixels.get(&loc) {
            self.get_annotation(p)
        } else {
            None
        }
    }

    pub fn iter_annotations(&self) -> std::collections::hash_map::Iter<IqPixel, T> {
        self.annotations.iter()
    }

    pub fn first(&self) -> &T {
        self.iter_annotations().next().unwrap().1
    }

    pub fn like<U>(ctx: &Context<U>, default: &T) -> Self
    where
        T: Clone,
    {
        let mut annotated_ctx = Self::empty();
        for pixel in ctx.iter() {
            annotated_ctx.insert_with_annotation(pixel.clone(), default.clone())
        }
        annotated_ctx
    }
}
