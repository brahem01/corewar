import struct
import sys

def read_cor(path):
    with open(path, "rb") as f:
        magic = struct.unpack(">I", f.read(4))[0]  # 4-byte big-endian int
        name = f.read(128).decode("utf-8", errors="ignore").rstrip("\x00")
        f.read(4)  # skip null separator
        size = struct.unpack(">I", f.read(4))[0]
        comment = f.read(2048).decode("utf-8", errors="ignore").rstrip("\x00")
        f.read(4)  # skip null separator
        code = f.read(size)

    print("Magic:", hex(magic))
    print("Name:", name)
    print("Size:", size)
    print("Comment:", comment)
    print("Code bytes:", code.hex())

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} path/to/file.cor")
        sys.exit(1)

    read_cor(sys.argv[1])

