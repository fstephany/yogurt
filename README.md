# Yogurt, a Pharo toolchain installer

**IN DEVELOPMENT**, I'm learning Rust through this project. Do not expect
anything from this project.

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

### License

MIT License

## Development

Yogurt uses a README-driven development. We write the documentation first and 
then implement the internal.

## Usage

    # List all the installed VMs 
    $ yogurt vm list

    # List all the available VMs
    $ yogurt vm install

    # Install a VM (download it)
    $ yogurt vm install  2017.07.09

    # List all the downloaded images
    $ yogurt images list

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
    |- pharoimage-version


## Future

- Define a config file so users can specify a personal repo and change the
  default local storage directory
