# RPM Spec Templates

This directory contains [Handlebars] templates of files we use when building
RPMs:

* `spec.hbs`: An [RPM Spec File] generated using metadata from `Cargo.toml`
* `service.hbs`: a [systemd service unit configuration] file (optional)

[Handlebars]: https://github.com/sunng87/handlebars-rust
[RPM Spec File]: https://rpm-guide.readthedocs.io/en/latest/rpm-guide.html#what-is-a-spec-file
[systemd service unit configuration]: https://fedoramagazine.org/systemd-getting-a-grip-on-units/
