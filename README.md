# licensor

`licensor` is a simple command line tool to select LICENSE for your sofware.

This tool has two commands,

* `update`. This command download and update licenses from other repository of [GitHub](https://github.com/atareao/licensor-templates)
* `select`. With this command you can select the license.

## Configuration

There is a configuration file in `~/.config/licensor/licensor.yml`.

In this file you find all the licenses with the description, the url of the repository and an array of `variables`. If you set these variables you can personalize every license as you select it. There are some special variables that are set autommatically as `YEAR`, `MONTH` and `DAY`. Some other variables, like `fullname` or `email` must be configured.
