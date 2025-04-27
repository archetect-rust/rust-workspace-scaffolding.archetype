# {{ project-title }}

[Specification](SPECIFICATION.md)

## Crates
{% for crate in crates %}{% if crate != "xtask" %}
- [{{ project-title }}: {{ crate | title_case }}](crates/{{ project-name }}-{{ crate | kebab_case }}/README.md){% endif %}
{%- endfor %}
