import sys

path_to_notebook = "~/Notebook"

dirs = [
    "+Assets",
    "+Templates",
    "Canvases",
    "Collections",
    "Essays",
    "Fields",
    "Fleeting Thoughts",
    "Log",
    "People",
    "Planning",
    "Projects",
    "Wiki",
]


def collect(args):
    print(args)

    if not args:
        print("No arguments provided for collect command.")
        return

    if args[0] == "link":
        collect_link(args[1:])


def collect_link(args):
    print("Collecting links with args:", args[1:])

    if not args:
        print("No arguments provided for collect link command.")
        return

    title = args[0]
    url = args[1] if len(args) > 1 else None
    if not url:
        print("No URL provided for link collection.")
        return

    # Clean up the title

    with open(f"{path_to_notebook}/Collections/Links/{title}.md", "w") as f:
        f.write(link(title, url))


def link(title, url):
    return f"""---
id: {title}
title: {title}
url: {url}
read: false
---"""


def main():
    args = sys.argv[1:]

    if not args:
        print("Usage: python main.py <command> [args]")
        return

    if args[0] == "collect":
        collect(args[1:])

    print("Hello from nb!")


if __name__ == "__main__":
    main()
