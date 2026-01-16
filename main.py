import os
import curses
import csv

def main(stdscr: curses.window) -> None:
    global bottles
    curses.curs_set(0)  # hide cursor
    stdscr.nodelay(True)  # non-blocking input
    stdscr.clear()
    cursor: dict[str,int] = {"x": 0, "y": 0, "height": 0, "pos": 0}
    current = 1
    c2 = 0
    stack = []


    while True:
        for y in range(rows):
            for x in range(col):
                draw_bottle(stdscr, get_x(x), get_y(y), bottles[y][x])
        x: int = get_x(cursor["x"])
        y: int = get_y(cursor["y"])
        y = y + (height - cursor["height"] - 1) * 2
        stdscr.addstr(y, x, f'{bottles[cursor["y"]][cursor["x"]][cursor["height"]]:2}', curses.A_REVERSE)
        stdscr.addstr(get_y(rows), 0, f'Current val {current}, left = {height - c2}')
        stdscr.refresh()

        key = stdscr.getch()

        if key == ord("k"):
            cursor["height"] += 1
            cursor["height"] %= height
        elif key == ord("j"):
            cursor["height"] -= 1
            cursor["height"] %= height
        elif key == ord("h"):
            cursor["pos"] -= 1
            cursor["pos"] %= fill
            cursor["x"] = cursor["pos"] % col
            cursor["y"] = cursor["pos"] // col
        elif key == ord("l"):
            cursor["pos"] += 1
            cursor["pos"] %= fill
            cursor["x"] = cursor["pos"] % col
            cursor["y"] = cursor["pos"] // col
        elif key == ord("n"):
            val = bottles[cursor["y"]][cursor["x"]][cursor["height"]]
            bottles[cursor["y"]][cursor["x"]][cursor["height"]]= current
            stack.append((cursor["x"], cursor["y"], cursor["height"], val))
            c2 += 1
            if c2 == height:
                c2 = 0
                current += 1
            if current > fill:
                break
        elif key == ord("m"):
            val = stack.pop()
            bottles[val[1]][val[0]][val[2]]= val[3]
            c2 -= 1
            if c2 == -1:
                c2 = height - 1
                current -= 1
        elif key == ord('q'):
            break
def draw_bottle(stdscr: curses.window, x: int, y: int, bottle: list[int]):
    for bot in range(len(bottle)):
        stdscr.addstr(y, x, f'{bottle[-1-bot]:2}')
        y += 2
def get_x(x: int) -> int:
    return x*3
def get_y(y: int) -> int:
    return y*height*3

ch: str = input("same as last?")
folder_name: str = "./data"
if not os.path.exists(folder_name):
    os.mkdir(folder_name)
if ch == "Y" or ch == "y":
    fr = open("data/choices.txt")
    rows: int = int(fr.readline())
    col: int = int(fr.readline())
    emp: int = int(fr.readline())
    height: int = int(fr.readline())
    fr.close()
else:
    rows: int = int(input("enter rows of bottles: "))
    col: int = int(input("enter column of bottles: "))
    emp: int = int(input("enter empty bottles: "))
    height: int = int(input("enter height: "))
    fw = open("data/choices.txt", "w")
    fw.write(f"{rows}\n{col}\n{emp}\n{height}\n{type}")
    fw.close()
bottles: list[list[list[int]]] = []
fill = rows*col - emp
count = 0
for y in range(rows):
    bottles.append([])
    for x in range(col):
        if count < fill:
            bottles[y].append([0 for _ in range(height)])
            count += 1
        else:
            bottles[y].append([])
print(bottles)
curses.wrapper(main)
fw = open("data/games.csv", "w")
cw = csv.writer(fw)
cw.writerow([0]*height)
for y in range(rows):
    for x in range(col):
        cw.writerow(bottles[y][x])
cw.writerow([emp]+[0]*(height-1))
fw.close()

