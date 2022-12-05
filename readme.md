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
- trigger = The single letter or character that will be used to call the bot (ex. ;command !command ?command mcommand) (optional)
- roles = Not currently implemented (optional)

## Commands
; is currently the default bot prompt

- ;console (command to send to mindustry server console)

(parenthesis not needed for console command)


## Todo
- Role Permission for console command
- Better error handling 
