## Discrod
https://discord.gg/FaXKDWQ9

## Usage
Download the executable from releases

Create a file where the bot will be run called .env

Inside of this file set DISCORD_TOKEN= to your bot token 

Run the bot (usually ./MSDB in a terminal on linux)

This will generate a default config file.  

The available options are:

- ip = Ip address of your mindustry server.  Usually leave as localhost if running the bot on the same machine as the mindustry server
- port = Port of the socket used by the mindustry server.  It is set to the default port used by the mindustry server but you can check which port is in use by useing the "config socketInputPort" command in the server console
- trigger = The single letter or character that will be used to call the bot (ex. ;command !command ?command mcommand)
- roles = Not currently implemented

## Commands
; is currently the default bot prompt

- ;console (command to send to mindustry server console)

(parenthesis not needed for console command)


## Todo
- Role Permission for console command
- Remove ansi encoding so mobile users can also clearly see console output
