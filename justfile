
generate year day:
    echo "{{year}} day{{day}}"
    cargo generate --path ./_template --name day{{day}} --destination {{year}}

generate-next-day year:
    echo "next day: $(python3 next-day.py {{year}})"
    just --justfile {{justfile()}} generate {{year}} `python3 next-day.py {{year}}`
