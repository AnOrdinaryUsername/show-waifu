# show-waifu

A command-line app using the Safebooru API to show SFW-ish anime fanart in your terminal.

### Example usage

Read from stdin and show an image
```sh
curl -s https://pbs.twimg.com/media/DoWo3unU4AA2etL\?format\=jpg\&name\=large | show-waifu
```

Search for a specific image based on tags and print details
```sh
show-waifu --details random --tags="ncr_veteran_ranger night~" 
```

Use a local file and change its height for viewing
```sh
show-waifu --height 10 file ~/Pictures/doge.jpg
```

### Command line options

```
USAGE:
    show-waifu [OPTIONS] [SUBCOMMAND]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --height <height>    Resize the image to a provided height
    -w, --width <width>      Resize the image to a provided width

SUBCOMMANDS:
    file      View an image from your file system
    help      Prints this message or the help of the given subcommand(s)
    random    View a random image from Safebooru
    url       View an image from a url
```

#### random (subcommand)

```
USAGE:
    show-waifu random [FLAGS] [OPTIONS]

FLAGS:
    -d, --details       Show data related to image (url, rating, width, height, tags)
    -h, --help          Prints help information
    -s, --suggestive    Display only suggestive images
    -V, --version       Prints version information

OPTIONS:
    -t, --tags <tags>    Search for an image based on Safebooru tags. Pass as a string separated by
                         spaces or commas. Look at Safebooru's cheatsheet for a full list of search
                         options
```