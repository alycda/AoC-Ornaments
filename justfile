# cheatsheet: https://cheatography.com/linux-china/cheat-sheets/justfile/

get-input year day:
    @curl -sSL -H "Cookie: session=${AOC_SESSION_TOKEN}" https://adventofcode.com/{{year}}/day/{{day}}/input
