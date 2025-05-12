# 🔠 Rust Letter Finder

A simple command-line Rust program that allows users to input a string and then search for a specific letter within that string. The program outputs the positions (indices) where the letter appears.

---

## 📌 Features

- ✅ Accepts user input for both a string and a single character.
- 🔍 Searches for all occurrences of the given character in the string.
- 🧠 Handles invalid input gracefully (e.g., entering more than one character).
- 📊 Outputs all matching indices if found.

---

## 🖥️ Example

```bash
$ cargo run
Enter a string: 
hello world
Enter a letter to find: 
l
The letter 'l' was found at positions: [2, 3, 9]
