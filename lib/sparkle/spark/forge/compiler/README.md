# Sparkle Compiler

The whimsical Zig-based compiler frontend for the Spark language.

Magical Components:
- Wand (Lexer/Parser): Waves through your code to detect magical patterns
- Spellbook (Code Generator): Transforms your incantations into working spells
- Sparkles (Tokens): The magical essence of your code

Enchanted Features:
- Native `**` operator support (replacing `::`)
- Module system with magical imports
- Variables are "potions"
- Imports are "scrolls"
- Functions are "enchantments"

Usage:
zig build
zig build test
./zig-out/bin/sparkle build your_magic.spark

Magical Syntax:
scroll std**math

enchant add(a: int, b: int) -> int {
    return a + b
}

potion result = add(40, 2)
