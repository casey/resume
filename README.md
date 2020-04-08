# resume

A simple resume for the web.

[Rendered](https://rodarmor.com/resume/), with custom font.

To adapt for yourself, fork and modify `index.yaml`.

You'll probably also want to apply some custom styling.

## runtime dependencies

- [`tidy`](http://www.html-tidy.org), used to clean up the formatted HTML.


## dramatiſ personæ

```
resume
├── index.yaml     # input data
├── justfile       # helpful shortcuts
├── netlify.toml   # netlify config
├── rustfmt.toml   # rustfmt config
├── src
│   ├── body.rs
│   ├── common.rs
│   ├── index.rs   # root template context
│   ├── item.rs
│   ├── link.rs
│   ├── main.rs    # main function
│   ├── section.rs
│   ├── style.rs
│   ├── tidy.rs
│   └── url.rs
├── templates
│   ├── index.css  # CSS template
│   └── index.html # HTML template
└── www            # output directory
```
