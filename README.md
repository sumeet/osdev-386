# osdev for 386
- Started out as wanting to learn operating systems, and more about the computer I used when growing up.
- Started a simpler bootloader in 386 Assembly
- Wanted to go into protected mode (32-bit mode) and get more access to features. And then I found out that MS-DOS actually didn't go into protected mode, it used the original real mode. (And as far as I understand, you were also only able to access 1MB of memory)
- Then decided to work in real mode (16-bit mode that the 386 boots up in), and use the BIOS to make an "OS". Really the first thing I want to do is make an interactive shell / REPL. So I'll be making a programming language.
- Rust makes doing this easier for me.
- It's still not for free, we're going to roll `no_std` and have to roll our own stuff.
