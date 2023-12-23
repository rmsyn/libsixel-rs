//! Types and methods for handling color map operations.

use alloc::vec::Vec;

use crate::std::{
    cmp,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{Error, Result};

mod box_vector;
mod palette;
mod tuple;

pub use box_vector::*;
pub use palette::*;
pub use tuple::*;

static COMPARE_PLANE: AtomicUsize = AtomicUsize::new(0);

/// Gets the global comapre plane value.
///
/// This is a parameter to the [compare_plane] function.
///
/// Separated to allow [compare_plane] to be used as a function parameter for `sort_by`.
pub fn global_compare_plane() -> usize {
    COMPARE_PLANE.load(Ordering::Relaxed)
}

/// Gets the global comapre plane value.
///
/// This is a parameter to the [compare_plane] function.
///
/// Separated to allow [compare_plane] to be used as a function parameter for `sort_by`.
pub fn set_global_compare_plane(plane: usize) {
    COMPARE_PLANE.store(plane, Ordering::SeqCst)
}

/// Sort [TupleInt] by item selected by the `global_compare_plane` parameter.
pub fn compare_plane(lhs: &TupleInt, rhs: &TupleInt) -> cmp::Ordering {
    let plane = global_compare_plane();

    let lhs_tuple = lhs.tuple.get(plane).unwrap_or(&0);
    let rhs_tuple = rhs.tuple.get(plane).unwrap_or(&0);

    lhs_tuple.cmp(rhs_tuple)
}

/// Histogram for [ColorMap] data.
pub type Histogram = Vec<u16>;

/// Built-in dither
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum BuiltinDither {
    /// Monochrome terminal with dark background
    #[default]
    MonoDark = 0,
    /// Monochrome terminal with dark background
    MonoLight,
    /// XTerm 16color
    Xterm16,
    /// XTerm 256color
    Xterm256,
    /// VT340 monochrome
    Vt340Mono,
    /// VT340 color
    Vt340Color,
}

/// Represents a map of colors and pixels.
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct ColorMap {
    pub table: Vec<TupleInt>,
}

impl ColorMap {
    /// Creates a new [ColorMap].
    pub const fn new() -> Self {
        Self { table: Vec::new() }
    }

    /// Creates a new [ColorMap] with the provided capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            table: Vec::with_capacity(cap),
        }
    }

    /// Creates a new [ColorMap] with `colors` [TupleInt]s, that each have `depth` entries.
    pub fn with_colors_and_depth(colors: usize, depth: usize) -> Self {
        Self {
            table: vec![TupleInt::with_depth(depth); colors],
        }
    }

    /// Gets a reference to the [ColorMap] table.
    pub fn table(&self) -> &[TupleInt] {
        self.table.as_ref()
    }

    /// Gets the length of the [ColorMap].
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Gets whether the [ColorMap] is empty.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// Produce a colormap containing the best colors to represent the
    /// image stream in file 'ifP'.  Figure it out using the median cut
    /// technique.
    ///
    /// The colormap will have 'reqcolors' or fewer colors in it, unless
    /// 'allcolors' is true, in which case it will have all the colors that
    /// are in the input.
    ///
    /// The colormap has the same maxval as the input.
    ///
    /// Put the colormap in newly allocated storage as a tupletable2
    /// and return its address as *colormapP.  Return the number of colors in
    /// it as *colorsP and its maxval as *colormapMaxvalP.
    ///
    /// Return the characteristics of the input file as
    /// *formatP and *freqPamP.  (This information is not really
    /// relevant to our colormap mission; just a fringe benefit).
    pub fn compute_from_input(
        data: &[u8],
        depth: usize,
        req_colors: usize,
        method_for_largest: MethodForLargest,
        method_for_rep: MethodForRep,
        quality_mode: QualityMode,
        orig_colors: &mut usize,
    ) -> Result<Self> {
        let mut color_map = Self::compute_histogram(data, depth, quality_mode)?;
        *orig_colors = color_map.table.len();

        if color_map.table.len() <= req_colors {
            Ok(color_map)
        } else {
            color_map.median_cut(depth, req_colors, method_for_largest, method_for_rep)
        }
    }

    /// Computes a histogram for a [ColorMap].
    pub fn compute_histogram(data: &[u8], depth: usize, quality_mode: QualityMode) -> Result<Self> {
        let len = data.len();

        let max_sample: usize = match quality_mode {
            QualityMode::Low | QualityMode::High => 18383,
            _ => 4_003_079,
        };

        let mut step = if len < max_sample.saturating_mul(depth) {
            depth.saturating_mul(6)
        } else {
            len.saturating_div(depth)
                .saturating_div(max_sample)
                .saturating_mul(depth)
        };

        if step == 0 {
            step = depth;
        }

        let hist_cap = 1usize.wrapping_shl(depth.saturating_mul(5) as u32);
        let mut histogram = vec![0u16; hist_cap];
        let mut hist_ref: Vec<u16> = Vec::with_capacity(hist_cap);

        for i in (0..len).step_by(step) {
            let bucket_idx = compute_hash(data[i..].as_ref(), 3);
            if bucket_idx >= hist_cap {
                continue;
            }

            let hist_val = histogram[bucket_idx];
            if hist_val == 0 {
                hist_ref.push(bucket_idx as u16);
            }

            if hist_val < u16::MAX {
                histogram[bucket_idx] = hist_val.saturating_add(1);
            }
        }

        let map_len = hist_ref.len();
        let mut color_map = Self::with_capacity(map_len);

        for &i in hist_ref.iter() {
            let hist_val = histogram[i as usize];
            if hist_val > 0 {
                let mut tuple = TupleInt::new();
                tuple.set_value(hist_val as u32);
                tuple.tuple = (0..map_len)
                    .map(|n| ((i >> (n * 5) & 0x1f) << 3) as u32)
                    .rev()
                    .collect();

                color_map.table.push(tuple);
            }
        }

        Ok(color_map)
    }

    /// Compute a set of only `new_colors` colors that best represent an
    /// image whose pixels are summarized by the histogram
    /// `color_freq_table`.  Each tuple in that table has depth `depth`.
    /// `color_freq_table.table[i]` tells the number of pixels in the subject image
    /// have a particular color.
    ///
    /// As a side effect, sort `color_freq_table`.
    pub fn median_cut(
        &mut self,
        depth: usize,
        new_colors: usize,
        method_for_largest: MethodForLargest,
        method_for_rep: MethodForRep,
    ) -> Result<Self> {
        let sum = self.table.iter().map(|t| t.value as usize).sum();

        // There is at least one box that contains at least 2 colors; ergo,
        // there is more splitting we can do.
        let mut bv = BoxVector::create(self.table.len(), new_colors, sum);

        let mut boxes = 1usize;
        let mut multicolor_boxes_exist = self.table.len() > 1;

        // Main loop: split boxes until we have enough.
        while boxes < new_colors && multicolor_boxes_exist {
            let bi = bv.0.iter().filter(|b| b.colors >= 2).count();

            if bi == 0 {
                multicolor_boxes_exist = false;
            } else {
                boxes = self.split_box(&mut bv, boxes, bi, depth, method_for_largest)?;
            }
        }

        self.from_bv(&bv, boxes, depth, new_colors, method_for_rep)
    }

    /// Split Box 'bi' in the box vector bv (so that bv contains one more box
    /// than it did as input).  Split it so that each new box represents about
    /// half of the pixels in the distribution given by 'colorfreqtable' for
    /// the colors in the original box, but with distinct colors in each of the
    /// two new boxes.
    ///
    /// Assume the box contains at least two colors.
    pub fn split_box(
        &mut self,
        bv: &mut BoxVector,
        mut boxes: usize,
        bi: usize,
        depth: usize,
        method_for_largest: MethodForLargest,
    ) -> Result<usize> {
        const MAX_DEPTH: usize = 16;

        if depth > MAX_DEPTH {
            return Err(Error::Quant(format!(
                "invalid depth, max: {MAX_DEPTH}, have: {depth}"
            )));
        }

        let bv_len = bv.0.len();
        if bi >= bv_len || boxes >= bv_len {
            return Err(Error::Quant(format!("invalid BoxVector index and/or number, length: {bv_len}, index: {bi}, boxes: {boxes}")));
        }

        let box_start = bv[bi].ind;
        let box_size = bv[bi].colors;
        let sm = bv[bi].sum;

        let mut min_val = [0u32; MAX_DEPTH];
        let mut max_val = [0u32; MAX_DEPTH];

        self.find_box_boundaries(depth, box_start, min_val.as_mut(), max_val.as_mut())?;

        // From libsixel comments:
        //
        // ```
        // Find the largest dimension, and sort by that component.  I have
        // included two methods for determining the "largest" dimension;
        // first by simply comparing the range in RGB space, and second by
        // transforming into luminosities before the comparison.
        // ```
        //
        // number of the plane with the largest spread
        let largest_dimension = match method_for_largest {
            MethodForLargest::Norm => largest_by_norm(&min_val[..depth], &max_val[..depth]),
            MethodForLargest::Lum => largest_by_luminosity(&min_val[..depth], &max_val[..depth]),
            _ => {
                return Err(Error::Quant(format!(
                    "internal: invalid valud for MethodForLargest, have: {method_for_largest}"
                )));
            }
        };

        // initial sum value
        let mut lower_sum = self.table[box_start].value as usize;

        // Set the gross global variable `compare_plane_plane` as a
        // parameter to `compare_plane()`, which is called by `qsort()`.
        let median_index = {
            // Now find the median based on the counts, so that about half
            // the pixels (not colors, pixels) are in each subdivision.

            let mut i = 1usize;

            while i < box_size && lower_sum < sm / 2 {
                lower_sum = lower_sum.saturating_add(self.table[box_start + i].value as usize);
                i += 1;
            }

            i
        };

        /* Split the box, and sort to bring the biggest boxes to the top.  */

        bv[bi].colors = median_index;
        bv[bi].sum = lower_sum;
        bv[boxes].ind = box_start.saturating_add(median_index);
        bv[boxes].colors = box_size.saturating_sub(median_index);
        bv[boxes].sum = sm.saturating_sub(lower_sum);

        boxes = boxes.saturating_add(1);

        set_global_compare_plane(largest_dimension);

        self.table[box_start..box_size]
            .as_mut()
            .sort_by(compare_plane);
        bv.0[..boxes].as_mut().sort_by(sum_compare);

        Ok(boxes)
    }

    /// Creates a [ColorMap] from a [BoxVector].
    ///
    /// From `libsixel` comments:
    ///
    ///
    /// ```no_build, no_run
    /// Ok, we've got enough boxes.  Now choose a representative color for
    /// each box.  There are a number of possible ways to make this choice.
    /// One would be to choose the center of the box; this ignores any structure
    /// within the boxes.  Another method would be to average all the colors in
    /// the box - this is the method specified in Heckbert's paper.  A third
    /// method is to average all the pixels in the box.
    /// ```
    pub fn from_bv(
        &self,
        bv: &BoxVector,
        boxes: usize,
        depth: usize,
        new_colors: usize,
        method_for_rep: MethodForRep,
    ) -> Result<Self> {
        let mut colormap = ColorMap::with_colors_and_depth(new_colors, depth);
        let bv_ref = bv.as_ref();

        for (bi, table) in bv_ref[..boxes]
            .iter()
            .zip(colormap.table[..boxes].iter_mut())
        {
            table.tuple = match method_for_rep {
                MethodForRep::CenterBox => self.center_box(bi.ind, bi.colors, depth)?,
                MethodForRep::AverageColors => self.average_colors(bi.ind, bi.colors, depth)?,
                MethodForRep::AveragePixels => self.average_pixels(bi.ind, bi.colors, depth)?,
                _ => {
                    return Err(Error::Quant(format!(
                        "invalid value of method_for_rep: {method_for_rep}"
                    )))
                }
            };
        }

        Ok(colormap)
    }

    /// Find the boundary of a box of pixels.
    ///
    /// From the `libsixel` docs:
    ///
    /// ```no_build, no_run
    /// Go through the box finding the minimum and maximum of each
    /// component - the boundaries of the box.
    /// ```
    pub fn find_box_boundaries(
        &self,
        depth: usize,
        box_start: usize,
        min_val: &mut [Sample],
        max_val: &mut [Sample],
    ) -> Result<()> {
        let table_len = self.table.len();
        let min_val_len = min_val.len();
        let max_val_len = max_val.len();
        let tuple_len = if let Some(tuple) = self.table.get(box_start) {
            tuple.tuple.len()
        } else {
            0
        };
        if (box_start + 1) > table_len
            || depth > tuple_len
            || depth > min_val_len
            || depth > max_val_len
        {
            return Err(Error::Quant(
                format!("box_start and/or depth is out-of-bounds, table length: {table_len}, box_start: {box_start}, tuple length: {tuple_len}, min_val length: {min_val_len} max_val length: {max_val_len}")
            ));
        }

        for (plane, &val) in self.table[box_start].tuple[..depth].iter().enumerate() {
            min_val[plane] = val;
            max_val[plane] = val;
        }

        for tuple in self.table[box_start + 1..].iter() {
            for (plane, &val) in tuple.tuple[..depth].iter().enumerate() {
                if val < min_val[plane] {
                    min_val[plane] = val;
                }

                if val > max_val[plane] {
                    max_val[plane] = val;
                }
            }
        }

        Ok(())
    }

    /// Finds the center of the box values.
    pub fn center_box(&self, box_start: usize, box_size: usize, depth: usize) -> Result<Tuple> {
        let table_len = self.table.len();
        let tuple_len = if let Some(tuple) = self.table.get(box_start) {
            tuple.tuple.len()
        } else {
            0
        };

        if box_start > table_len || box_size > table_len || depth > tuple_len {
            Err(Error::Quant(format!("box_start, box_size, and/or depth is out-of-bounds: table length: {table_len}, box_start: {box_start}, box_size {box_size}, tuple length: {tuple_len}, depth: {depth}")))
        } else {
            let mut new_tuple = Tuple::with_capacity(depth);

            for plane in 0..depth {
                let val = self.table[box_start].tuple[plane];
                let (mut min_val, mut max_val) = (val, val);

                for tuple in self.table[box_start + 1..box_size].iter() {
                    let v = tuple.tuple[plane];
                    min_val = cmp::min(v, min_val);
                    max_val = cmp::max(v, max_val);
                }

                new_tuple.push(min_val.saturating_add(max_val).saturating_div(2));
            }

            Ok(new_tuple)
        }
    }

    /// Averages color values in a [ColorMap].
    pub fn average_colors(&self, box_start: usize, box_size: usize, depth: usize) -> Result<Tuple> {
        let table_len = self.table.len();
        if box_start > table_len || box_size > table_len || box_start > box_size {
            Err(Error::Quant(format!("box_start and/or box_size is out-of-bounds, table length: {table_len}, box start: {box_start}, box size: {box_size}")))
        } else {
            let mut new_tuple = Tuple::with_capacity(depth);

            (0..depth).for_each(|plane| {
                let sum: u32 = self.table[box_start..box_size]
                    .iter()
                    .map(|t| t.tuple.get(plane).unwrap_or(&0))
                    .sum();

                new_tuple.push(sum / box_size as u32);
            });

            Ok(new_tuple)
        }
    }

    /// Averages pixel values in a [ColorMap].
    pub fn average_pixels(&self, box_start: usize, box_size: usize, depth: usize) -> Result<Tuple> {
        let table_len = self.table.len();
        if box_start > table_len || box_size > table_len || box_start > box_size {
            Err(Error::Quant(format!("box_start and/or box_size is out-of-bounds, table length: {table_len}, box start: {box_start}, box size: {box_size}")))
        } else {
            let n: u32 = self.table[box_start..box_size]
                .iter()
                .map(|t| t.value)
                .sum();

            let mut new_tuple = Tuple::with_capacity(depth);

            (0..depth).for_each(|plane| {
                let sum: u32 = self.table[box_start..box_size]
                    .iter()
                    .map(|t| t.tuple.get(plane).unwrap_or(&0).saturating_mul(t.value))
                    .sum();

                new_tuple.push(sum / n);
            });

            Ok(new_tuple)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<()> {
        let min_val = [1u32];
        let max_val = [2u32];

        assert_eq!(largest_by_luminosity(min_val.as_ref(), max_val.as_ref()), 0);

        // Should return the middle index, middle luminosity multiplier is significantly higher
        let min_val = [1u32, 2u32, 3u32];
        let max_val = [2u32, 4u32, 8u32];

        assert_eq!(largest_by_luminosity(min_val.as_ref(), max_val.as_ref()), 1);

        let min_val = [1u32, 3u32, 2u32];
        let max_val = [2u32, 8u32, 4u32];

        assert_eq!(largest_by_luminosity(min_val.as_ref(), max_val.as_ref()), 1);

        // Should return the last index, 5x max_val is enough to overcome middle luminosity
        // multiplier
        let min_val = [1u32, 2u32, 3u32];
        let max_val = [2u32, 4u32, 20u32];

        assert_eq!(largest_by_luminosity(min_val.as_ref(), max_val.as_ref()), 2);

        let min_val = [3u32, 1u32, 2u32];
        let max_val = [8u32, 2u32, 4u32];

        assert_eq!(largest_by_luminosity(min_val.as_ref(), max_val.as_ref()), 0);

        // Luminosity only calculate for depth == 3
        let min_val = [3u32, 1u32, 2u32, 0u32];
        let max_val = [8u32, 2u32, 4u32, 0u32];

        assert_eq!(largest_by_luminosity(min_val.as_ref(), max_val.as_ref()), 0);

        let min_val = [3u32, 1u32];
        let max_val = [8u32, 2u32];
        assert_eq!(largest_by_luminosity(min_val.as_ref(), max_val.as_ref()), 0);

        Ok(())
    }
}
