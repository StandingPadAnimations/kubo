# Kubo
A dotfile manager that watches files in real time.

## Usage
Create a directory called `.kubo` in `$HOME`, then create a file called `kubo.toml` in `.kubo`. `kubo.toml` lists your dotfiles, and can look something like this:
```toml
hypr =  { source = "/home/USER/.config/hypr"  }
eww  =  { source = "/home/USER/.config/eww"   }
kitty = { source = "/home/USER/.config/kitty" }
```
(Note at the moment, `~` does not work for paths defined in kubo.toml yet, but I plan to fix that)

`source` defines where the dotfiles would be stored normally in the home directory, which is mirrored in `.kubo`.

To add dotfiles easily, you can run `kubo add -n <name> -s <dotfiles>`. Using Hyprland as an example:
```sh
kubo add -n hyprland -s ~/.config/hypr -t hypr
```

Finally, run `kubo daemon` to start the daemon.

### Completions
Kubo can automatically generate completions using `kubo --generate <shell>`, and these completions can be used in your shell configuration.
```bash
# .bashrc
eval "$(kubo --generate bash)"

# .zshrc
eval "$(kubo --generate zsh)"

# Same for other shells...
```

## Why another dotfile manager? Why a daemon of all things?
I wanted a dotfile manager that didn't do sym-links yet also updated by backed up dotfiles in real time. I used Chezmoi before, and while it's nice, it has the following issues:
- Does not update in real time.
- Deleting old configs is a pain.

In addition, I also wanted to declare my dotfiles with a config file. I tried [toml-bombadil](https://github.com/oknozor/toml-bombadil) (sym-linked based, but I figured I would try it anyway) but it ended up deleting my dotfiles due to me thinking it would copy my config files first. I was able to restore those files, but it was a pain and I didn't want to do that again.

So I wrote Kubo, a dotfiles manager without sym links that updates in real time. Kubo's philosophy is the following:
- Directories outside `.kubo` should only be modified after A. telling the user what will be modified and B. with user permission.
- No sym-links; a backup is to be a second copy, not the only copy.
- Declarative configs are amazing, let's do more of those.
- Do only job only: properly manage and back up dotfiles.
- Kubo should only copy to `.kubo`, the user decides what they want to do with `.kubo`.

## Why the name Kubo?
When I tried toml-bombadil (i.e. the manager that deleted my dotfiles), I was also watching the anime "Kubo Won't Let Me Be Invisible". I figured Kubo made a good name (and naming things is hard).

## What are some features planned?
Kubo is "mostly complete", I just got to iron out bugs, make the code cleaner, etc. I do want to add a `packages` option eventually, and have Kubo either list or install packages associated with one's dotfiles. After all, I also want to declare what packages use what set of dotfiles.

In addition, I want to add some subcommands to make adding dotfiles easier, like `kubo add` and `kubo rm`.

## Are there other dotfile managers?
[The Arch Wiki](https://wiki.archlinux.org/title/Dotfiles#Tools) has a pretty large list of them, I suggest looking there.

## Ew, Rust (Actual comment I've gotten)
You're not forced to use this, I created this for personal use and wanted to share it.

## Contributing
Check out `CONTRIBUTING.md` in the GitHub repo for more information!
