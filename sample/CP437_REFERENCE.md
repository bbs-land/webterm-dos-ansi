# CP437 Character Reference

This document lists the CP437 (DOS) character codes used in the sample ANSI files.

## Box Drawing Characters (Single Line)

| Hex  | Dec | Char | Description              |
|------|-----|------|--------------------------|
| 0xDA | 218 | ┌    | Top-left corner          |
| 0xC4 | 196 | ─    | Horizontal line          |
| 0xBF | 191 | ┐    | Top-right corner         |
| 0xB3 | 179 | │    | Vertical line            |
| 0xC0 | 192 | └    | Bottom-left corner       |
| 0xD9 | 217 | ┘    | Bottom-right corner      |
| 0xC3 | 195 | ├    | Left T-junction          |
| 0xB4 | 180 | ┤    | Right T-junction         |
| 0xC2 | 194 | ┬    | Top T-junction           |
| 0xC1 | 193 | ┴    | Bottom T-junction        |
| 0xC5 | 197 | ┼    | Cross                    |

## Box Drawing Characters (Double Line)

| Hex  | Dec | Char | Description              |
|------|-----|------|--------------------------|
| 0xC9 | 201 | ╔    | Top-left corner          |
| 0xCD | 205 | ═    | Horizontal line          |
| 0xBB | 187 | ╗    | Top-right corner         |
| 0xBA | 186 | ║    | Vertical line            |
| 0xC8 | 200 | ╚    | Bottom-left corner       |
| 0xBC | 188 | ╝    | Bottom-right corner      |
| 0xCC | 204 | ╠    | Left T-junction          |
| 0xB9 | 185 | ╣    | Right T-junction         |
| 0xCB | 203 | ╦    | Top T-junction           |
| 0xCA | 202 | ╩    | Bottom T-junction        |
| 0xCE | 206 | ╬    | Cross                    |

## Mixed Box Drawing (Single Horizontal, Double Vertical)

| Hex  | Dec | Char | Description              |
|------|-----|------|--------------------------|
| 0xD5 | 213 | ╒    | Top-left corner          |
| 0xB8 | 184 | ╕    | Top-right corner         |
| 0xD4 | 212 | ╘    | Bottom-left corner       |
| 0xBE | 190 | ╛    | Bottom-right corner      |
| 0xC7 | 199 | ╞    | Left T-junction          |
| 0xB6 | 182 | ╡    | Right T-junction         |
| 0xD1 | 209 | ╤    | Top T-junction           |
| 0xCF | 207 | ╧    | Bottom T-junction        |
| 0xD8 | 216 | ╪    | Cross                    |

## Mixed Box Drawing (Double Horizontal, Single Vertical)

| Hex  | Dec | Char | Description              |
|------|-----|------|--------------------------|
| 0xD6 | 214 | ╓    | Top-left corner          |
| 0xB7 | 183 | ╖    | Top-right corner         |
| 0xD3 | 211 | ╙    | Bottom-left corner       |
| 0xBD | 189 | ╜    | Bottom-right corner      |
| 0xC6 | 198 | ╟    | Left T-junction          |
| 0xB5 | 181 | ╡    | Right T-junction         |
| 0xD7 | 215 | ╥    | Top T-junction           |
| 0xCF | 207 | ╧    | Bottom T-junction        |

## Shade/Block Characters

| Hex  | Dec | Char | Description              |
|------|-----|------|--------------------------|
| 0xB0 | 176 | ░    | Light shade (25%)        |
| 0xB1 | 177 | ▒    | Medium shade (50%)       |
| 0xB2 | 178 | ▓    | Dark shade (75%)         |
| 0xDB | 219 | █    | Full block (100%)        |
| 0xDF | 223 | ▀    | Upper half block         |
| 0xDC | 220 | ▄    | Lower half block         |
| 0xDE | 222 | ▐    | Right half block         |
| 0xDD | 221 | ▌    | Left half block          |

## Special Characters

| Hex  | Dec | Char | Description              |
|------|-----|------|--------------------------|
| 0x10 | 16  | ►    | Right-pointing triangle  |
| 0x0F | 15  | ☼    | Sun/asterisk             |
| 0xFB | 251 | √    | Square root / checkmark  |

## ANSI Escape Sequences Used

### Clear Screen
- `ESC[2J` - Clear entire screen
- `ESC[H` - Move cursor to home (1,1)

### Cursor Positioning
- `ESC[{row};{col}H` - Move cursor to position

### Colors (Foreground)
- `ESC[30m` - Black
- `ESC[31m` - Red
- `ESC[32m` - Green
- `ESC[33m` - Yellow
- `ESC[34m` - Blue
- `ESC[35m` - Magenta
- `ESC[36m` - Cyan
- `ESC[37m` - White

### Colors (Background)
- `ESC[40m` - Black background
- `ESC[41-47m` - Other backgrounds (same order as foreground)

### Text Attributes
- `ESC[0m` - Reset all attributes
- `ESC[1m` - Bold/bright
- `ESC[2m` - Dim
- `ESC[4m` - Underline
- `ESC[5m` - Blink
- `ESC[7m` - Reverse video

## Creating CP437 Files

To create proper CP437 ANSI files in bash:

```bash
printf '\xC9\xCD\xCD\xBB\n\xBA  \xBA\n\xC8\xCD\xCD\xBC\n' > box.ans
```

Or use ANSI art editors that support CP437:
- TheDraw (DOS)
- PabloDraw (Windows)
- Moebius (Cross-platform)
