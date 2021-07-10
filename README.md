# show-waifu

A command-line app using the Safebooru + Danbooru API to show anime fanart in your terminal.

![Using the CLI app to show an anime girl in a terminal](assets/showcase.gif)

## Installation

Download `show-waifu-1.0.0.tar.gz` from releases and extract it using
either a GUI or the command line.

If you're using the command line run

```sh
tar -xf show-waifu-1.0.0.tar.gz
```

Move the `show-waifu` executable into your bin

```sh
# If ~/bin doesn't exist, create it using the following
mkdir -p ~/bin
mv show-waifu ~/bin
```

Test it to see if it works, it should output an image in your terminal

```sh
~/bin/show-waifu
```

To make `show-waifu` available everywhere on the command line,
add ~/bin to $PATH and reload the Bash configuration

```sh
echo 'export PATH=~/bin:$PATH' >> ~/.bash_profile
source ~/.bash_profile
```

Test it again and if it works, you're all set to use it!

```sh
show-waifu
```

## Safebooru vs Danbooru

<table>
  <tbody>
    <tr>
      <th></th>
      <th align="center">Safebooru</th>
      <th align="center">Danbooru</th>
    </tr>
    <tr>
      <th>Safe for work</th>
      <td align="center">Fairly safe</td>
      <td align="center">Not really safe</td>
    </tr>
    <tr>
      <th>Searching</th>
      <td align="left">
      	<ul>
          <li>Basic amount of options</li>
          <li>Has no tag limit</li>
        </ul>
      </td>
      <td align="left">
      	<ul>
          <li>Large variety of options</li>
          <li>Has a tag limit depending on level</li>
        </ul>
      </td>
    </tr>
    <tr>
      <th>Image details</th>
      <td align="left">
      	<ul>
          <li>Link</li>
          <li>Rating</li>
          <li>Dimensions</li>
          <li>Tags</li>
        </ul>
      </td>
      <td align="left">
      	<ul>
          <li>Character</li>
          <li>Source</li>
          <li>Artist</li>
          <li>Link</li>
          <li>Rating</li>
          <li>Dimensions</li>
          <li>Tags</li>
        </ul>
      </td>
    </tr>
  </tbody>
</table>

More details can be found below

Searching: [Safebooru's cheatsheet](https://safebooru.org/index.php?page=help&topic=cheatsheet) + [Danbooru's cheatsheet](https://danbooru.donmai.us/wiki_pages/help:cheatsheet)

Image ratings: [Safebooru's image ratings](https://safebooru.org/index.php?page=help&topic=rating) + [Danbooru's image ratings](https://danbooru.donmai.us/wiki_pages/howto:rate) 

### Danbooru Authentication

**For Anonymous and Member-level users, authentication provides no benefits.**

Depending on your account level, authenticating comes with some benefits: an 
increased tag limit and a longer search timemout length. You can authenticate
by doing the following:

 - Go to your [user profile](https://danbooru.donmai.us/profile) and generate an
   API key by clicking the "Generate API key" button

 - Create 2 environmental variables `DANBOORU_USERNAME` and `DANBOORU_API_KEY`
   and add it to ~/.bashrc

    ```sh
    echo 'export DANBOORU_USERNAME="your-name-here"' >> ~/.bashrc
    echo 'export DANBOORU_API_KEY="api-key-here"' >> ~/.bashrc
    ```

 - Add this snippet to ~/.bash_profile

   ```sh
   echo "if [ -f ~/.bashrc ]; then
	. ~/.bashrc
    fi" >> ~/.bash_profile
   ```

 - Check your environmental variables and search for more than 2 tags to verify
   it works

   ```sh
   # Check environmental variables. Both environmental variables should pop-up
   printenv | grep -E '(DANBOORU_USERNAME|DANBOORU_API_KEY)' 
   # If configured properly, you should be allowed to search more than 2 tags
   show-waifu dan --safe --tags="when_the_imposter_is_sus_(meme) jerma985 cat_boy cat_paws cat_ears chartags:1"
   ```

## Example usage

Read from stdin and show an image

```sh
curl -s https://pbs.twimg.com/media/DoWo3unU4AA2etL\?format\=jpg\&name\=large | show-waifu
```

Search for a specific image on Safebooru based on tags, and print details

```sh
show-waifu safe --details --tags="ncr_veteran_ranger night~"
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
    dan     Look at random images from Danbooru
    file    View an image from your file system
    help    Prints this message or the help of the given subcommand(s)
    safe    Look at random images from Safebooru
    url     View an image from a url
```

#### dan (subcommand)

```
USAGE:
    show-waifu dan [FLAGS] [OPTIONS]

FLAGS:
    -d, --details         Show data related to image (artist, source, character, url, rating,
                          dimensions, tags)
    -e, --explicit        Only display images with explicit sexual content
    -h, --help            Prints help information
    -q, --questionable    Only display images with some nox-explicit nudity or sexual content
    -s, --safe            Only display images lacking sexual content. Includes lingerie, swimsuits,
                          innocent romance, etc. NOTE: this doesn't mean "safe for work."
    -V, --version         Prints version information

OPTIONS:
    -k, --key <key>              Pass your Danbooru API key for authentication. NOTE: This doesn't
                                 set a persistent environmental variable and instead only works for
                                 one session
    -t, --tags <tags>            Search for an image based on Danbooru tags. Pass as a string
                                 separated by spaces or commas. Look at Danbooru's cheatsheet for a
                                 full list of search options
    -u, --username <username>    Pass your Danbooru username for authentication. NOTE: This doesn't
                                 set a persistent environmental variable and instead only works for
                                 one session
```

#### safe (subcommand)

```
USAGE:
    show-waifu safe [FLAGS] [OPTIONS]

FLAGS:
    -d, --details         Show data related to image (url, rating, dimensions, tags)
    -h, --help            Prints help information
    -q, --questionable    Only display images with suggestive content
    -V, --version         Prints version information

OPTIONS:
    -t, --tags <tags>    Search for an image based on Safebooru tags. Pass as a string separated by
                         spaces or commas. Look at Safebooru's cheatsheet for a full list of search
                         options
```