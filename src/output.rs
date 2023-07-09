//! Types and functions for outputting Sixel data.

use alloc::vec::Vec;

use crate::{
    config::PaletteType,
    constants::*,
    dimension::*,
    dither::{self, *},
    encoder::*,
    pixel_format::*,
    quant::QualityMode,
    std::{cmp, fmt},
    tosixel::*,
    Error, Result,
};

mod node;
#[cfg(test)]
mod tests;

pub use node::*;

/// Output packet length
pub const PACKET_LEN: usize = SIXEL_OUTPUT_PACKET_SIZE;
/// Allocate two packet lengths to allow for partial overflow
pub const OUTPUT_BUFFER_LEN: usize = PACKET_LEN * 2;

/// Convenience alias for an output packet buffer
pub type Packet = [u8; PACKET_LEN];

/// Convenience alias for a write callback function.
pub type WriteFn = fn(&[u8], &mut fmt::Formatter<'_>) -> fmt::Result;

/// Represents whether the was a palette hit or change.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PaletteState {
    #[default]
    None = 0,
    Hit = 1,
    Change = 2,
}

/// Wrapper for formatting output data.
pub struct OutputBuf<'o> {
    pub data: &'o [u8],
    pub fn_write: WriteFn,
}

impl<'o> fmt::Display for OutputBuf<'o> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.fn_write)(self.data, f)
    }
}

/// Represents how to output Sixel data
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    has_8bit_control: bool,
    has_sixel_scrolling: bool,
    has_gri_arg_limit: bool,
    has_sdm_glitch: bool,
    skip_dcs_envelope: bool,
    palette_type: PaletteType,
    fn_write: WriteFn,
    save_pixel: u8,
    save_count: usize,
    active_palette: u32,
    penetrate_multiplexer: bool,
    encode_policy: EncodePolicy,
    ormode: bool,
    buffer: [u8; OUTPUT_BUFFER_LEN],
    pos: usize,
}

impl Output {
    /// Creates a new [Output].
    pub fn new() -> Self {
        Self {
            has_8bit_control: false,
            has_sixel_scrolling: false,
            has_gri_arg_limit: false,
            has_sdm_glitch: false,
            skip_dcs_envelope: false,
            palette_type: PaletteType::Auto,
            fn_write: write_callback,
            save_pixel: 0,
            save_count: 0,
            active_palette: 0,
            penetrate_multiplexer: false,
            encode_policy: EncodePolicy::Auto,
            ormode: false,
            buffer: [0u8; OUTPUT_BUFFER_LEN],
            pos: 0,
        }
    }

    /// Gets whether the terminal has 8-bit (`true`) or 7-bit (`false`) support.
    pub const fn has_8bit_control(&self) -> bool {
        self.has_8bit_control
    }

    /// Sets whether the terminal has 8-bit (`true`) or 7-bit (`false`) support.
    pub fn set_has_8bit_control(&mut self, val: bool) {
        self.has_8bit_control = val;
    }

    /// Builder function that sets whether the terminal has 8-bit (`true`) or 7-bit (`false`) support.
    pub fn with_8bit_control(mut self, val: bool) -> Self {
        self.set_has_8bit_control(val);
        self
    }

    /// Gets whether the terminal has Sixel scrolling.
    pub const fn has_sixel_scrolling(&self) -> bool {
        self.has_sixel_scrolling
    }

    /// Sets whether the terminal has Sixel scrolling.
    pub fn set_has_sixel_scrolling(&mut self, val: bool) {
        self.has_sixel_scrolling = val;
    }

    /// Gets whether argument of repeat introducer (DECGRI) is limited to `255`.
    pub const fn has_gri_arg_limit(&self) -> bool {
        self.has_gri_arg_limit
    }

    /// Sets whether argument of repeat introducer (DECGRI) is limited to `255`.
    pub fn set_has_gri_arg_limit(&mut self, val: bool) {
        self.has_gri_arg_limit = val;
    }

    /// Gets whether the terminal has the SDM glitch to enable Sixel scrolling.
    ///
    /// Glitch: `DECSDM set CSI ? 80 h`
    pub const fn has_sdm_glitch(&self) -> bool {
        self.has_sdm_glitch
    }

    /// Sets whether the terminal has the SDM glitch to enable Sixel scrolling.
    ///
    /// Glitch: `DECSDM set CSI ? 80 h`
    pub fn set_has_sdm_glitch(&mut self, val: bool) {
        self.has_sdm_glitch = val;
    }

    /// Gets whether to skip the DCS envelope.
    pub const fn skip_dcs_envelope(&self) -> bool {
        self.skip_dcs_envelope
    }

    /// Sets whether to skip the DCS envelope.
    pub fn set_skip_dcs_envelope(&mut self, val: bool) {
        self.skip_dcs_envelope = val;
    }

    /// Gets the [PaletteType].
    pub const fn palette_type(&self) -> PaletteType {
        self.palette_type
    }

    /// Sets the [PaletteType].
    pub fn set_palette_type(&mut self, val: PaletteType) {
        self.palette_type = val;
    }

    /// Builder function that sets the [PaletteType].
    pub fn with_palette_type(mut self, val: PaletteType) -> Self {
        self.set_palette_type(val);
        self
    }

    /// Gets the save pixel.
    pub const fn save_pixel(&self) -> u8 {
        self.save_pixel
    }

    /// Sets the save pixel.
    pub fn set_save_pixel(&mut self, val: u8) {
        self.save_pixel = val;
    }

    /// Builder function that sets the save pixel.
    pub fn with_save_pixel(mut self, val: u8) -> Self {
        self.set_save_pixel(val);
        self
    }

    /// Gets the save pixel count.
    pub const fn save_count(&self) -> usize {
        self.save_count
    }

    /// Sets the save pixel count.
    pub fn set_save_count(&mut self, val: usize) {
        self.save_count = val;
    }

    /// Increments the save pixel count.
    pub fn increment_save_count(&mut self) {
        self.save_count = self.save_count.saturating_add(1);
    }

    /// Gets the active palette.
    pub const fn active_palette(&self) -> u32 {
        self.active_palette
    }

    /// Sets the active palette.
    pub fn set_active_palette(&mut self, val: u32) {
        self.active_palette = val;
    }

    /// Builder function that sets the active palette.
    pub fn with_active_palette(mut self, val: u32) -> Self {
        self.set_active_palette(val);
        self
    }

    /// Gets whether to penetrate the terminal multiplexer.
    pub const fn penetrate_multiplexer(&self) -> bool {
        self.penetrate_multiplexer
    }

    /// Sets whether to penetrate the terminal multiplexer.
    pub fn set_penetrate_multiplexer(&mut self, val: bool) {
        self.penetrate_multiplexer = val;
    }

    /// Builder function that sets whether to penetrate the terminal multiplexer.
    pub fn with_penetrate_multiplexer(mut self, val: bool) -> Self {
        self.set_penetrate_multiplexer(val);
        self
    }

    /// Gets the [EncodePolicy].
    pub const fn encode_policy(&self) -> EncodePolicy {
        self.encode_policy
    }

    /// Sets the [EncodePolicy].
    pub fn set_encode_policy(&mut self, val: EncodePolicy) {
        self.encode_policy = val;
    }

    /// Gets whether the terminal is in ormode.
    pub const fn ormode(&self) -> bool {
        self.ormode
    }

    /// Sets whether the terminal is in ormode.
    pub fn set_ormode(&mut self, val: bool) {
        self.ormode = val;
    }

    /// Gets a reference to the [Output] buffer.
    pub fn buffer(&self) -> &[u8] {
        self.buffer[self.pos..].as_ref()
    }

    /// Gets a mutable reference the [Output] buffer.
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.buffer[self.pos..].as_mut()
    }

    /// Sets the [Output] buffer.
    pub fn set_buffer(&mut self, buf: &[u8]) {
        let len = cmp::min(buf.len(), self.buffer.len());
        self.buffer[..len].copy_from_slice(buf[..len].as_ref());
        self.pos = len;
    }

    /// Builder function that sets the [Output] buffer.
    pub fn with_buffer(mut self, buf: &[u8]) -> Self {
        self.set_buffer(buf);
        self
    }

    /// Writes data to the [Output] buffer.
    #[cfg(feature = "std")]
    pub fn write(&self, data: &[u8]) {
        let out_buf = OutputBuf {
            data,
            fn_write: self.fn_write,
        };

        print!("{out_buf}");
    }

    /// Writes data to the [Output] buffer.
    #[cfg(not(feature = "std"))]
    pub fn write(&self, data: &[u8]) {
        use crate::print;

        let out_buf = OutputBuf {
            data,
            fn_write: self.fn_write,
        };

        print!("{out_buf}");
    }

    /// Sets the write callback function.
    pub fn set_write_fn(&mut self, fn_write: WriteFn) {
        self.fn_write = fn_write;
    }

    /// Builder function that sets the write callback function.
    pub fn with_write_fn(mut self, fn_write: WriteFn) -> Self {
        self.set_write_fn(fn_write);
        self
    }

    /// Wraps Sixel buffer data in DCS escape sequences.
    pub fn penetrate(&mut self, dcs_start: &[u8], dcs_end: &[u8]) {
        self.write(dcs_start);
        self.write(self.buffer[..PACKET_LEN].as_ref());
        self.write(dcs_end);
    }

    /// Checks if the buffer has enough data to write a full Sixel packet.
    pub fn advance(&mut self, n_write: usize) {
        // if the buffer is a full packet, write buffer data
        let mut adv_pos = self.pos + n_write;
        if adv_pos >= PACKET_LEN {
            if self.penetrate_multiplexer {
                self.penetrate(DCS_START_7BIT.as_ref(), DCS_END_7BIT.as_ref())
            } else {
                self.write(self.buffer[..PACKET_LEN].as_ref());
            }

            // get the number of pixels advanced into the second packet
            adv_pos %= PACKET_LEN;

            // copy bytes from the second packet into the first packet, kind of like a ring buffer
            let copy_start = PACKET_LEN;
            let copy_end = copy_start + adv_pos;

            for i in copy_start..copy_end {
                self.buffer[i - PACKET_LEN] = self.buffer[i];
            }

            // set the position to the advanced position
            self.pos = adv_pos;
        } else {
            self.pos += n_write;
        }
    }

    /// Writes saved pixel(s) to the [Output] buffer.
    pub fn put_flash(&mut self) -> Result<()> {
        let save_pixel = self.save_pixel;

        // VT240 Max 255 ?
        if self.has_gri_arg_limit {
            while self.save_count > 255 {
                // argument of DECGRI('!') is limitted to 255 in real VT
                puts(self.buffer_mut(), "!255");
                self.advance(4);

                putc(self.buffer_mut(), save_pixel);
                self.advance(1);

                self.save_count = self.save_count.saturating_sub(255);
            }
        }

        let save_count = self.save_count;

        if save_count > 3 {
            // DECGRI Graphics Repeat Introducer ! Pn Ch
            putc(self.buffer_mut(), b'!');
            self.advance(1);

            let n_write = putnum(self.buffer_mut(), save_count as u32);
            self.advance(n_write);

            putc(self.buffer_mut(), save_pixel);
            self.advance(1);
        } else {
            for _ in 0..self.save_count {
                self.buffer_mut()[0] = save_pixel;
                self.advance(1);
            }
        }

        self.save_pixel = 0;
        self.save_count = 0;

        Ok(())
    }

    /// Save a pixel to the [Output].
    pub fn put_pixel(&mut self, mut pix: u8) -> Result<()> {
        // check pixel is in valid range
        if !(0..b'?').contains(&pix) {
            pix = 0;
        }

        // shift pixel value to sixel range
        pix += b'?';

        if pix == self.save_pixel {
            // save pixel is repeated
            self.save_count = self.save_count.saturating_add(1);
        } else {
            // write previous save pixels
            self.put_flash()?;

            // store the new save pixel
            self.save_pixel = pix;
            self.save_count = 1;
        }

        Ok(())
    }

    /// Writes a [Node] to the [Output] buffer.
    ///
    /// Params:
    /// - `x`: header position
    /// - `np`: node object
    /// - `ncolors`: number of palette colors
    /// - `keycolor`: transparent color number
    pub fn put_node(
        &mut self,
        mut x: u32,
        np: &Node,
        map: &[u8],
        ncolors: usize,
        keycolor: i32,
    ) -> Result<u32> {
        if ncolors != 2 || keycolor == -1 {
            // designate palette index
            if self.active_palette != np.pal() {
                putc(self.buffer_mut(), b'#');
                self.advance(1);

                let nwrite = putnum(self.buffer_mut(), np.pal());
                self.advance(nwrite);
                self.active_palette = np.pal();
            }
        }

        while x < np.sx() {
            x += 1;

            if x != keycolor as u32 {
                self.put_pixel(0)?;
            }
        }

        while x < np.mx() {
            x += 1;

            if x != keycolor as u32 {
                let mrange = np.map();
                let pix = map[mrange.start..mrange.end]
                    .get(x as usize)
                    .copied()
                    .unwrap_or(0);

                self.put_pixel(pix)?;
            }
        }

        self.put_flash()?;

        Ok(x)
    }

    /// Encode the Sixel header into the [Output] buffer.
    pub fn encode_header(&mut self, width: usize, height: usize) -> Result<()> {
        let mut p = [0u8; 3];
        let mut pcount = p.len();

        let use_raster_attributes = true;

        if self.ormode {
            p[0] = 7;
            p[1] = 5;
        }

        self.pos = 0;

        if !self.skip_dcs_envelope {
            if self.has_8bit_control {
                putb(self.buffer_mut(), DCS_START_8BIT.as_ref());
                self.advance(DCS_START_8BIT.len());
            } else {
                putb(self.buffer_mut(), DCS_START_7BIT.as_ref());
                self.advance(DCS_START_7BIT.len())
            }
        }

        if p[2] == 0 {
            pcount = pcount.saturating_sub(1);

            if p[1] == 0 {
                pcount = pcount.saturating_sub(1);

                if p[0] == 0 {
                    pcount = pcount.saturating_sub(1);
                }
            }
        }

        if pcount > 0 {
            let mut nwrite = putnum(self.buffer_mut(), p[0] as u32);
            self.advance(nwrite);

            if pcount > 1 {
                putc(self.buffer_mut(), b';');
                self.advance(1);

                nwrite = putnum(self.buffer_mut(), p[1] as u32);
                self.advance(nwrite);

                if pcount > 2 {
                    putc(self.buffer_mut(), b';');
                    self.advance(1);

                    nwrite = putnum(self.buffer_mut(), p[2] as u32);
                    self.advance(nwrite);
                }
            }
        }

        putc(self.buffer_mut(), b'q');
        self.advance(1);

        if use_raster_attributes {
            // write width and height dimensions
            puts(self.buffer_mut(), r#""1;1;"#);
            self.advance(5);

            let mut nwrite = putnum(self.buffer_mut(), width as u32);
            self.advance(nwrite);

            putc(self.buffer_mut(), b';');
            self.advance(1);

            nwrite = putnum(self.buffer_mut(), height as u32);
            self.advance(nwrite);
        }

        Ok(())
    }

    /// Writes an RGB palette to the [Output] buffer.
    pub fn rgb_palette_definition(&mut self, palette: &[u8], n: usize, keycolor: usize) {
        if n != keycolor {
            // DECBGI Graphics Color Introducer # Pc; Pu; Px; Py; Pz
            putc(self.buffer_mut(), b'#');
            self.advance(1);

            let mut nwrite = putnum(self.buffer_mut(), n as u32);
            self.advance(nwrite);

            puts(self.buffer_mut(), ";2;");
            self.advance(3);

            let mut pnum = palette.get(n * 3).map(|&p| p as u32).unwrap_or(0);
            nwrite = putnum(self.buffer_mut(), (pnum * 100 + 127) / 255);
            self.advance(nwrite);

            putc(self.buffer_mut(), b';');
            self.advance(1);

            pnum = palette.get(n * 3 + 1).map(|&p| p as u32).unwrap_or(0);
            nwrite = putnum(self.buffer_mut(), (pnum * 100 + 127) / 255);
            self.advance(nwrite);

            putc(self.buffer_mut(), b';');
            self.advance(1);

            pnum = palette.get(n * 3 + 2).map(|&p| p as u32).unwrap_or(0);
            nwrite = putnum(self.buffer_mut(), (pnum * 100 + 127) / 255);
            self.advance(nwrite);
        }
    }

    /// Writes an HLS palette to the [Output] buffer.
    pub fn hls_palette_definition(&mut self, palette: &[u8], n: usize, keycolor: usize) {
        if n != keycolor {
            let r = palette.get(n * 3).map(|&p| p as u32).unwrap_or(0);
            let g = palette.get(n * 3 + 1).map(|&p| p as u32).unwrap_or(0);
            let b = palette.get(n * 3 + 2).map(|&p| p as u32).unwrap_or(0);
            let max = cmp::max(cmp::max(r, g), b);
            let min = cmp::min(cmp::min(r, g), b);
            let l = ((max + min) * 100 + 255) / 510;
            let (h, s) = if max == min {
                (0, 0)
            } else {
                let s = if l < 50 {
                    ((max - min) * 100) / (max + min)
                } else {
                    ((max - min) * 100) / ((255 - max) + (255 - min))
                };

                let h = if r == max {
                    120 + g.saturating_sub(b) * 60 / (max - min)
                } else if g == max {
                    240 + b.saturating_sub(r) * 60 / (max - min)
                } else if r < g {
                    // b == max
                    (360 + (r as i32 - g as i32) * 60 / (max as i32 - min as i32)) as u32
                } else {
                    r.saturating_sub(g) * 60 / (max - min)
                };

                (h, s)
            };

            // DECGCI Graphics Color Introducer # Pc; Pu; Px; Py; Pz
            putc(self.buffer_mut(), b'#');
            self.advance(1);

            let mut nwrite = putnum(self.buffer_mut(), n as u32);
            self.advance(nwrite);

            puts(self.buffer_mut(), ";1;");
            self.advance(3);

            nwrite = putnum(self.buffer_mut(), h);
            self.advance(nwrite);

            putc(self.buffer_mut(), b';');
            self.advance(1);

            nwrite = putnum(self.buffer_mut(), l);
            self.advance(nwrite);

            putc(self.buffer_mut(), b';');
            self.advance(1);

            nwrite = putnum(self.buffer_mut(), s);
            self.advance(nwrite);
        }
    }

    /// Encode the Sixel body into the [Output] buffer.
    // FIXME: write unit tests against libsixel test cases
    pub fn encode_body(
        &mut self,
        pixels: &[u8],
        mut space_dimension: SpaceDimension,
        palette: &[u8],
        color_dimension: ColorDimension,
        body_only: bool,
        pal_state: &[PaletteState],
    ) -> Result<()> {
        let (width, height) = (space_dimension.width, space_dimension.height);
        let (ncolors, keycolor) = (color_dimension.n_colors, color_dimension.key_color);

        if ncolors < 1 {
            Err(Error::Output("encode_body: invalid `ncolors` < 1".into()))
        } else {
            let len = ncolors * width;
            let mut node_list = NodeList::new().with_map(vec![0u8; len + 3 * width]);

            if !body_only && (ncolors != 2 || keycolor == -1) {
                if self.palette_type == PaletteType::Hls {
                    for n in 0..ncolors {
                        self.hls_palette_definition(palette, n, keycolor as usize);
                    }
                } else {
                    for n in 0..ncolors {
                        self.rgb_palette_definition(palette, n, keycolor as usize);
                    }
                }
            }

            // check color map invariants, and fill in the pixel map values
            let mut i = 0;
            for y in 0..height {
                let mut fillable = if self.encode_policy != EncodePolicy::Size {
                    false
                } else if !pal_state.is_empty() {
                    let pix = pixels
                        .get((y - i) * width)
                        .map(|&p| p as usize)
                        .unwrap_or(0);

                    pix < ncolors
                } else {
                    // normal sixel
                    true
                };

                // fill the color map values
                space_dimension.y = y;
                fillable = node_list.fill_map(
                    pixels,
                    color_dimension,
                    space_dimension,
                    i,
                    pal_state,
                    fillable,
                )?;

                i += 1;
                if i < 6 && (y + 1) < height {
                    continue;
                }

                // create Node entries for the color map
                node_list.fill_nodes(ncolors, width)?;

                if y != 5 {
                    // DECGNL Graphics Next Line
                    putc(self.buffer_mut(), b'-');
                    self.advance(1);
                }

                // write nodes to the Output buffer
                self.write_nodes(&mut node_list, ncolors, keycolor, i, fillable)?;

                i = 0;
                node_list.map_mut()[..len].iter_mut().for_each(|m| *m = 0);
            }

            if !pal_state.is_empty() {
                putc(self.buffer_mut(), b'$');
                self.advance(1);
            }

            Ok(())
        }
    }

    fn write_nodes(
        &mut self,
        node_list: &mut NodeList,
        ncolors: usize,
        keycolor: i32,
        i: usize,
        mut fillable: bool,
    ) -> Result<()> {
        let mut x = 0;
        while !node_list.nodes().is_empty() {
            if x > node_list.nodes()[0].sx() {
                // DECGCR Graphics Carriage Return
                putc(self.buffer_mut(), b'$');
                self.advance(1);
                x = 0;
            }

            if fillable {
                let (start, end) = {
                    let np = &node_list.nodes()[0];
                    let s = np.map().start;
                    let e = s + np.mx().saturating_sub(np.sx()) as usize;

                    (s, e)
                };
                let fill = (1 << i) - 1;

                node_list.map_mut()[start..end]
                    .iter_mut()
                    .for_each(|m| *m = fill);
            }

            x = self.put_node(x, &node_list.nodes()[0], node_list.map(), ncolors, keycolor)?;
            node_list.nodes_inner_mut().remove(0);

            let mut nodes_remove: Vec<usize> = Vec::with_capacity(node_list.nodes().len());

            // fill color map regions where Node::sx >= x
            for i in 0..node_list.nodes().len() {
                if node_list.nodes()[i].sx() < x {
                    continue;
                }

                if fillable {
                    let (start, end) = {
                        let np = &node_list.nodes()[i];
                        let s = np.map().start;
                        let e = s + np.mx().saturating_sub(np.sx()) as usize;
                        (s, e)
                    };
                    let fill = (1 << i) - 1;

                    node_list.map_mut()[start..end]
                        .iter_mut()
                        .for_each(|m| *m = fill);
                }

                x = self.put_node(x, &node_list.nodes()[i], node_list.map(), ncolors, keycolor)?;
                nodes_remove.push(i);
            }

            // remove nodes that were filled
            for &r in nodes_remove.iter() {
                if node_list.nodes().get(r).is_some() {
                    node_list.nodes_inner_mut().remove(r);
                }
            }

            fillable = false;
        }

        Ok(())
    }

    /// Encodes the Sixel footer into the [Output] buffer.
    pub fn encode_footer(&mut self) {
        if self.skip_dcs_envelope && !self.penetrate_multiplexer {
            // write the DCS ending escape sequence
            if self.has_8bit_control {
                putb(self.buffer_mut(), DCS_END_8BIT.as_ref());
                self.advance(DCS_END_8BIT.len());
            } else {
                putb(self.buffer_mut(), DCS_END_7BIT.as_ref());
                self.advance(DCS_END_7BIT.len());
            }
        }

        // flush buffer
        if self.pos > 0 {
            if self.penetrate_multiplexer {
                self.penetrate(DCS_START_7BIT.as_ref(), DCS_END_7BIT.as_ref());

                let mut buf = dcs_7bit(&[0o033]);
                buf.extend_from_slice(dcs_7bit(b"\\").as_ref());

                self.write(buf.as_ref());
            } else {
                self.write(self.buffer());
            }
        }
    }

    /// Encode the Sixel body into the [Output] buffer using bitwise-OR to combine pixel data from
    /// six rows of pixels.
    pub fn encode_body_ormode(
        &mut self,
        pixels: &[u8],
        space_dimension: SpaceDimension,
        palette: &[u8],
        color_dimension: ColorDimension,
    ) -> Result<()> {
        let (width, height) = (space_dimension.width, space_dimension.height);
        let (ncolors, keycolor) = (color_dimension.n_colors, color_dimension.key_color);

        let max_dim = width * height + (width * 5);
        let pixel_len = pixels.len();

        if max_dim > pixel_len {
            let err_msg =
                format!("max dimensions ({max_dim}) is out-of-bounds: pixel_len ({pixel_len})");
            return Err(Error::Output(err_msg));
        }

        for n in 0..ncolors {
            self.rgb_palette_definition(palette, n, keycolor as usize);
        }

        let mut nplanes = 8;
        for i in (2..=8).rev() {
            nplanes = i;
            if ncolors > (1 << (nplanes - 1)) {
                break;
            }
        }

        let mut buf = pixels;
        for _cur_h in (6..=height).step_by(6) {
            for plane in 0..nplanes {
                putc(self.buffer_mut(), b'#');
                self.advance(1);

                let nwrite = putnum(self.buffer_mut(), 1u32 << plane);
                self.advance(nwrite);

                let mut buf_p = buf;
                for _x in 0..width {
                    // take bits from the first pixel of the next 6 widths
                    let pix = (buf_p[0] >> plane) & 0b00_0001
                        | ((buf_p[width] >> plane) << 1) & 0b00_0010
                        | ((buf_p[width * 2] >> plane) << 2) & 0b00_0100
                        | ((buf_p[width * 3] >> plane) << 3) & 0b00_1000
                        | ((buf_p[width * 4] >> plane) << 4) & 0b01_0000
                        | ((buf_p[width * 5] >> plane) << 5) & 0b10_0000;

                    self.put_pixel(pix)?;
                    buf_p = &buf_p[1..];
                }

                self.put_flash()?;

                putc(self.buffer_mut(), b'$');
                self.advance(1);
            }

            putc(self.buffer_mut(), b'-');
            self.advance(1);

            buf = &buf[width * 6..];
        }

        let cur_h = height + (height % 6);

        if cur_h > height {
            for plane in 0..nplanes {
                putc(self.buffer_mut(), b'#');
                self.advance(1);

                let nwrite = putnum(self.buffer_mut(), 1u32 << plane);
                self.advance(nwrite);

                let buf_p = buf;
                for _x in 0..width {
                    let mut pix = (buf_p[0] >> plane) & 0x1;

                    let one_or = ((buf_p[width * 4] >> plane) << 4) & 0b01_0000;
                    let two_or = ((buf_p[width * 3] >> plane) << 3) & 0b00_1000;
                    let three_or = ((buf_p[width * 2] >> plane) << 2) & 0b00_0100;
                    let def_or = ((buf_p[width] >> plane) << 1) & 0b00_0010;

                    match cur_h - height {
                        1 => pix |= one_or | two_or | three_or | def_or,
                        2 => pix |= two_or | three_or | def_or,
                        3 => pix |= three_or | def_or,
                        _ => pix |= def_or,
                    }

                    self.put_pixel(pix)?;
                }

                self.put_flash()?;

                putc(self.buffer_mut(), b'$');
                self.advance(1);
            }
        }

        Ok(())
    }

    /// Encodes the Sixel header and body using [SixelDither] to apply the color map palette.
    pub fn encode_dither(
        &mut self,
        pixels: &[u8],
        space_dimension: SpaceDimension,
        dither: &mut SixelDither,
    ) -> Result<()> {
        let (width, height) = (space_dimension.width, space_dimension.height);

        let input_pixels: Vec<u8> = match dither.pixel_format {
            PixelFormat::Pal1
            | PixelFormat::Pal2
            | PixelFormat::Pal4
            | PixelFormat::G1
            | PixelFormat::G2
            | PixelFormat::G4 => {
                let buf_size: usize = width * height * 3;
                let mut paletted_pixels = vec![0u8; buf_size];
                dither
                    .pixel_format
                    .normalize(paletted_pixels.as_mut(), pixels, width, height)?;
                paletted_pixels
            }
            PixelFormat::Pal8 | PixelFormat::G8 | PixelFormat::Ga88 | PixelFormat::Ag88 => {
                pixels.into()
            }
            _ => dither.apply_palette(pixels, width, height)?,
        };

        self.encode_header(width, height)?;

        let color_dimension = ColorDimension {
            n_colors: dither.ncolors,
            key_color: dither.key_color,
        };

        if self.ormode {
            self.encode_body_ormode(
                input_pixels.as_ref(),
                space_dimension,
                dither.palette.as_ref(),
                color_dimension,
            )
        } else {
            self.encode_body(
                input_pixels.as_ref(),
                space_dimension,
                dither.palette.as_ref(),
                color_dimension,
                dither.body_only,
                &[],
            )
        }
    }

    /// Encodes a highcolor image into the [Output] buffer.
    pub fn encode_highcolor(
        &mut self,
        pixels: &mut [u8],
        mut sd: SpaceDimension,
        dither: &mut SixelDither,
    ) -> Result<()> {
        const MAX_COLORS: usize = 1 << 15;

        let (width, mut height) = (sd.width, sd.height);
        // w x h: paletted_pixels
        // max_colors * 2: rgbhit + rgb2pal
        // width * 6: marks
        let whole_size = width * height + (MAX_COLORS * 2) + (width * 6);

        if dither.pixel_format != PixelFormat::Rgb888 {
            let normal_len = width * height * 3;
            let mut normalized_pixels = vec![0u8; normal_len];

            dither
                .pixel_format
                .normalize(normalized_pixels.as_mut(), pixels, width, height)?;

            let pixel_len = pixels.len();
            if pixel_len < normal_len {
                return Err(Error::Output(format!(
                    "pixels length ({pixel_len}) is shorter than normalized length ({normal_len})"
                )));
            }

            pixels[..normal_len].copy_from_slice(normalized_pixels.as_ref());
        }

        // libsixel does things differently here
        //
        // they allocate a single chunk of contiguous memory, and use mutable offsets from the base
        // pointer for these different views.
        //
        // Since safe Rust won't let us take multiple mutable references, nor shared references
        // while a mutable reference is held, let's just use offsets into the buffer instead.
        let mut paletted_pixels = vec![0u8; whole_size];
        let mut pal_hit_count = [0u8; SIXEL_PALETTE_MAX];
        let mut pal_state = [PaletteState::default(); SIXEL_PALETTE_MAX];

        let rgbhit = width * height;
        let rgb2pal = rgbhit + MAX_COLORS;
        let marks = rgb2pal + MAX_COLORS;

        let mut output_count = 0;
        let mut pal = 0;
        let mut pix_offset = 0;

        let cd = ColorDimension {
            n_colors: 255,
            key_color: 255,
        };

        'next: loop {
            // current index into the `paletted_pixels` buffer
            let mut dst = 0;
            let mut next_pal = 0;
            let mut threshold = 1;
            let mut dirty = false;
            let mut pix = 0usize;
            let mut mptr = marks;
            let (mut y, mut mod_y) = (0usize, 0usize);
            sd.y = y;

            'inner: loop {
                height = sd.height;

                let highcolor_state = (
                    paletted_pixels.as_mut(),
                    pal_state.as_mut(),
                    pal_hit_count.as_mut(),
                    &mut pix_offset,
                    &mut pix,
                    &mut pal,
                    &mut next_pal,
                    &mut mptr,
                    &mut dst,
                    &mut threshold,
                    &mut dirty,
                    &mut sd,
                );

                Self::highcolor_pixel_loop(
                    pixels,
                    dither,
                    rgbhit,
                    rgb2pal,
                    output_count,
                    highcolor_state,
                )?;

                y += 1;
                if y >= height {
                    if dirty {
                        mod_y = 5;
                    } else {
                        break 'next;
                    }
                }

                if dirty && (mod_y == 5 || y >= height) {
                    let orig_height = height;

                    if output_count == 0 {
                        self.encode_header(width, height)?;
                    }

                    output_count += 1;
                    sd.height = y;
                    sd.y = y;

                    self.encode_body(
                        paletted_pixels.as_mut(),
                        sd,
                        dither.palette.as_ref(),
                        cd,
                        dither.body_only,
                        pal_state.as_ref(),
                    )?;

                    if y >= orig_height {
                        break 'next;
                    }

                    pix_offset = pix_offset.saturating_sub(6 * width * 3);
                    sd.height = orig_height - sd.height + 6;
                    break 'inner;
                }

                if mod_y == 5 {
                    paletted_pixels[marks..marks + (width * 6)].fill(0u8);
                }

                mod_y = (mod_y + 1) % 6;
            }
        }

        if output_count == 0 {
            self.encode_header(width, height)?;
        }

        self.encode_body(
            paletted_pixels.as_mut(),
            sd,
            dither.palette.as_ref(),
            cd,
            dither.body_only,
            pal_state.as_ref(),
        )?;

        self.encode_footer();

        Ok(())
    }

    fn highcolor_pixel_loop(
        pixels: &mut [u8],
        dither: &mut SixelDither,
        rgbhit: usize,
        rgb2pal: usize,
        output_count: usize,
        highcolor_state: HighcolorState,
    ) -> Result<()> {
        let (
            paletted_pixels,
            pal_state,
            pal_hit_count,
            pix_offset,
            pix,
            pal,
            next_pal,
            mptr,
            dst,
            threshold,
            dirty,
            sd,
        ) = highcolor_state;

        for x in 0..sd.width {
            if *mptr < paletted_pixels.len() - 1 {
                *mptr += 1;
            }
            if *dst < pal_state.len() - 1
                && *dst < pal_hit_count.len() - 1
                && *dst < paletted_pixels.len() - 1
            {
                *dst += 1;
            }
            if *pix_offset < pixels.len() - 4 {
                *pix_offset += 3;
            }

            if paletted_pixels[*mptr] != 0 {
                paletted_pixels[*dst] = 255;
            } else {
                sd.x = x;

                dither::method::apply_15bpp_dither(pixels, *sd, dither.method_for_diffuse)?;

                *pix = (((pixels[*pix_offset] & 0xf8) << 7)
                    | ((pixels[*pix_offset + 1] & 0xf8) << 2)
                    | ((pixels[*pix_offset + 2] >> 3) & 0x1f)) as usize;

                if paletted_pixels[rgbhit + *pix] == 0 {
                    while (*next_pal >= 255 && *threshold < 255)
                        && (pal_state[*next_pal] == PaletteState::None
                            || pal_hit_count[*next_pal] > *threshold as u8)
                    {
                        if *next_pal >= 255 {
                            *threshold = if *threshold == 1 { 9 } else { 255 };
                        } else {
                            *next_pal += 1;
                        }
                    }

                    if *next_pal >= 255 {
                        *dirty = true;
                        paletted_pixels[*dst] = 255;
                    } else {
                        *pal = *next_pal * 3;

                        paletted_pixels[rgbhit + *pix] = 1;
                        if output_count > 0 {
                            let p = &dither.palette[*pal..];
                            let rgb_idx = (((p[0] & 0xf8) << 7)
                                | ((p[1] & 0xf8) << 2)
                                | ((p[2] >> 3) & 0x1f))
                                as usize;

                            paletted_pixels[rgbhit + rgb_idx] = 0;
                        }

                        paletted_pixels[*dst] = *next_pal as u8;
                        paletted_pixels[rgb2pal + *pix] = *next_pal as u8;

                        *next_pal += 1;
                        *mptr += 1;

                        pal_state[*dst] = PaletteState::Change;
                        pal_hit_count[*dst] = 1;

                        dither.palette[*pal] = pixels[0];
                        *pal += 1;
                        dither.palette[*pal] = pixels[1];
                        *pal += 1;
                        dither.palette[*pal] = pixels[2];
                        *pal += 1;
                    }
                } else {
                    paletted_pixels[*dst] = paletted_pixels[rgb2pal + *pix];
                    *mptr = 1;
                    if pal_state[*dst] == PaletteState::None {
                        pal_state[*dst] = PaletteState::Hit;
                    }

                    pal_hit_count[*dst] = pal_hit_count[*dst].saturating_add(1);
                }
            }
        }

        Ok(())
    }

    /// Encode an image into the [Output] buffer.
    pub fn encode(
        &mut self,
        pixels: &mut [u8],
        dither: &mut SixelDither,
        sd: SpaceDimension,
    ) -> Result<()> {
        let (width, height) = (sd.width, sd.height);
        if width < 1 {
            Err(Error::Output(format!(
                "encode: bad width({width}) paramter, must be >= 1"
            )))
        } else if height < 1 {
            Err(Error::Output(format!(
                "encode: bad height({height}) paramter, must be >= 1"
            )))
        } else if dither.quality_mode == QualityMode::HighColor {
            self.encode_highcolor(pixels, sd, dither)
        } else {
            self.encode_dither(pixels, sd, dither)
        }
    }
}

type HighcolorState<'s> = (
    // paletted pixels
    &'s mut [u8],
    // palette state
    &'s mut [PaletteState],
    // pal_hit_count
    &'s mut [u8],
    // pix_offset
    &'s mut usize,
    // pix
    &'s mut usize,
    // pal
    &'s mut usize,
    // next_pal
    &'s mut usize,
    // mptr
    &'s mut usize,
    // dst
    &'s mut usize,
    // threshold
    &'s mut usize,
    // dirty
    &'s mut bool,
    // space dimension
    &'s mut SpaceDimension,
);

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}
