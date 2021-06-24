# PolaronProton
This is a simple program to make symlinks in steam compatdata folder.
Primary use is to symlink one prefix to another, in order to save space and use single superprefix for every game.
This way, you also don't need to reinstall every single dependency (.net, fonts, dx) each time you try to use proton.

ArchLinux [AUR page](https://aur.archlinux.org/packages/polaronproton-git/)

##### Roadmap:

 - [x] Basic linking support in library
 - [x] CLI interface (Usable, but WIP)
 - [x] Make backups for link folder if it exists already (preventing link folder to be deleted)
 - [ ] Ability to link games by it's name, not just appid (steam web api)
 - [ ] Ability to make superprefix with different name (instead of u32 -> string names)
 - [ ] Store links configuration somewhere for easy linking (by aliases) later
 - [ ] GUI interface (tray, gtk4)

 This roadmap probably isn't complete and will be expanded later
