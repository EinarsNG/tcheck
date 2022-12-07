# tcheck
Check image transparency

## How to use:
Pipe image file location to this program and it will echo that file path if it is transparent.

### Example 1 (Simple check for transparency):
* Command: `echo image.png | tcheck`
* Outputs: `image.png` if image is transparent, otherwise doesn't output anything.

### Example 2 (Simple check for opaqueness):
* Command: `echo image.png | tcheck --opaque`
* Outputs: `image.png` if image is opaque, otherwise doesn't output anything.

### Example 3 (Multiple files):
* Command `echo -n "image.png\nimage2.png" | tcheck`
* Outputs all image paths that have transparent images.

PS: If it can't open the image it outputs the error to `stderr`.
