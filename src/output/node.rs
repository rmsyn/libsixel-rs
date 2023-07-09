use alloc::vec::Vec;

use crate::{
    dimension::*,
    std::{cmp, ops::Range},
    Error, Result,
};

use super::PaletteState;

/// Encoded Sixel node data
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pal: u32,
    sx: u32,
    mx: u32,
    map: Range<usize>,
}

impl Node {
    /// Createa a new [Node].
    pub const fn new() -> Self {
        Self {
            pal: 0,
            sx: 0,
            mx: 0,
            map: (0..0),
        }
    }

    /// Gets the palette value.
    pub const fn pal(&self) -> u32 {
        self.pal
    }

    /// Gets the sixel value.
    pub const fn sx(&self) -> u32 {
        self.sx
    }

    /// Sets the sixel value.
    pub fn set_sx(&mut self, val: u32) {
        self.sx = val;
    }

    /// Builder function that sets the sixel value.
    pub fn with_sx(mut self, val: u32) -> Self {
        self.set_sx(val);
        self
    }

    /// Gets the map value.
    pub const fn mx(&self) -> u32 {
        self.mx
    }

    /// Sets the map value.
    pub fn set_mx(&mut self, val: u32) {
        self.mx = val;
    }

    /// Builder function that sets the map value.
    pub fn with_mx(mut self, val: u32) -> Self {
        self.set_mx(val);
        self
    }

    /// Gets a range over the color map.
    pub fn map(&self) -> &Range<usize> {
        &self.map
    }

    /// Sets the color map range.
    pub fn set_map(&mut self, map: Range<usize>) {
        self.map = map;
    }

    /// Builder function that sets the color map range.
    pub fn with_map(mut self, map: Range<usize>) -> Self {
        self.set_map(map);
        self
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

/// List of [Node]s over color map values.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeList {
    nodes: Vec<Node>,
    map: Vec<u8>,
}

impl NodeList {
    /// Creates a new [NodeList].
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            map: Vec::new(),
        }
    }

    /// Creates a new [NodeList] with provided capacities.
    pub fn with_capacity(node_cap: usize, map_cap: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(node_cap),
            map: Vec::with_capacity(map_cap),
        }
    }

    /// Gets a reference to the list of [Node]s.
    pub fn nodes(&self) -> &[Node] {
        self.nodes.as_ref()
    }

    /// Gets a mutable reference to the list of [Node]s.
    pub fn nodes_mut(&mut self) -> &mut [Node] {
        self.nodes.as_mut()
    }

    /// Sets the list of [Node]s.
    pub fn set_nodes<N: Into<Vec<Node>>>(&mut self, nodes: N) {
        self.nodes = nodes.into();
    }

    /// Builder function that sets the list of [Node]s.
    pub fn with_nodes<N: Into<Vec<Node>>>(mut self, nodes: N) -> Self {
        self.set_nodes(nodes);
        self
    }

    /// Gets a reference to the list of [Node]s.
    pub fn nodes_inner_mut(&mut self) -> &mut Vec<Node> {
        &mut self.nodes
    }

    /// Gets a reference to the color map.
    pub fn map(&self) -> &[u8] {
        self.map.as_ref()
    }

    /// Gets a mutable reference to the color map.
    pub fn map_mut(&mut self) -> &mut [u8] {
        self.map.as_mut()
    }

    /// Sets the color map.
    pub fn set_map<M: Into<Vec<u8>>>(&mut self, map: M) {
        self.map = map.into();
    }

    /// Builder function that sets the color map.
    pub fn with_map<M: Into<Vec<u8>>>(mut self, map: M) -> Self {
        self.set_map(map);
        self
    }

    /// Gets a reference to the color map.
    pub fn map_inner_mut(&mut self) -> &mut Vec<u8> {
        &mut self.map
    }

    /// Fills the [Node] list values.
    pub fn fill_nodes(&mut self, ncolors: usize, width: usize) -> Result<()> {
        let max_depth = ncolors * width + (3 * width);
        let map_len = self.map.len();

        if max_depth > map_len {
            let err_msg = format!("ncolors({ncolors}) * width({width}) + 3*width > map_len({map_len}), max_depth({max_depth}) is out-of-bounds");
            Err(Error::Output(err_msg))
        } else {
            for c in 0..ncolors {
                for sx in 0..width {
                    if self.map[c * width + sx] == 0 {
                        continue;
                    }

                    let mut mx = sx + 1;
                    while mx < width {
                        if self.map[c * width + mx] != 0 {
                            mx += 1;
                            continue;
                        }

                        let mut n = mx + 1;
                        while n < width {
                            if self.map[c * width + mx + n] != 0 {
                                break;
                            }

                            n += 1;
                        }

                        if n >= 10 || (mx + n) >= width {
                            break;
                        }

                        mx = mx + n - 1;
                    }

                    self.nodes.push(Node::new());
                    let node_idx = self.nodes.len() - 1;
                    {
                        let np = &mut self.nodes[node_idx];

                        np.pal = c as u32;
                        np.sx = sx as u32;
                        np.mx = mx as u32;
                        np.map = (c * width)..self.map.len();
                    }
                    self.nodes.sort_by(|lhs, rhs| {
                        if lhs.sx < rhs.sx || (lhs.sx == rhs.sx && lhs.mx > rhs.mx) {
                            cmp::Ordering::Less
                        } else {
                            cmp::Ordering::Equal
                        }
                    });
                }
            }

            Ok(())
        }
    }

    /// Fills the color map values.
    pub fn fill_map(
        &mut self,
        pixels: &[u8],
        color_dimension: ColorDimension,
        space_dimension: SpaceDimension,
        i: usize,
        pal_state: &[PaletteState],
        mut fillable: bool,
    ) -> Result<bool> {
        let (width, y) = (space_dimension.width, space_dimension.y);
        let (ncolors, keycolor) = (color_dimension.n_colors, color_dimension.key_color);

        for x in 0..width {
            if y > i32::MAX as usize / width {
                let err_msg =
                    format!("encode_body: integer overflow: y=({y}) > i32::MAX / width({width})");
                return Err(Error::Output(err_msg));
            }
            let mut check_int_overflow = y * width;
            if check_int_overflow > i32::MAX as usize - x {
                let err_msg = format!(
                    "encode_body: integer overflow: y({y}) * width({width}) > i32::MAX - x({x})"
                );
                return Err(Error::Output(err_msg));
            }
            let pix = pixels
                .get(check_int_overflow + x)
                .map(|&p| p as usize)
                .unwrap_or(0);
            if (0..ncolors).contains(&pix) && pix as i32 != keycolor {
                if pix > i32::MAX as usize / width {
                    let err_msg = format!(
                        "encode_body: integer overflow: pix({pix}) > i32::MAX / width({width})"
                    );
                    return Err(Error::Output(err_msg));
                }
                check_int_overflow = pix * width;
                if check_int_overflow > i32::MAX as usize - x {
                    let err_msg = format!("encode_body: integer overflow: pix({pix}) * width({width}) > i32::MAX - x({x})");
                    return Err(Error::Output(err_msg));
                }
                self.map[pix * width + x] |= 1 << i;
            } else if pal_state.is_empty() {
                fillable = false;
            }
        }

        Ok(fillable)
    }
}
