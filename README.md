# Extension-Focused Utilities

Two utilities for applying custom file transformations. `ecp` and `emv` can largely be used in lieu of `cp` and `mv`, but can also perform conversions.

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

The following command will use magick to convert and image.webp -> image.png, and remove image.webp:
`emv image.webp image.png`

And the following command will create png thumbnails for all the videos in the current directory:
`ecp *.mp4 .png`

## Usage
