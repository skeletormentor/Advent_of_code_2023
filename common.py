def read_file(filename: str) -> list[str]:
    with open(filename) as f:
        return f.read().splitlines()