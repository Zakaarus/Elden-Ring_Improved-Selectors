This is my first time making a large, serious project on Rust and making a .dll mod. 



todo:
- readme
- fix `spell_selector::internal::MAGIC_SLOT` being unsafe
- spell_selector logic
- config keybinds to action buffer function
- other selection changes
- separate the crate into a workspace consisting of generic (DllMain + Settings, can be reused across games, in /src), Elden Ring (Elden Ring specific code, in src/implementation), and Elden Ring Modlist (Specific Elden Ring mods that make the gameplay changes, in implementation/modlist)
- scrolling support
- some kind of UI element or bringing up the bottom left menu to help with seeing what spell you're on
- fix some problem with overlapping controls? or multi button controls?
- items and weapons selection system next
- build+verify powershell script
- COMPLETELY REWORK THE INPUT LOGIC! IT DOESN'T CONSIDER FRAME DELTA AND STUFF! AND IT'S GENERALLY JUST WEIRD AND BUGGY!



ty to people on discord for being patient with me and answering my questions

?ServerName?
- axd1x8a
- chainfailure
- _indura
- thechewanater



The Grand Archives
- inunorii
- tremwil



ty to people on bsky for reading my cursed code

- philpax.me
- aapoalas.trynova.dev
- hjvt.dev



libs from:
- chainfailure (elden ring rust crate + I used the special-effect example as a template)





Credits from previous versions:
libs from:
- Rdbo (libmem) (nvm currently not using)
libmem psychiatric hospital
- Rdbo
- the_real_hypno
- alexanderdth (also made the resolve pointer chain function)