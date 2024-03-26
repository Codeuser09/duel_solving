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


class better_cube:
     def __init__(self, side_ring_in, forward_ring_in):
        self.ring_matrix = [side_ring_in, forward_ring_in]
        self.top_value = self.ring_matrix[1][2]

     def roll (self, shift, isForward):
        ring_copy = self.ring_matrix[isForward].copy()
        ring_real = self.ring_matrix[isForward]
        if shift % 4 != 0:
            for i in range(len(ring_real)):
                index_unwrapped = i+shift
                index_wrapped = index_unwrapped % 4 if index_unwrapped > 0 else index_unwrapped % -4
                ring_real[i] = ring_copy[index_wrapped]
        print(ring_real)
        self.top_value = ring_real[2]

class game:
    def __init__(self):
        game_matrix = np.zeros((9, 8))

best_cube = better_cube(side_ring_in=[6, 4, 1, 3], forward_ring_in=[6, 2, 1, 5])
best_cube.roll(shift=1, isForward=0)
print(best_cube.top_value)
