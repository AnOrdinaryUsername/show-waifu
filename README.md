# show-waifu

A command-line app using the Safebooru API to show SFW-ish anime fanart in your terminal.

![Using the CLI app to show an anime girl in a terminal](assets/showcase.gif)

## Installation

Download `show-waifu-0.1.0.tar.gz` from releases and extract it.

In the command line

```sh
tar -xf show-waifu-0.1.0.tar.gz
```

Move the show-waifu executable into your bin

```sh
# If ~/bin doesn't exist, create it using the following
mkdir -p ~/bin
mv show-waifu ~/bin
```

Test it to see if it works

```sh
~/bin/show-waifu
```

If ~/bin isn't in your path, add ~/bin to $PATH and reload Bash configuration

```sh
echo 'export PATH=~/bin:$PATH' >> ~/.bash_profile
source ~/.bash_profile
```

Test it again and if it works, it should output an image in your terminal

```sh
show-waifu
```

### Example usage

Read from stdin and show an image

```sh
curl -s https://pbs.twimg.com/media/DoWo3unU4AA2etL\?format\=jpg\&name\=large | show-waifu
```

Search for a specific image based on tags and print details

```sh
show-waifu random --details --tags="ncr_veteran_ranger night~"
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
