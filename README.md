# Yogurt, a Pharo toolchain installer

Yogurt manages your Pharo VMs and images. Its interface is heavily inspired by
[rbenv](https://github.com/rbenv/rbenv) and [rustup](https://www.rustup.rs/). It's 
a standalone binary without any external dependencies (i.e., it's super easy to install).

Currently, Yogurt only works with 64bits VMS and images (that might change in the 
future).

## Usage

    # List all the installed VMs 
    $ yogurt vm list

    # List all the downloaded images
    $ yogurt images list

    # List all the available VMs
    $ yogurt vm install

    # Install a VM (download it)
    $ yogurt vm install  2017.07.09

    # Install an image
    $ yogurt image install 64-60507

    # Set the VM that will be used when the user types `pharo`
    $ yogurt vm global 64-2017.07.09

## Storage

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

## Development

Yogurt uses a README-driven development. We write the documentation first and 
then implement the internal.

