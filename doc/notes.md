Floppy Ultra Compression Filesystem
---

- LZ4 Each file before assigning blocks.
- BS: 512bytes


File descriptor:
---

block location 0000101101000000 2 Bytes

file part 0000101101000000 2 Bytes

filename 8 bytes name + 4 Byte extension UTF-8 only. 12 bytes total.(Yes I know. TINY.) Depad the first 8 bytes(maybe...)

(this could be 9 bytes name + 3 Byte extension. but some people have weird programs that use four character extensions. BLEGH)

no directories

File allocation table:

Total file allocation table size:
- 45 Kilobytes
- 1395 Kilobytes of space left on drive.
