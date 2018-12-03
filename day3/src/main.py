import re 
NOT_MARKED = 0
VISITED    = 1
OVERLAPPING = 2

class Rect:
    def __init__(self, id, x, y, w, h):
        self.id = id 
        self.x = x 
        self.y = y
        self.w = w 
        self.h = h 
        self.marked = 0

    def __str__(self):
        return f"{self.id} => [({self.x}:{self.y}):({self.w}:{self.h})]"



def solve_easy(rectangles):
    canvas = [[0] * 1000  for i in range(1000)] 
    for r in rectangles:
        print(r)
        for i in range(r.y, r.y + r.h):
            for j in range(r.x, r.x + r.w):
                canvas[i][j] += 1

    count = 0
    fp = open("./dump.log", "w")
    for row in canvas:
        for col in row:
            if col >= 2:
                count += 1
        fp.write("\n")
    fp.close()
    return count 


def solve(rectangles, width = 1000, height = 1000, mark_overlapping = False):
    area = 0 
    rects_sorted = sorted(rectangles, key = lambda r: r.x)
    current_pos_x = 0
    while current_pos_x < width or not rects_sorted:
        rect_in_line, new_rects_sorted = [], []
        for rect in rects_sorted:
            if rect.x < current_pos_x:
                continue 
            elif rect.x >= current_pos_x or rect.x + rect.w < current_pos_x:
                rect_in_line.append(rect)
            elif rect.x + rect.w >= current_pos_x:
                new_rects_sorted.append(rect)
            
            current_pos_y = 0 
            rect_in_line = list(sorted(rect_in_line))
            while current_pos_y < height or not rect_in_line:
                count = 0
                for r in rect_in_line:
                    if r.y <= current_pos_y and r.y + r.h >= current_pos_y:
                        count += 1
                        if mark_overlapping:
                            rect.marker = 2 if count > 1 else 1
                    elif r.y > current_pos_y:
                        break 
                if count > 1:
                    area += 1
                current_pos_y += 1 
        
        rects_sorted = new_rects_sorted 
        current_pos_x += 1



    
def reader(fobj):
    regex = re.compile(r"#(\d+)\W+(\d+),(\d+):\W+(\d+)x(\d+)")
    for line in fobj:
        groups = list(map(int, regex.match(line).groups()))
        yield Rect(*groups)

if __name__ == "__main__":
    with open("/home/misiek/Project/rust/advent2018/inputs/input3.txt", 'r') as fi:
        rects = list(reader(fi))
        print("Solution1: ", solve_easy(rects))

