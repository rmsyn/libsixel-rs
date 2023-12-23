Documentation from vt100.net [VT330/VT340 Programmer Reference Manual](https://vt100.net/docs/vt3xx-gp/chapter14.html)

# Sixel Graphics

## What are Sixels?

The VT300 can send and receive sixel graphics data. You can draw monochrome or color (VT340 only) images with sixel data.

A sixel is a group of six pixels in a vertical column. A pixel (picture element) is the smallest displayable unit on a video screen. Sixels represent bitmap data for a graphic image. The terminal processes sixel data as bits of information. A bit value of 1 means turn on a pixel. A bit value of 0 means turn off the pixel.

You use a single character code for each sixel. The terminal uses 6 bits of the 8-bit character code to encode bitmap data.

You can use sixels to design character sets and fonts for display. Volume 1, Chapter 5 of this manual describes how to design and load soft character sets into the terminal.

## Sixel Data Format

The VT300 uses a device control string to send and receive sixel images.

___NOTE: See Volume 1, Chapter 2 of this manual for general information about device control strings.___

Many of Digital's printers recognize the control string format. Here are some examples.

|         |      |       |
|---------|------|-------|
| LA12    | LA50 | LA100 |
| LA34-VA | LN03 |       |

Different printers have different output quality. For example, dot matrix printers are very different from laser printers. When you design sixel images on the terminal for printing, you should use parameter values that are appropriate for your printer. For more information, see your printer's programmer reference manual.

### Device Control String

The format for the device control string is as follows.

| DCS | P1   | ;    | P2;  | P3;  |  q   | s..s  | ST   |
|-----|------|------|------|------|------|-------|------|
| 9/0 | `**` | 3/11 | `**` | `**` | 7/1  | `***` | 9/12 |

where

- **DCS** is a C1 control character that introduces the sixel data sequence. You can also express DCS as the 7-bit escape sequence ESC P for a 7-bit environment.
- **P1** is the macro parameter. This parameter indicates the pixel aspect ratio used by the application or terminal. The pixel aspect ratio defines the shape of the pixel dots the terminal uses to draw images. For example, a pixel that is twice as high as it is wide has as aspect ratio of 2:1. The following list shows the values you can use for P1.
- ___NOTE: The macro parameter is provided for compatibility with existing Digital software. New applications should set P1 to 0 and use the set raster attributes control, described later in this chapter.___

| P1          | Pixel Aspect Ratio (Vertical:Horizontal) |
|-------------|------------------------------------------|
| **Omitted** | **2:1** (default)                        |
| 0, 1        | 2:1                                      |
| 2           | 5:1                                      |
| 3, 4        | 3:1                                      |
| 5, 6        | 2:1                                      |
| 7, 8, 9     | 1:1                                      |

You can override the setting of the macro parameter by using the set raster attributes character (", 2/2) in a sixel data string. See below.

**;** is a semicolon (3/11). This character separates numeric parameters in a DCS string.

**P2** selects how the terminal draws the background color. You can use one of three values.

| P2                   | Meaning
|----------------------|-------------------------------------------------------------------------|
| **0** or 2 (default) | Pixel positions specified as 0 are set to the current background color. |
| 1                    | Pixel positions specified as 0 remain at their current color.           |

**P3** is the horizontal grid size parameter. The horizontal grid size is the horizontal distance between two pixel dots. The VT300 ignores this parameter because the horizontal grid size is fixed at 0.0195 cm (0.0075 in).

**q** indicates that this device control string is a sixel command.

**s...s** is the sixel-encoded data string. The sixel data characters are characters in the range of ? (hex 3F) to ~ (hex 7E). Each sixel data character represents six vertical pixels of data. Each sixel data character represents a binary value equal to the character code value minus hex 3F.

Examples

- ? (hex 3F) represents the binary value 000000.
- t (hex 74) represents the binary value 110101.
- ~ (hex 7E) represents the binary value 111111.

The terminal translates the six bits to a sixel – six pixels in a vertical column. The least significant bit is at the top.

___NOTE: For information on how to code sixel characters, see "Soft Character Sets" in Volume 1, Chapter 5 of this manual.___

You can also use sixel control functions in the data string. The next section describes these characters and their functions.

**ST** is the string terminator. ST is a C1 control character. You can also express ST as the 7-bit escape sequence ESC \ for a 7-bit environment.

## Sixel Control Functions

You can use sixel control functions to perform special functions, such as selecting colors and raster attributes.

### Graphics Repeat Introducer (!)

The ! (2/1) character introduces a repeat sequence. A repeat sequence lets you repeat a graphic character a specified number of times. You use the following format for the repeat sequence.

| !    | Pn   | character |
|------|------|-----------|
| 2/1  | `**` | `****`    |

where

- **Pn** is the repeat count. The repeat count can be any decimal value. For example, if you use a repeat count of 23, the next character repeats 23 times.

- **character** is the character to repeat. You can use any character in the range of ? (hex 3F) to ~ (hex 7E).


### Raster Attributes (")

The " (2/2) character is the set raster attributes command. This command selects the raster attributes for the sixel data string that follows it. You must use the command before any sixel data string. The " command overrides any raster attributes set by the macro parameter described above. You use the following format for the " command.

| "   | Pan  |  ;   | Pad; | Ph;  | Pv   |
|-----|------|------|------|------|------|
| 2/2 | `**` | 3/11 | `**` | `**` | `**` |

where

**Pan** and **Pad** define the pixel aspect ratio for the following sixel data string. Pan is the numerator, and Pad is the denominator.

```
Pan
--- = pixel aspect ratio
Pad
```

The pixel aspect ratio defines the shape of the pixels the terminal uses to draw the sixel image.

Pan defines the vertical shape of the pixel. Pad defines the horizontal shape of the pixel. For example, to define a pixel that is twice as high as it is wide, you use a value of 2 for Pan and 1 for Pad.

If you use the set raster attributes command (") in a sixel data string, you must specify a pixel aspect ratio. You can only use integer values for Pan and Pad. The VT300 rounds the pixel aspect ratio to the nearest integer.

**Ph** and **Pv** define the horizontal and vertical size of the image (in pixels), respectively.

Ph and Pv do not limit the size of the image defined by the sixel data. However, Ph and Pv let you omit background sixel data from the image definition and still have a color background. They also provide a concise way for the application or terminal to encode the size of an image.

___NOTE: The VT300 uses Ph and Pv to erase the background when P2 is set to 0 or 2.___

### Color Introducer (#)

The # (2/3) color introducer starts a color selection sequence. There are two ways to select colors.

- Select a color map entry by number.
- Use HLS (hue, lightness, and saturation) or RGB (red, green, blue) colors.

#### Basic Colors

You can use the following format to select a basic color map entry.

| #   | Pc   |
|-----|------|
| 2/3 | `**` |

where

- **Pc** is the color number (Table 14-1).

___NOTE: The VT330 has 4 available color map entries, the VT340 has 16.___

### HLS or RGB Colors

You use the following format to specify HLS or RGB colors. HLS and RGB are universally recognized color coordinate systems.

| #    | Pc   |  ;     | Pu;  |  Px; | Py;  | Pz   |
|------|------|--------|------|------|------|------|
| 2/3  | `**` | 3/11   | `**` | `**` | `**` | `**` |

where

- **Pc** is the color number.
- **Pu** is the color coordinate system (HLS or RGB).
- **Px**, **Py**, and **Pz** are the color coordinates in the specified system. Table 14-1 lists the possible values.

Table 14-1 Color Specifier Parameter     Possible Values     Definition

| Parameter     | Possible Values  | Definition                           |
|---------------|------------------|--------------------------------------|
| Pc            | 0 to 255         | The color number to define.          |
| Pu (required) | 1                | HLS (hue, lightness, and saturation) |
|               | 2                | RGB (red, green, and blue)           |

___NOTE: The values of the following parameters depend on the color coordinate system selected (HLS or RGB).___

| HLS Values | Possible Values  | Definition |
|------------|------------------|------------|
| Px         | 0 to 360 degrees | Hue angle  |
| Py         | 0 to 100 percent | Lightness  |
| Pz         | 0 to 100 percent | Saturation |

| RGB Values | Possible Values  | Definition      |
|------------|------------------|-----------------|
| Px         | 0 to 100 percent | Red intensity   |
| Py         | 0 to 100 percent | Green intensity |
| Pz         | 0 to 100 percent | Blue intensity  |

___NOTE: See the "Output Mapping" section in Chapter 2 for a discussion of shade and color programming.___

#### Graphics Carriage Return ($)

The $ (2/4) character indicates the end of the sixel line. The active position returns to the left page border of the same sixel line. You can use this character to overprint lines.

#### Graphics New Line (-)

The - (2/13) character indicates the end of a sixel line. The active position moves to the left margin of the next sixel line.

#### Parameter Separator (;)

The ; (3/11) character separates numeric parameters in a device control string. If there is no number before the separator, the terminal assumes that parameter is 0. If there is a number after the separator, the terminal assumes that parameter is 0.

## Sixel Scrolling Mode

You can set the sixel scrolling mode by using the Sixel Scrolling feature in the Graphics Set-Up screen. You can also select this mode by using the sixel display mode (DECSDM) control function.

#### Sixel Scrolling Enabled

When sixel display mode is enabled, the sixel active position begins at the upper-left corner of the ANSI text active position. Scrolling occurs when the sixel active position reaches the bottom margin of the graphics page. When sixel mode is exited, the text cursor is set to the current sixel cursor position.

The VT300 sends a sixel next line (-) character following a sixel dump. The top line of the sixel image may scroll off the screen if (1) your application returns the sixel dump to the terminal, or (2) you perform a sixel dump to a video terminal connected to the VT300 printer port.

___NOTE: You can prevent the sixel image from scrolling off the screen by disabling the sixel scrolling feature.___

#### Sixel Scrolling Disabled

When sixel scrolling is disabled, the sixel active position begins at the upper-left corner of the active graphics page. The terminal ignores any commands that attempt to advance the active position below the bottom margin of the graphics page. When sixel mode is exited, the text cursor does not change from the position it was in when sixel mode was entered.

#### Sixel Display Mode Control Function

You can set the sixel scrolling mode by using the sixel display mode (DECSDM) control function.

When sixel display mode is set, the Sixel Scrolling feature is enabled. When sixel display mode is reset, the Sixel Scrolling feature is disabled.

To set DECSDM, the control function is.

| CSI  | ?    | 8   | 0   | h   |
|------|------|-----|-----|-----|
| 9/11 | 3/15 | 3/8 | 3/0 | 6/8 |

To reset DECSDM, the control function is.

| CSI  | ?    | 8    | 0     | 1      | 
|------|------|------|-------|--------|
| 9/11 | 3/15 | 3/8  | 3/0   | 6/12   |
