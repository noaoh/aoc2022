import math
import functools


class File:
    def __init__(self, name, size):
        self.name = name
        self.size = size

    def as_dict(self):
        return {
            "name": self.name,
            "size": self.size
        }

    def __repr__(self):
        return f"File(name={self.name}, size={self.size})"


class Directory: 
    def __init__(self, name):
        self.name = name
        self.files = {} 
        self.directories = {} 

    @functools.cached_property
    def size(self):
        file_sizes = sum(file.size for file in self.files.values())
        return file_sizes + sum(d.size for d in self.directories.values())

    def as_dict(self):
        files = [file.as_dict() for file in self.files.values()]
        dirs = [dir.as_dict() for dir in self.directories.values()]
        return {
            "name": self.name,
            "files": files,
            "directories": dirs,
            "size": self
        }

def parse_ls_tree(s):
    parent_dir_nodes = [] 
    current_dir_node = Directory("/")
    for line in s.split("\n")[1:]:
        parsed_line = line.split(" ")
        if parsed_line[0] == "$":
            cmd = parsed_line[1]
            if cmd == "cd":
                d = parsed_line[2]
                if d == "..":
                    current_dir_node = parent_dir_nodes.pop(-1)
                    assert(current_dir_node != None) 

                else:
                    tmp = current_dir_node.directories[d]
                    parent_dir_nodes.append(current_dir_node)
                    current_dir_node = tmp
            
        else:
            dir_or_num = parsed_line[0]
            if dir_or_num == "dir":
                dir_name = parsed_line[1]
                current_dir_node.directories[dir_name] = Directory(dir_name)

            else:
                size = int(dir_or_num)
                file_name = parsed_line[1]
                current_dir_node.files[file_name] = File(file_name, size)

    root_node = parent_dir_nodes.pop(0)
    return root_node


def part1(root):
    total = 0
    if root.size <= 100000:
        total += root.size

    for v in root.directories.values():
        total += part1(v)
    
    return total

def __part2(root, l=[]):
    l.append(root.size)
    for x in root.directories.values():
        __part2(x, l=l)
    
    return l

def part2(root):
    yeet = __part2(root)
    sorted_yeet = sorted(yeet)
    current_unused_disk_space = 70000000 - sorted_yeet[-1]
    for x in sorted_yeet:
        if current_unused_disk_space + x > 30000000:
            return x
    
    assert(True == False)


def main():
    with open("in.txt") as f:
        s = f.read()
        root = parse_ls_tree(s)
        print(part1(root))
        print(part2(root))

if __name__ == "__main__":
    main()