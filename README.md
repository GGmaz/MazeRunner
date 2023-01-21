# MazeRunner
Find the shortest path to exit the maze. <br />
The maze dimension is 6x9. <br />

The maze architecture is given in amandaMaze.txt file where each line represents one field in the maze.<br />
The first 4 bits represent directions where you can move from that field in the following order: west, east, north, south.<br />
The second 4 bits say if there is a door in the same order as directions.<br />
The first 2 bits from the third 4 bits represent having the key on that field.<br />
The second 2 bits from the third 4 bits say is that the exit of the maze.<br />

Rule:
If you want to pass through the door you need 1 key and after passing that key stays in the door.
