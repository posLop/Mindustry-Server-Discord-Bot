## Discrod
https://discord.gg/sRKCKQAdU4

## Usage
Download the executable from releases

Run the bot (usually ./MSDB in a terminal on linux)

This will generate a default config file called config.toml  

Or you can download the template from the git

The available options are:

- discordToken = the discord bot token (required)
- ip = Ip address of your mindustry server.  Usually leave as localhost if running the bot on the same machine as the mindustry server (required)
- port = Port of the socket used by the mindustry server.  It is set to the default port used by the mindustry server but you can check which port is in use by useing the "config socketInputPort" command in the server console (required)
- prefix = The word or character that will be used to call the bot (ex. ;command !command ?command MScommand) (optional)
- roles = list of role ids that are allowed to use the command they are with. invalid role ids are ignored.  if all role ids are invalid then the command can be used by anyone

## Commands
; is currently the default bot prompt

- ;console (command to send to mindustry server console)

- ;git (post git link)

- ;discord (post discord link)

(parenthesis not needed)


## Todo
- Better error handling 
- better help command