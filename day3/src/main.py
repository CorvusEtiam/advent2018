class Rect:
    def __init__(self, x, y, w, h):
        self.x = x 
        self.y = y
        self.w = w 
        self.h = h 

def solve_easy(rectangles):
    canvas = [ [0] * 1000 for i in range(1000) ]
    for r in rectangles:
        for y in range(r.y, r.h):
            for x in range(r.x, r.w):
                canvas[y][x] += 1
    count = 0
    for row in canvas:
        for col in row:
            if col > 1:
                count += 1
    return count 


def solve(rectangles, width = 1000, height = 1000):
    area = 0 
    rects_sorted = sorted(rectanges, key = lambda r: r.x)
    current_pos_x = 0
    while current_pos_x < width or not rect_sorted:
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
                    if r.y <= current_pos_y and r.y + r.h >= current_pos.y:
                        count += 1     
                    elif r.y > current_pos_y:
                        break 
                if count > 1:
                    area += 1
                current_pos_y += 1 
        
        rects_sorted = new_rects_sorted 
        current_pos_x += 1

    
