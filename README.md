# SVG to ICO Favicon Generator [<img alt="Logo for favicon-rs" src="static/favicon.svg" height="96" align="right"/>](https://favicon.fileformat.info/)

[![deploy](https://github.com/FileFormatInfo/favicon-rs/actions/workflows/gcr-deploy.yaml/badge.svg)](https://github.com/FileFormatInfo/favicon-rs/actions/workflows/gcr-deploy.yaml)
![NodePing status](https://img.shields.io/nodeping/status/q9lw86hq-xgwb-40h3-8pqc-1ezfqcshnttq)

A website to make favicons from SVGs or PNGs/JPEGs.

## Using

Go to [favicon.fileformat.info](https://favicon.fileformat.info/) and upload an `.svg` file!

## Running

If you have rust installed, you should be able to run `./run.sh`.

If you have docker installed, you should be able to run `./docker-run.sh`.

## License

[GNU Affero General Public License v3.0](LICENSE.txt)

## Credits

[![Bootstrap](https://www.vectorlogo.zone/logos/getbootstrap/getbootstrap-ar21.svg)](https://getbootstrap.com/ "HTML/CSS Framework")
[![Git](https://www.vectorlogo.zone/logos/git-scm/git-scm-ar21.svg)](https://git-scm.com/ "Version control")
[![Github](https://www.vectorlogo.zone/logos/github/github-ar21.svg)](https://github.com/ "Code hosting")
[![LibRsvg](https://www.vectorlogo.zone/logos/gnome/gnome-ar21.svg)](https://gitlab.gnome.org/GNOME/librsvg "SVG processing library")
[![Google CloudRun](https://www.vectorlogo.zone/logos/google_cloud_run/google_cloud_run-ar21.svg)](https://cloud.google.com/run/ "Hosting")
[![NodePing](https://www.vectorlogo.zone/logos/nodeping/nodeping-ar21.svg)](https://nodeping.com?rid=201109281250J5K3P "Uptime monitoring")
[![Rust](https://www.vectorlogo.zone/logos/rust-lang/rust-lang-ar21.svg)](https://www.rust-lang.org/?utm_source=vectorlogozone&utm_medium=referrer "Programming language")

## Future 

- [ ] generate from a URL
- [ ] demo mode: pick a random icon
- [ ] CLI that takes a filename or URL
- [ ] bundle all assets into binary: [include_dir](https://crates.io/crates/include_dir)