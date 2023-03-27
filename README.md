# Scrype

Script your typing.

## What is this?

Scrype is a cross-platform program that watches what you type and runs scripts when it detects certain characters have been typed.

Those scripts can then perform actions and return text to type out in response.

For example you could type `shrug`, Scrype would erase that, and the script would type out `¯\_(ツ)_/¯`.

### Prefixes & Suffixes

You can provide global prefixes and suffixes to matches. If you defined `;;` as your suffix, and had a script matching `shrug`, you would need to type out `shrug;;` to trigger it.

This feature exists because in alternatives such as [Espanso](https://github.com/espanso/espanso)--the inspiration for this project--, macros have to define prefixes and suffixes themselves, and often use different ones. Different people also prefer different setups. That problem frequently causes confusion when trying to memorize how to use macros you download.

## Features

| Feature | Status | How To Implement |
| - | :-: | - |
| Deno Macro Runner | ✅ |  |
| Global Prefix & Suffix | ✅ |  |
| Text Pasting | ✅ |  |
| Direct Text Injection | ⏲️ | Write native bindings for each platform. |
| Custom Deno Path | ⏲️ | Set path in main config. |
| Macro Permissions With Deno | ⏲️ | Set Deno args in main config. |
| Backspace To Undo | ⏲️ | Track how many characters are sent and the history match of the last macro. |

