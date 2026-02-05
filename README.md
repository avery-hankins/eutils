# Extension-Focused Utilities

Two utilities for applying custom file type transformations. `ecp` and `emv` can largely be used in lieu of `cp` and `mv`, respectively. But they can also perform conversions.

With the following config file:

```json
{
    "file_formats": [
        {
            "name": "image",
            "members": [".png", ".jpg", ".jpeg", ".webp"],
            "transformations": [
                ["image", "magick {s} {e}"]
            ]
        },
        {
            "name": "video",
            "members": [".mp4", ".mov", ".avi", ".mkv"],
            "transformations": [
                ["video", "ffmpeg -i {s} {e}"],
                ["image", "ffmpeg -i {s} -vf select=eq(n\\,0) -frames:v 1 -update 1 -q:v 3 {e}"]
            ]
        }
    ]
}
```

The following command will use magick to convert image.webp -> image.png, and remove image.webp. In effect moving it:
`emv image.webp image.png`

And the following command will create png thumbnails for all the videos in the current directory:
`ecp *.mp4 .png`

## Usage

`ecp` and `emv` can be used similarly to `cp` and `mv` in most respects. The following are all valid:
- `emv img.webp dir1`, moves img.webp to dir1, no conversion
- `emv img.webp dir1/img.png`, moves img.webp to dir1/img.png, converting to img.png
- `emv *.webp dir1/.png`, moves all .webp files to dir1, converting to .png

## Configuration

Config file location: `~/.config/eutils/preferences.json` (created automatically on first run)

### Placeholder Values

- `{s}` - source file path
- `{e}` - destination file path (with new extension)

## Dependencies

The utilities themselves have no runtime dependencies, but your transformations may require external tools:
- [ImageMagick](https://imagemagick.org/) (`magick`) for image conversions
- [FFmpeg](https://ffmpeg.org/) (`ffmpeg`) for general media conversion

## Installation

`cargo install eutils`

## TODO

- Replicate major `cp` and `mv` flags

## License

MIT
