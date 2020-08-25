<h1 align="center">html_gen</h1>

<p align="center"> <a href="#installation">Installation</a> | <a href="#usage">Usage</a> | <a href="#components-and-data">Components and data</a> </p>

---
<p align="center">
  <img src="https://travis-ci.org/dcxo/html_gen.svg?branch=master"/> 
</p>

html\_gen is a tool to generate static pages, using components and data written in json files

## Installation
You will need [cargo](https://github.com/rust-lang/cargo) installed
```zsh
> git clone https://github.com/dcxo/html_gen
> cd html_gen
> cargo install --path .
```

## Usage
1. Create a html_gen project
```zsh
> html_gen create [name] # If you don't write a name, html_gen will ask you
```

2. Then write some content in the `index.html` file, add some componets and some data (more on this later), and finally build the project with:
```zsh
> html_gen build
```

3. Your static page will be avaliable on the `dist` folder

## Components and data
[TODO: add documentation (It will be added when there is a new way to write the components). ](https://github.com/dcxo/html_gen/issues/2)
