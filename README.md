# mimici

## names

* reposit
* ~~bank~~
* ~~vault~~
* stower
* stowage
* binco
* ~~bindle~~
* portpack (like backpack)
* portmanteau (hard to spell)
* cohold/conhold/confhold/confighold (like stronghold)
* ddot
* **modal**

---

## Config structure

```bash
~/.config/reposit/
|-- store -> ~/.local/share/reposit
|-- profile -> ./store/profiles/<profile>

~/.local/share/reposit/
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

## CLI

```bash
# global options
reposit
    --store=~/.local/reposit

# repo
reposit init <profile>

# profile
reposit activate <profile>
reposit deactivate <profile>

# tracking
reposit add  <file|dir> # add a new config file
    --group=...
    --app=...
    --install=...
    --permissions=...
reposit rm   <file|dir> # remove a config file

# backup and sync
reposit backup  # backup all changes made to tracked files
    --message="..."
reposit diff    # check diff between vault vs live
reposit sync    # copy files over
    --force

# plumbing
reposit git ...
```

## Thoughts

* profiles are tracked via branches
