# 28XX

28XX represents bytes as [Unicode Braille characters](https://www.unicode.org/charts/PDF/U2800.pdf). In order to achieve this a `28` is added infront of the hexadecimal representation of a byte. The UTF-16 interpretation of the 2-byte combination is then a braille symbol.

It supports byte to braille, as well as braille to byte translation.
It can be used to visualize binary or to reduce character count when transmitting bytes in a text-only way.

||X`0`|X`1`|X`2`|X`3`|X`4`|X`5`|X`6`|X`7`|X`8`|X`9`|X`A`|X`B`|X`C`|X`D`|X`E`|X`F`|
|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
|**`0`X**|`⠀`|`⠁`|`⠂`|`⠃`|`⠄`|`⠅`|`⠆`|`⠇`|`⠈`|`⠉`|`⠊`|`⠋`|`⠌`|`⠍`|`⠎`|`⠏`|
|**`1`X**|`⠐`|`⠑`|`⠒`|`⠓`|`⠔`|`⠕`|`⠖`|`⠗`|`⠘`|`⠙`|`⠚`|`⠛`|`⠜`|`⠝`|`⠞`|`⠟`|
|**`2`X**|`⠠`|`⠡`|`⠢`|`⠣`|`⠤`|`⠥`|`⠦`|`⠧`|`⠨`|`⠩`|`⠪`|`⠫`|`⠬`|`⠭`|`⠮`|`⠯`|
|**`3`X**|`⠰`|`⠱`|`⠲`|`⠳`|`⠴`|`⠵`|`⠶`|`⠷`|`⠸`|`⠹`|`⠺`|`⠻`|`⠼`|`⠽`|`⠾`|`⠿`|
|**`4`X**|`⡀`|`⡁`|`⡂`|`⡃`|`⡄`|`⡅`|`⡆`|`⡇`|`⡈`|`⡉`|`⡊`|`⡋`|`⡌`|`⡍`|`⡎`|`⡏`|
|**`5`X**|`⡐`|`⡑`|`⡒`|`⡓`|`⡔`|`⡕`|`⡖`|`⡗`|`⡘`|`⡙`|`⡚`|`⡛`|`⡜`|`⡝`|`⡞`|`⡟`|
|**`6`X**|`⡠`|`⡡`|`⡢`|`⡣`|`⡤`|`⡥`|`⡦`|`⡧`|`⡨`|`⡩`|`⡪`|`⡫`|`⡬`|`⡭`|`⡮`|`⡯`|
|**`7`X**|`⡰`|`⡱`|`⡲`|`⡳`|`⡴`|`⡵`|`⡶`|`⡷`|`⡸`|`⡹`|`⡺`|`⡻`|`⡼`|`⡽`|`⡾`|`⡿`|
|**`8`X**|`⢀`|`⢁`|`⢂`|`⢃`|`⢄`|`⢅`|`⢆`|`⢇`|`⢈`|`⢉`|`⢊`|`⢋`|`⢌`|`⢍`|`⢎`|`⢏`|
|**`9`X**|`⢐`|`⢑`|`⢒`|`⢓`|`⢔`|`⢕`|`⢖`|`⢗`|`⢘`|`⢙`|`⢚`|`⢛`|`⢜`|`⢝`|`⢞`|`⢟`|
|**`A`X**|`⢠`|`⢡`|`⢢`|`⢣`|`⢤`|`⢥`|`⢦`|`⢧`|`⢨`|`⢩`|`⢪`|`⢫`|`⢬`|`⢭`|`⢮`|`⢯`|
|**`B`X**|`⢰`|`⢱`|`⢲`|`⢳`|`⢴`|`⢵`|`⢶`|`⢷`|`⢸`|`⢹`|`⢺`|`⢻`|`⢼`|`⢽`|`⢾`|`⢿`|
|**`C`X**|`⣀`|`⣁`|`⣂`|`⣃`|`⣄`|`⣅`|`⣆`|`⣇`|`⣈`|`⣉`|`⣊`|`⣋`|`⣌`|`⣍`|`⣎`|`⣏`|
|**`D`X**|`⣐`|`⣑`|`⣒`|`⣓`|`⣔`|`⣕`|`⣖`|`⣗`|`⣘`|`⣙`|`⣚`|`⣛`|`⣜`|`⣝`|`⣞`|`⣟`|
|**`E`X**|`⣠`|`⣡`|`⣢`|`⣣`|`⣤`|`⣥`|`⣦`|`⣧`|`⣨`|`⣩`|`⣪`|`⣫`|`⣬`|`⣭`|`⣮`|`⣯`|
|**`F`X**|`⣰`|`⣱`|`⣲`|`⣳`|`⣴`|`⣵`|`⣶`|`⣷`|`⣸`|`⣹`|`⣺`|`⣻`|`⣼`|`⣽`|`⣾`|`⣿`|

## About Braille

The bottom 2 dots represent the positions 7 and 8 since historically braille only had 6 positions and the bottom line was added at a later time. Because of this the top 3 dots of the left side together with the top dot of the right side represent the first 4 bits while the other dots represent the last 2 bits.
|   |   |
|---|---|
| **1** | **4** |
| **2** | *5* |
| **3** | *6* |
| *7* | *8* |

Examples:

|Braille | Unicode / UTF-16 | <--> | Represented Byte / Hex | Binary |
|---|---|---|---|---|
|`⠏` | `(U+)280F`| <--> | `0F` | `00001111` |
|`⣰` | `(U+)28F0`| <--> | `F0` | `11110000` |
|`⣿` | `(U+)28FF`| <--> | `FF` | `11111111` |
|`⠀` | `(U+)2800`| <--> | `00` | `00000000` |
|`⡩` | `(U+)2869`| <-->　| `69` | `01101001` |

# TODO

- ~~braille to text/byte translation~~ (done)
- ~~byte/text to braille translation~~ (done)
- take cli input
- files -> braille
- text input -> braille
- braille -> files
- braille -> text / hexadecimal
