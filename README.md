# Intro to fileproof

fileproof generates .svg QR-codes of sha2 checksums for files.

```bash
fileproof [filenames]
```
where `[filenames]` is any whitespace separated list of file names.

## Why?

Mostly, I just wanted to practice rust.

I had a usecase where I needed to create evidence that I had a revision of 
a file without revealing the file itself. 
The easiest way is to write an email or mail something that includes a checksum of the files in question.
As many people in my circle have phones capable of reading QR codes, 
and I don't want to need to trust someone to transcribe a checksum, combining 
them seemed appropriate.

Also, a .svg file can be turned nearly transparent and included in the background of html documents, allowing one to include it discretely in an email.

# Future work

# Dependencies
 - sha2
 - qrcodegen

# Acnowledgements
I used code from 

# License
Currently licensed under Apache2
