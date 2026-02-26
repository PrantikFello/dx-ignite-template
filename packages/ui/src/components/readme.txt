since the oackage is to be compiled into a single binary,
it is recommended to include_str the css assets.

replace -> Assets = asset!
with -> &str = include_str!
