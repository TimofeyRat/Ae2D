# Ae2D (Aeterno's 2D)
### Ae2D is a game engine which main target is to run on low-end PCs without any problems while giving decent gaming and visual experience.

## Development documentation

- [Building the engine](#building)
- [Initialization](#initializing-the-window)

## Building
Building is successful on Linux and Windows. On both platforrms you need Rust toolchain and SDL2 library with extra SDL2_image package. Simply run `cargo build --release` and Cargo will build everything for you.

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