extern crate ncurses;

use std::str;
use ncurses::*;

#[derive(Copy)]

struct Point {
    x: i32,
    y: i32,
}

struct Block {
    start: Point,
    end: Point, 
    size: Point,
    cursor: Point,
}

impl Block {

    // Initialize a Block. Only need a start and size points,
    // the end Point is created here.
    fn new (start: Point, size: Point) -> Block {
        
        // Declare end & cursor first to avoid moving the value out of start and size
        // as Point does not implement the Copy trait
        Block { end: Point { x: size.x + start.x + 1, y: size.y + start.y + 1 },
                cursor: Point { x: start.x + 1, y: start.y + 1 },
                start: start,
                size: size,
        }
    }

    // Draw the box based on it's given size and location
    fn draw_block(&self) {
        for n in self.start.x+1..self.end.x {
            mvaddch(self.start.y, n, 45);
            mvaddch(self.end.y, n, 45);
        }
    
        for i in self.start.y+1..self.end.y {
            mvaddch(i, self.start.x, 124);
            mvaddch(i, self.end.x, 124);
        }
    }
    
    // Clear the block's area of content
    fn clear_block (&self) {
        for i in self.start.y+1..self.end.y {
            for j in self.start.x+1..self.end.x {
                mvaddch(i, j, 32);
            }
        }
        refresh();
    }

    // Write content into the block at the current cursor position.
    // Returns the starting point of what was printed
    // TODO: Take colour as arg
    fn write_block(&self, content: &str) -> Point {
        let size = content.len();
        let oldx = self.cursor.x;
        let oldy = self.cursor.y;

        mv(self.cursor.x, self.cursor.y);
        
        //Check for size of block and if there is room to print
        printw(content);
        
        let ret = self.cursor;
        self.cursor.x = oldx + size as i32;
        self.cursor.y = oldy + 0;

        ret
    }

    // Update a part of the block without moving the cursor
    // TODO: take colour as an arg
    fn update_block (&self, location: Point, content: &str) {

    }
    
    
}


fn main() {
    initscr();
    cbreak();
    let b = Block::new( Point { x: 5,  y: 5 },
                        Point { x: 75, y: 5 },
                 );
    let e = Block::new( Point { x: 5,  y: 15 },
                        Point { x: 10, y: 1 },
                 );
    b.draw_block();
    e.draw_block();
    
    b.write_block("Hello");
    refresh();
    getch();
    endwin();
}

