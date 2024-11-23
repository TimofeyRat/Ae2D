# Ae3D (Aeterno's 2D)
### Ae3D is a game engine which main target is to run on low-end PCs without any problems while giving decent gaming and visual experience.

## Development documentation

- [Building the engine](#building)
- [Initialization](#initialization)
- [Animations](#writing-animations)
- [Nice texts](#nice-texts)

## Building
Building is successful on Linux and Windows. On both platforrms you need Rust toolchain and SDL2 library with extra SDL2_image and SDL2_ttf package. Simply run `cargo build --release` and Cargo will do everything for you.

## Initialization

### Setting up the engine
The main config file is located at `res/global/config.json`. It contains following sections:
- `init` - Variables needed to create window and start the engine;
- `optional` - Variables that are not necessary for the engine, but can be used by it;
- `custom` - Global variables which can be accessed from anywhere in the engine.

### Example:
```json
{
	"init": {
		"title": "Ae3D",
		"style": "fullscreen",
		"size": { "w": 1920, "h": 1080 }
	},
	"optional": {
		"OpenGL": true
	},
	"custom": {
	    "parameter": "value"
	}
}
```

### `init` Entries:
- `title` - Initial name of the window, can be any string value;
- `style` - Style of the window:
	- `default` - Simple window, can't be resized;
	- `resizable` - Simple window, can be resized;
	- `borderless` - Window without decorations;
	- `fullscreen` - fullscreen window.
- `size` - Size of the window. Should contain width and height of the screen in pixels, i.e. `{ "w": 1920, "h": 1080}`. It won't be used with the `fullscreen` style.
### `optional` Entries:
- `OpenGL` - Should window use OpenGL? Can be `true` or `false`;
- `position` - Position of the window. Should contain X and Y coordinates, i.e `{ "x"; 860, "y": 540 }`. If not provided, the window will appear in the center of the screen.

### Color palette
Ae3D lets you to [color](#text-coloring) your texts, but for that you need a palette. It can be defined in `res/global/colors.json` file as follows:

```json
{
	"colorName": {
		"r": 255,
		"g": 255,
		"b": 255,
		"a": 255
	}
}
```

After that you can use these colors via their names.

## Nice texts

### Text formatting

Ae3D allows you to style your texts via markdown:
- `^()` - Regular text;
- `^(*)` - **Bold text**;
- `^(/)` - *Italic text*;
- `^(_)` - <ins>Underlined text</ins>;
- `^(-)` - ~~Strikethrough text~~.

Styles also can be combined, i.e. `^(* /)` - ***Both bold and italic***.

The text will contain the style that was written in front of it; In case you want to reset the style, you have to write `^()` before the part that has to be regular:
> This is ^(*)**a bold text, and** $(/)*this is an italic text.*

> Are you ^(* _ /)***<ins>Sure</ins>*** ^()that you want to quit programming? ^(*)**Yes** ^(/)*No*

### Text coloring

When setting the style for text, you can also set the color for that text part: `^(clr=red)`. For more information about colors look [here](#color-palette). The default text color is white.

## Writing animations
### TODO
