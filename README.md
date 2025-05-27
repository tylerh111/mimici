# mimici

## roadmap

* [ ] initialize
* [ ] git passthrough integration
* [ ] manifest code
* [ ] tracking (add, rm, mv)
* [ ] backup
* [ ] sync
* [ ] profile
* [ ] templates

### initial structure

```bash
<repodir>/ (default=~/.local/share/mimici)
|-- .git
|-- default
|-- <app>
|-- manifest.json
|-- README.md
```

```json
{
    "profile": "...",
    "prefix": null | "/path/to/dir",
    "special": "home" | "root",
    "files": [
        {
            "app": "...",
            "path": "...",
            "install": "...",
            "permissions": "...",
        },
        ...
    ]
}
```

```bash
# global options
mimici
    -C=~/.local/share/mimici

# repo
mimici init

# plumbing
mimici git

# tracking
mimici add  <file|dir> # add a new config file
    --app=...
    --install=...

# backup and sync
mimici backup  # backup all changes made to tracked files
    --message="..."
mimici diff    # check diff between vault vs live
```

```bash
mimici add ~/.bashrc
mimici add ~/.config/user-dirs.dirs
mimici add ~/.gdbinit --app gdb
mimici add ~/.config/fish/* --app fish
# manifest
# {
#     ...,
#     "files": [
#         { "path": "default/.config/user-dirs.dirs", "install": ".config/user-dirs.dirs"                        },
#         { "path": "default/.bashrc",                "install": ".bashrc"                                       },
#         { "path": "fish/functions/ll.fish",         "install": ".config/fish/functions/ll.fish", "app": "fish" },
#         { "path": "fish/config.fish",               "install": ".config/fish/config.fish",       "app": "fish" },
#         { "path": "gdb/.gdbinit",                   "install": ".gdbinit",                       "app": "fish" },
#     ]
# }
# repo
# |-- default
# |   |-- .config
# |   |   |-- user-dirs.dirs
# |   |-- .bashrc
# |-- fish
# |   |-- functions
# |   |   |-- ll.fish
# |   |-- config.fish
# |-- gdb
# |   |-- .gdbinit
# |-- manifest.json
```


---

## Config structure

```bash
~/.config/mimici/
|-- store -> ~/.local/share/mimici
|-- profile -> ./store/profiles/<profile>

~/.local/share/mimici/
|-- .git
|-- profiles
    |-- default
        |-- home
        |-- root
        |-- manifest.json
    |-- <profile>
        |-- home  # prefix `$HOME`
            |-- default # installed directly in path
                |-- <path/to/config>
            |-- gdb
                |-- gdbinit
                |-- gdbearlyinit
            |-- fish
                |-- config.fish
                |-- functions
                    |-- fisher.fish
                    |-- nvm.fish
            |-- bash
                |-- bashrc
        |-- root  # prefix `/`
        |-- <group>  # custom group
        |-- manifest.json
```

## `manifest.json`

use json: https://docs.rs/json/latest/json/

```json
{
    "name": "<profile>",
    "id": "<uuid>",
    "group": [
        {
            "name": "home",
            "install": "$HOME",
            "paths": [
                {
                    "path": "...",
                    "install": "...",
                    "permissions": "...",
                    ...
                }
            ],
            "paths": {
                "gdb/gdbinit": ".",
                "gdb/gdbearlyinit": ".",
                "fish/config.fish": ".config/fish/config.fish",
                "fish/functions/fisher.fish": ".config/fish/functions/fisher.fish",
                "fish/functions/nvm.fish": ".config/fish/functions/nvm.fish",
                "bash/bashrc": {
                    "install": ".bashrc",
                    "permissions": "755",
                    "obey_umask": true,
                }
            }
        },
        {
            "name": "root",
            "install": "/",
            ...
        },
        {
            "name": "<group>",
            "install": "<path>",
            ...
        }
    ]
}
```

## Templates

use jinja: https://docs.rs/json/latest/json/


## CLI

```bash
# global options
mimici
    --config=~/.config/mimici/mimici.toml
    --repo=~/.local/share/mimici

# repo
mimici init <profile>

# profile
mimici activate <profile>
mimici deactivate <profile>
mimici switch <profile>

# tracking
mimici add  <file|dir> # add a new config file
    --group=...
    --app=...
    --install=...
    --permissions=...
mimici rm   <file|dir> # remove a config file

# backup and sync
mimici backup  # backup all changes made to tracked files
    --message="..."
mimici diff    # check diff between vault vs live
mimici sync    # copy files over
    --force

# plumbing
mimici git ...
```

## Thoughts

* profiles are tracked via branches
