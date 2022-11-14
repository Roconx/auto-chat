# Auto Chat

## Concept

By using the live client api, the program detects whether you kill or die in-game.
When the program detects either a kill or death, it performs a certain action.
The current actiions that can be performed are:
- Writing a message in /all chat
- Showing mastery
- Starting a surrender vote

## Usage

To use it, you only need to clone the repository and type 'cargo run' in the terminal, the program should detect the game as soon as loading screen ends.
<br>You can edit the "you_killed.txt" and "you_died.txt" with phrases to your liking.
<br>Config.json example:
<br>
```json
{
  "death": "surrender",
  "kill": "mastery"
}

```
<br>The avaliable options are:
- message
- mastery
- surrender

## Note

This is pretty much a port of [Auto-chat](https://github.com/Gabattal/Scripts-LeagueOfLegends/tree/main/Auto-Chat) to rust with a few added funcions such as mastery and surrender.
All feedback is apreciated!
