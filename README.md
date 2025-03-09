# Music-Sorter

Simple utility to sort mp3 files based on metadata tags.

Takes files in an input folder and sorts them into an output folder based on album, artist, and album artist.
- output directory
    - album artist directory (or artist if that's not available)
        - album title directory
            - files

- Sorts individual mp3 files
- Sorts collections of songs contained within zip files (like albums that you would get from Bandcamp). Moves the original zip file to a processed items folder afterwards.
