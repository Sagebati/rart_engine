R(Art Engine)
===

Fast art generator using layers of images then fusing them randomly with rarity taken into account.

# Usage
## Installation

Install rust link: https://www.rust-lang.org/tools/install

`cargo build --release`

The binary will be in the `target/` folder

## Usage
create the folder where you want your images ex: `mkdir output`

Then:

with the cargo or the binary* :

`cargo run --release -n 1000`

or 

`./merge -n 1000` *needs the cargo build --release*

- n is the number of wanted images

**Care when you run the binary you want to be on the folder with the layers**

# Config
If rust is installed check the config.toml, and configure your project.

## Layers

```toml
        layers = [
        {folder = "background", prob = 1},
        {folder = "contour", prob = 1},
        {folder = "couleur", prob = 1},
        {folder = "couleur 2", prob = 1},
        {folder = "oreille rose", prob = 0.5},
        {folder = "yeux", prob = 1},
        {folder = "taches", prob = 1},
        {folder = "bouche,nez", prob = 1},
        {folder = "lunettes", prob = 0.5},
        {folder = "chapeau", prob = 0.1},
    ]
```

The layers array describes the folders where we can pick the layers to generate the png. the probability is the probability 
of the final png to have that layer. Ex: `{folder="eyes", prob="0.5"}` the resulting png will only have 50% of having eyes.
Spooky no ?

## Rarity

```toml
rarities = [
    {prefix = "r", factor = 20.0},
    {prefix = "sr", factor = 100.0}
]
```

The rarities config is more abstract, you must put "r" or "sr" suffixing your files names. Ex: `<filename>_sr.png`. Then
adjust the factor. The higher it is the rarest. It can be read like this "sr is 100 times rarer than the basic"
