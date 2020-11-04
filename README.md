# What is this?

This is supposed to be a program very much like [TF2 Bot Detector](
https://github.com/PazerOP/tf2_bot_detector). 
It's a program that connects to a local Team Fortress 2 remote console(RCON for short) and sends commands and read from the game's console log file information, and present this in a user interface. 
It also handles rule files that allows you to specify that players with certain name or chat text patterns will result in some action. 
The most typical action is to mark a player as a cheater and try to automatically vote-kick that player or at least inform the other team they have a bot.

# Background

I play TF2(say hi to aftershave on EU, mostly Stockholm, servers) since years and the bot problem is at times making the game not fun to play. 
The bots have running amok in TF2 since about 2-3 years now.
Valve is very slow with banning the bot accounts and the in-game tools are primitive and frustrating. 
Actually, except fixing some game crashing bugs, Valve has not done anything from what I know. 
And they are criminally bad at communication. 
Oh, I almost forgot, they made it impossible for Free2Play accounts to not use any chat, voice or otherwise along with some other restrictions.
Not a good way to build a community. Despite this, TF2 is more popular than ever.

I've been programming since young age, and these days when I'm not too tired after work it would be nice to have a fun project and to develop some new skill set.

In my case I'm very interested in learning the [Rust Programming Language](https://www.rust-lang.org/).
It's an interesting low-level-ish language with some concepts not seen very often. 
The tools and community also make it a pleasant and modern experience.
So I thought why not combine my two interests and try to do a rewrite of TF2 Bot Detector.

The hardest and most labor intensive part will be making a satisfying user interface.
I hope I don't end up using some Electron style user interface. There are several user interface libraries for Rust. See https://www.areweguiyet.com/ for an overview. My feeling is that most of them are pretty far from being near the functionality and polish of Gtk, Qt, Cocoa and WinUI. There are of course wrappers for these frameworks for Rust.

# Application architecture

This is a work in progress.
Nothing is decided yet, and I read the source code of TF2 Bot Detector and try to understand how it work, bit by bit along with implementing similar code in this application.

Here's a list of things that I think the application need to do:

- **DONE** Read and write JSON files containing rules. 
- **DONE** Read player info from Steam Web API.
- **DONE** Start TF2 with command line arguments that sets the RCON password and port along with some other arguments.
- **NOT DONE** Read and write JSON files containing player lists.
- **NOT DONE** Monitor the TF2 process.
- **NOT DONE** Open a socket to the TF2 RCON.
- **NOT DONE** Write commands and read responses in the binary format RCON uses.
- **NOT DONE** Monitor the TF2 log file to see the output from some RCON commands.
- **NOT DONE** Parse lines in the TF2 log file. The format feels quite free-form and ad hoc from a first look. Might be a bit tricky.
- **NOT DONE** Truncate the TF2 log file periodically. For efficiency.
- **NOT DONE** The whole user interface. Will write more here when the time comes.

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

## RCON

I'm about to start implementing this and I need to translate Rust structs into byte arrays to send over a socket to the TF2 RCON.
I've found the library [Packed_Struct](https://github.com/hashmismatch/packed_struct.rs).
Looks like once I figure out what TF2 RCON wants it should be a similar experience to using Serde to pack those bytes into a readily transmittable array of bytes.

Here are some resources I plan to read a lot:

- https://developer.valvesoftware.com/wiki/Source_RCON_Protocol

- https://github.com/Subtixx/source-rcon-library

- https://github.com/PazerOP/SourceRCON

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
