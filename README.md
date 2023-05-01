# Feature Generator
Generate feature modules automatically
## Installation
From the release page download the executable and put it on your path or at the root of your project
## Usage
```bash
# First, generate local configuration for base-package and app-name
$ feature_generator config --app-name Skeleton --base-package com.cardinalblue
# Generate new feature module
$ feature_generator gen-mod -f movie-details -s movie
# Generate new screen inside existing feature
$ feature_generator gen-screen -f movie-details -s credits
```
```bash
$ feature_generator --help
```
```
Generates new features and screens inside them

Usage: feature_generator [OPTIONS] <COMMAND>

Commands:
  gen-mod     Generates new feature module
  gen-screen  Generates new screen for a feature module
  config      Adds local or global configuration
  help        Print this message or the help of the given subcommand(s)

Options:
  -d, --debug                        Turn debugging on
  -b, --base-package <BASE_PACKAGE>  Optional Base package name
  -a, --app-name <APP_NAME>          Optional Application name
  -h, --help                         Print help
  -V, --version                      Print version
```
```bash
$ feature_generator gen-mod --help
```
```
Generates new feature module

Usage: feature_generator gen-mod --feature <FEATURE> --start-screen <START_SCREEN>

Options:
  -f, --feature <FEATURE>            The name of the new feature
  -s, --start-screen <START_SCREEN>  The name of the starting screen
  -h, --help                         Print help
```

```bash
$ feature_generator gen-screen --help
```
```
Generates new screen for a feature module

Usage: feature_generator gen-screen --feature <FEATURE> --screen <SCREEN>

Options:
  -f, --feature <FEATURE>  The name of the existing feature
  -s, --screen <SCREEN>    The name of the new screen
  -h, --help               Print help
```

```bash
$ feature-generator config --help
```
```
Adds local or global configuration

Usage: feature_generator config [OPTIONS]

Options:
  -g, --global                       Configure globally
  -b, --base-package <BASE_PACKAGE>  Base package name
  -a, --app-name <APP_NAME>          Application name
  -h, --help                         Print help
```



