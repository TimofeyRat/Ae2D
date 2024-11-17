# Ae2D (Aeterno's 2D)
### Ae2D is a game engine which main target is to run on low-end PCs without any problems while giving decent gaming and visual experience.

## Development documentation

- [Building the engine](#building)
- [Initialization](#initializing-the-window)
- [Animations](#writing-animations)
- [Nice texts](#text-formatting)

## Building
Building is successful on Linux and Windows. On both platforrms you need Rust toolchain and SDL2 library with extra SDL2_image and SDL2_ttf package. Simply run `cargo build --release` and Cargo will do everything for you.

## Initializing the window
The main config file is located at `res/global/config.json`. It contains following sections:
- `init` - Variables needed to create window and start the engine;
- `optional` - Variables that are not necessary for the engine, but can be used by it;
- `custom` - Global variables which can be accessed from anywhere in the engine.

### Example:
```json
{
	"init": {
		"title": "Ae2D",
		"style": "fullscreen",
		"size": { "w": 1920, "h": 1080 }
	},
	"optional": {
		"OpenGL": true
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

## Text formatting

Ae2D allows you to style your texts via markdown:
- `^()` - Regular text;
- `^(*)` - **Bold text**;
- `^(/)` - *Italic text*;
- `^(_)` - <ins>Underlined text</ins>;
- `^(-)` - ~~Strikethrough text~~.

Styles also can be combined, i.e. `^(* /)` - ***Both bold and italic***.

The text will contain the style that was written in front of it; In case you want to reset the style, you have to write `^()` before the part that has to be regular:
> This is ^(*)**a bold text, and** $(/)*this is an italic text.*

> Are you ^(* _ /)***<ins>Sure</ins>*** ^()that you want to quit programming? ^(*)**Yes** ^(/)*No*

## Writing animations
### TODO