# html\_gen

html\_gen is a tool to generate static pages, using componets and data written in json files

## Installation
```
> git clone https://github.com/dcxo/html_gen
> cd html_gen
> cargo install --path .
```

## Usage
1. Create a html_gen project
```
> html_gen create [name] # If you don't write a name, html_gen will ask you
```
2. Then write some content in the `index.html` file, add some componets and some data (more on this later), and finally build the project with:
```
> html_gen build
```
3. Your static page will be avaliable on the `dist` folder

## Components and data
[TODO: add documentation]
