/*
            ##
       #    ##   ##
       ##   ##   ##
       ##   ##   ##
       ###  ##  ###        _  ___   _____    _ _ _
         ########         | |/ (_) | ____|__| (_) |_ ___  _ __
            ##            | ' /| | |  _| / _` | | __/ _ \| '__|
          ######          | . \| | | |__| (_| | | || (_) | |
        ### ## ###        |_|\_\_| |_____\__,_|_|\__\___/|_|
       ##   ##   ##
      ##    ##    ##      Multi-cursor combinatoric modal editor
      ##    ##    ##
      ##    ##    ##
      #     ##    ##
            ##
            ##
*/

const INTRO: &'static str = "

Welcome to this Ki Tutor aimed at absolute beginners that will teach you
the basics of using the editor.

click i to go to the next line.

Ki is a modal editor meaning it has multiple modes. The editor starts in
normal mode by default (indicated by 'NORM' in the status bar) where you
can change selection and execute actions. To insert Text you will need to
go to insert mode (indicated by 'INST' in the status bar) by clicking h (←
Insert) or o (Insert →). h (← Insert) will put the cursor at the beginning
of the selection and o (Insert →) will put it at the end of the selection.
Now you can type normally, click the escape key (Esc) to go back to normal
mode and click Enter to save."

const SPACE_MENU: &'static str = "
                              ╭────────────╮
                              │ Space Menu │
                              ╰────────────╯
Click space to open the main editor menu. Each square in the help menu
represents the position of a key, the bottom text in each square represent
what the key does when you click it, the text in the middle represents
what the key does when you hold shift and click it. The right side
contains shortcuts and actions, for example pick Editor (n) and then Quit
(v).

Space menu reference: https://ki-editor.org/docs/normal-mode/space-menu

You can also click space + / to open the help menu that shows you the main
editor keymap. Core movements are on the right, some quick actions and
selection modes are on the left."

const SELECTION_AND_MOVEMENTS: &'static str = "
                        ╭─────────────────────────╮
                        │ Selection And Movements │
                        ╰─────────────────────────╯
You are currently in the line selection mode (indicated by 'LINE' in the
status bar). Click n (<<) to select previous line and i (>>) for the next
line. Click r to switch to word selection mode. Click n (<<) to select
previous word and i (>>) for the next word. Notice how the same keys (n i)
do different actions depending on the selection mode, these are called
movements, every selection mode uses the same movements (positional
coherence).
These are the primary selection modes:
 q: Char
 w: Subword
 r: Word
 R: Word* (Big Word)
 a: Line
 A: Line* (Full Line)
 s: Syntax Node
 S: Syntax Node*

and these are the core movements:
 ╭─────┬────────┬─────┬───────────┬─────╮
 │  j  │    l   │  u  │     y     │  ;  │
 │ |<  │    <   │  ^  │     >     │  >| │
 ╰─────┼────────┼─────┼───────────┼─────╯
       │    n   │  e  │     i     │
       │   <<   │  V  │     >>    │
       ├────────┼─────┼───────────┤
       │M: index│     │     .     │
       │m: jump │     │parent Line│
       ╰────────╯     ╰───────────╯

These movements are shared between selection modes, and each one follows a
pattern that will help you discover its function:
╭───────────┬────────────────┬─────────────────────────╮
│ Movements │      Name      │         Speed           │
├───────────┼────────────────┼─────────────────────────┤
│   <, >    │ Previous, Next │ Slowest, granular       │
├───────────┼────────────────┼─────────────────────────┤
│  <<, >>   │ Left, Right    │ Moderate, commonly used │
├───────────┼────────────────┼─────────────────────────┤
│   ^, v    │ Up, Down       │ Fastest                 │
├───────────┼────────────────┼─────────────────────────┤
│  |<, >|   │ First, Last    │ -                       │
╰───────────┴────────────────┴─────────────────────────╯

more about selection modes:
https://ki-editor.org/docs/category/selection-modes-1

and more about movements:
https://ki-editor.org/docs/normal-mode/core-movements
https://ki-editor.org/docs/normal-mode/other-movements"

const LINE: &'static str = "
                                 ╭──────╮
                                 │ Line │
                                 ╰──────╯
The only difference between Line and Line* selection modes is that Line*
(Full Line) includes whitespaces at the edges while Line doesn't.

You might have noticed that in line selection mode i (>>) and n (<<) skip
empty lines, You can use l (<) and y (>) if you don't want to skip empty
lines, You can also use u (^) and e (v) to skip to empty lines. You can
click Backspace to go to previous selection, j (|<) to go to the beginning
and ; (>|) to go to the end. To go to a specific line click M (shift+m),
type the line number and click Enter. You can also use m (jump) and type
the first character of the line you want to jump to, if there's only one
line that begins with that character it will get selected, if there are
multiple lines starting in that character, each line will show a different
character at the beginning, click the character that is shown at the
beginning of the line to select it."

const WORD: &'static str = "
                                 ╭──────╮
                                 │ Word │
                                 ╰──────╯
╭───────────┬──────────────────────────────────────────────╮
│ Movements │                   Action                     │
├───────────┼──────────────────────────────────────────────┤
│   <, >    │ Previous or next word                        │
├───────────┼──────────────────────────────────────────────┤
│  <<, >>   │ Previous or next word (non-symbol word only) │
├───────────┼──────────────────────────────────────────────┤
│   ^, v    │ Nearest word in the next or previous line    │
├───────────┼──────────────────────────────────────────────┤
│  |<, >|   │ First or last word                           │
╰───────────┴──────────────────────────────────────────────╯"

const SUBWORD: &'static str = "
                                ╭─────────╮
                                │ Subword │
                                ╰─────────╯
"

const CHAR: &'static str = "
                                 ╭──────╮
                                 │ Char │
                                 ╰──────╯
"

const SYNTAX_NODE: &'static str = "
                              ╭─────────────╮
                              │ Syntax Node │
                              ╰─────────────╯
"
