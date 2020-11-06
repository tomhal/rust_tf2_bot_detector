# What is this?

This is supposed to be a program very much like [TF2 Bot Detector](
https://github.com/PazerOP/tf2_bot_detector). 
It's a program that connects to a local Team Fortress 2 remote console(RCON for short) and sends commands and read from the game's console log file information, and present this in a user interface. 
It also handles rule files that allows you to specify that players with certain name or chat text patterns will result in some action. 
The most typical action is to mark a player as a cheater and try to automatically vote-kick that player or at least inform the other team they have a bot.

# Background

I play TF2 since years and the bot problem is at times making the game not fun to play. 
The bots have been running amok in TF2 since about 2-3 years now.
Valve is very slow with banning the bot accounts and the in-game tools are primitive and frustrating. 

I've been programming since young age, and these days when I'm not too tired after work it would be nice to have a fun project to develop some new skill set.

In my case I'm very interested in learning the [Rust Programming Language](https://www.rust-lang.org/).
So I thought why not combine my two interests and try to do a rewrite of TF2 Bot Detector.

# How to build

First you need to have a recent installation of Rust. Follow the instructions at https://www.rust-lang.org/tools/install for how to get Rust onto your system.

Then download the source code using git or a .zip. Open a command prompt and navigate to inside rust_tf2_bot_detector.

Currently only one binary can be build, the RCON Prompt.

## RCON Prompt

To run the RCON prompt type 

    cargo run --bin rconprompt -- --port 40434 --password rconpwd

This will build and start an RCON to `127.0.0.1:40434`. 

Try

    cargo run --bin rconprompt -- --help

for options and usage.

To start TF2 with an RCON at port `40434` you can use this:

    "C:\Program Files (x86)\Steam\steamapps\common\Team Fortress 2\hl2.exe" -steam -game tf  -usercon -high +developer 1 +alias developer +contimes 0 +alias contimes +ip 0.0.0.0 +alias ip +sv_rcon_whitelist_address 127.0.0.1 +alias sv_rcon_whitelist_address +sv_quota_stringcmdspersecond 1000000 +alias sv_quota_stringcmdspersecond +rcon_password rconpwd +alias rcon_password +hostport 40434 +alias hostport +alias cl_reload_localization_files +net_start +con_timestamp 1 +alias con_timestamp -condebug -conclearlog -novid -nojoy -nosteamcontroller -nohltv -particles 1 -console

# Status 

Here's a list of things that I think the application need to do:

- **DONE** Read and write JSON files containing rules. 
- **DONE** Read player info from Steam Web API.
- **DONE** Start TF2 with command line arguments that sets the RCON password and port along with some other arguments.
- **DONE** Implement the Source RCON protocol.
- **DONE** RCON prompt utility.
- **NOT DONE** Read and write JSON files containing player lists.
- **NOT DONE** Monitor the TF2 process.
- **NOT DONE** Monitor the TF2 log file to see the output from some RCON commands.
- **NOT DONE** Parse lines in the TF2 log file. The format feels quite free-form and ad hoc from a first look. Might be a bit tricky.
- **NOT DONE** Truncate the TF2 log file periodically. For efficiency.
- **NOT DONE** The whole user interface. Will write more here when the time comes.

# Application architecture

## Platform choice

No platform-specific code is used so far, this should build and work on the platforms Rust support out of the box. I'll try to keep it cross-platform.

## Async vs threads

Async in Rust requires a run-time to make it work.
There are several run-times for this.
Often but not always you can compile the libraries to use the async run-time you use.

I don't think async is something that would help the architecture of this application, so I'll settle for a couple a well-placed threads and probably use message queues for internal communication.

# Library considerations and choices

## Rules and player list files

I used [Serde](https://serde.rs/) and [Serde-JSON](https://github.com/serde-rs/json) and all it took was a few attributes/annotations on structs and fields to make it possible to serialize and deserialize the data. Very impressed with Serde.

Here are the JSON schemas for the rules and player lists that TF2 Bot Detector uses:
- https://github.com/PazerOP/tf2_bot_detector/tree/master/schemas/v3

## TF2 process monitor

So far Rust's standard library seems to have everything I need to start the TF2 application with a list of arguments, and get a handle back that I can use to wait or kill the TF2 process.

https://doc.rust-lang.org/std/process/struct.Command.html

## User interface

The TF2 Bot Detector uses [Dear ImGui](https://github.com/ocornut/imgui). 
It's popular toolkit for games, it's fast and can probably be made to work inside existing gaming engines render pipeline.
But I don't much like the looks of it and how non-native it feels. 
I think it will need a lot of work to make the UI look not home-made.
I don't feel like spending that time so I'll see if I can find an alternative first.

I'd like to program the user interface using something very similar to [The Elm Architecture](https://guide.elm-lang.org/architecture/).

In short: 
1. An initial state is made the current state.
2. Render current state to get a user interface.
3. An event from the user interface is passed through code that is given the current state and produces an updated state that is made the current state.
4. Go to 2.

There are some frameworks and libraries for Rust that does some close variant of this.

- [iced](https://github.com/hecrj/iced). This looks like a clean and easy to use library.
  - Pro: Cross-platform.
  - Pro: Easy to understand and use.
  - Cons: Downside seems to be the available widgets.

- [relm](https://github.com/iovxw/relm). Have not tried it. Looks clean and easy to use, similar to iced.
  - Pro: Cross-platform.
  - Pro: Uses gtk meaning it will look nice and have plenty of widgets to pick from.
  - Cons: Depends on the Rust nightly version. 
  - Cons: Uses async.
 
- [vgtk](https://github.com/bodil/vgtk). Have not tried this yet. Looks very interesting.
  - Pro: Cross-platform.
  - Pro: Uses gtk meaning it will look nice and have plenty of widgets to pick from.
  - Pro & Cons: Uses some declarative markup language. While it looks slick, I'm not sure how this affects trouble-shooting.
  - Pro: Made by Bodil Stokke. The implementation is likely of high quality and well thought through design choices. I guess support will be easy to get.
  - Cons: Extra library files needed for gtk. Rust produces a single executable file. I'd like to keep it that way.

- [druid](https://github.com/linebender/druid). Have not used, only read example code. Looks pretty neat and I like the comments the author has made on Hacker News.
  - Pro: Cross-platform.
  - Pro: Actively developed.
  - Pro: Has documentation, even a "book".
  - Cons: Not any clear downsides I can see right now. It seems ambitious but without trying it I feel it could need to mature a bit to not break things constantly. Just a feeling.

- [WinRT-RS](https://github.com/microsoft/winrt-rs). This is a Rust interface to the Universal Windows Platform, WinRT, and WinUI. Microsoft and names...
  - Pro: Official support from Microsoft.
  - Pro: Rich set of native, high-quality widgets.
  - Pro: Design the UI using XAML.
  - Cons: Not cross-platform right now. Might be in the future.
  - Cons: Looks complicated to use. Maybe that's because I've not used WinRT and C++. XAML and C# is easy, but this doesn't quite look like it's the same as XAML and C# but with C++ instead.
