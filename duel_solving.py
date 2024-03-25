import numpy as np

class cube:
    def __init__(self):
        self.side_ring = [6, 4, 1, 3]
        self.forward_ring = [6, 2, 1, 5]

        self.top_value = self.side_ring[2]

    def roll_left (self):
        side_ring_copy = self.side_ring.copy()
        for i in range(len(self.side_ring)):
            if i == len(self.side_ring) - 1:
                self.side_ring[i] = side_ring_copy[0]
            else:
                self.side_ring[i] = side_ring_copy[i+1]
        self.top_value = self.side_ring[2]

    def roll_right (self):
        side_ring_copy = self.side_ring.copy()
        for i in range(len(self.side_ring)):
            if i == 0:
                self.side_ring[i] = side_ring_copy[-1]
            else:
                self.side_ring[i] = side_ring_copy[i-1]
        self.top_value = self.side_ring[2]

    def roll_forward (self):
        forward_ring_copy = self.forward_ring.copy()
        for i in range(len(self.forward_ring)):
            if i == 0:
                self.forward_ring[i] = forward_ring_copy[-1]
            else:
                self.forward_ring[i] = forward_ring_copy[i-1]
        self.top_value = self.forward_ring[2]

    def roll_backward (self):
        forward_ring_copy = self.forward_ring.copy()
        for i in range(len(self.forward_ring)):
            if i == len(self.forward_ring) - 1:
                self.forward_ring[i] = forward_ring_copy[0]
            else:
                self.forward_ring[i] = forward_ring_copy[i+1]
        self.top_value = self.forward_ring[2]

class game:
    def __init__(self):
        game_matrix = np.zeros((9, 8))

my_cube = cube()
my_cube.roll_backward()
my_cube.roll_left()
my_cube.roll_forward()
my_cube.roll_right()
print(my_cube.top_value)
