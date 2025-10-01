This is my first time making a large, serious project on Rust and making a .dll mod. 



todo:
- readme
- fix `spell_selector::internal::END_MAGIC_SLOT` being unsafe
- spell_selector logic
- config keybinds to action buffer function
- other gameplay changes
- separate the crate into a workspace consisting of generic (DllMain + Settings, can be reused across games), Elden Ring (Elden Ring specific code), and Elden Ring Modlist (Specific Elden Ring mods that make the gameplay changes)



ty to people on discord for being patient with me and answering my questions

?ServerName?
- axd1x8a
- chainfailure
- _indura

libmem psychiatric hospital
- Rdbo
- the_real_hypno
- alexanderdth (also made the resolve pointer chain function)

The Grand Archives
- inunorii
- tremwil



ty to people on bsky for reading my cursed code

- philpax.me
- aapoalas.trynova.dev
- hjvt.dev



libs from:
- Rdbo (libmem) (nvm currently not using)
- chainfailure (elden ring rust crate + I used the special-effect example as a template)
