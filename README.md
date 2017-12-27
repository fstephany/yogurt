# Yogurt, a Pharo toolchain installer

**IN DEVELOPMENT**, I'm learning Rust through this project. Do not expect
anything from it.

## Description

Yogurt streamlines the usage of Pharo by:

- managing your Pharo VMs and images. Its interface is heavily inspired by
[rbenv](https://github.com/rbenv/rbenv) and [rustup](https://www.rustup.rs/).
It's a standalone binary without any external dependencies (i.e., it's
super easy to install).
- providing a canonical way to create Pharo projects. Yogurt provides a
way to structure a project with sensible defaults; users don't need to
spend too much  effort to know how to get started. This is inspired by
[create-react-app](https://github.com/facebookincubator/create-react-app),
a tool that makes it easy to get started with
[React](https://facebook.github.io/react/).

Currently, Yogurt only works with 64bits VMS and images (that might
change in the future).


## Usage

    # List all the installed VMs 
    $ yogurt vm list local

    # List all the available VMs from the server
    $ yogurt vm list remote

    # Install a VM (download it)
    $ yogurt vm install 2017.07.09

    # List all the downloaded images
    $ yogurt image list local

    # Install an image
    $ yogurt image install 64-60507

    # Set the VM that will be used when the user types `pharo`
    $ yogurt vm global 64-2017.07.09
    
    # Will remove all the installed vms/images from your system.
    # Note that Yogurt will ask to confirm.
    $ yogurt vm reset
    $ yogurt image reset
    
    # pass -f to force the deletion without any confirmation
    $ yogurt vm reset -f
    $ yogurt image reset -f
    
    
Get all the available options by just typing `yogurt` without any arguments.

## Development

Yogurt uses a README-driven development. We write the documentation first and 
then implement the internal.

When developing, you'll want to pass the `DEV=1` environment variable to the
binary. Doing so will use the `` `pwd`/dev-storage`` directory instead of the 
regular one (i.e., `$HOME/.yogurt`). It also won't attack the real webserver 
but a mock one serving files located in `` `pwd`/dev-server``.

We advise to use a [simple http-server](https://github.com/richardanaya/http-server)
to serve the `dev-server` directory.


    $ cargo run --bin dev_server
    $ DEV=1 cargo run --bin yogurt vm list remote

## Implementation details

### Storage

Yogurt stores all the VMs and images in the `$HOME/.yogurt/` directory. The structure
on disk is as followed:

    ~/.yogurt/
    |- sources/
    |  |- PharoV60.sources
    |  |- PharoV70.sources
    |- vms/
    |  |- 2017.06.27/
    |  |- 2017.07.09/
    |- images/
    |  |- 60/
    |  |  |- 60506/
    |  |  |  |- pharo.image
    |  |  |  |- pharo.changes
    |  |  |- 60507/
    |  |  |  |- pharo.image
    |  |  |  |- pharo.changes
    |  |- 70/
    |  |  |- ...
    |  |- workspace/
    |  |  |- // TO BE DEFINED
    |- pharovm-version // stores the current global VM to use
    |- pharoimage-version

### Remote Storage

The files listing the available VMs and images are stored on Azure by default. 
Nothing prevents you to store them elsewhere as long as you serve them with HTTP.


    /vms-linux-x86-64/
    |- vm-list.txt
    |- vm-24122017-01.zip
    |- vm-28122017-01.zip
    /images-64/
    |- images-list.txt
    |- pharo-60-01.zip
    /sources/
    |- PharoV60.sources
    |- PharoV70.sources

### Adding VMs and Images

Files come from: 
- http://files.pharo.org/get-files/
- http://files.pharo.org/vm/pharo-spur64/

VMs are stored with version numbers: `5.0-201712211450`.


Prefix for Linux Spur 64: http://files.pharo.org/vm/pharo-spur64/linux


## Future

- Define a config file so users can specify a personal repo and change the
  default local storage directory

### License

MIT License