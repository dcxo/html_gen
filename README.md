<h1 align="center">html_gen</h1>

<p align="center"> <a href="#installation">Installation</a> | <a href="#usage">Usage</a> | <a href="#components-and-data">Components and data</a> </p>

---
<p align="center">
  <img src="https://travis-ci.org/dcxo/html_gen.svg?branch=master" /> 
  <img src="https://github.com/dcxo/html_gen/workflows/Rust%20Workflow/badge.svg" />
</p>

html\_gen is a tool to generate static pages, using components and data written in json files

## Installation
You will need [cargo](https://github.com/rust-lang/cargo) installed
```zsh
> git clone https://github.com/dcxo/html_gen
> cd html_gen
> cargo install --path .
```
you can simply do:
```zsh
> cargo install html_gen
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

## Components
To create a component, you have to create a html file in the `components` folder. The content of that file can be:
```html
<Component [component's attributes]>
    [component's body]
</Component>
```
like this to use attributes in your component, or directly without the `Component` tag:
```html
[component's body]
```
to not use attributes.

To use it, you just write in your index or other component 
```html
<[component's name] [component's attributes] />
```

## Data
To use data, create a json file in `data` folder, for example, the file `info.json`:
```json
{
    "name": "html_gen",
    "tags": [
        "components", "html"
    ]
}
```
to use it use wrap in double curly brackets `{{...}}` a path to the value you want to use, i.e.:
```html
<!-- html content -->
{{ info.name }}
{{ info.tags.0 }}
{{ info.tags.1 }}
<!-- more html content -->
```
