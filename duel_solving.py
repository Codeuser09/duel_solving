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

     def get_index(self, value):
        if value > 0:
            return (value %  4)
        else:
            return (value % -4)


     def roll (self, shift, isForward):
        ring_copy = self.ring_matrix.copy()[isForward]
        ring_real = self.ring_matrix[isForward]
        for i in range(len(ring_real)):
            ring_real[i] = ring_copy[self.get_index(i+1)]
        print(ring_real)
        self.top_value = ring_real[2]

'''
     def roll (self, shift, isForward):
#        shift %= 4 if  shift > 0 else -4 #Module operators with negative numbers are weird
        for i in range(shift):
            matrix_copy = self.ring_matrix.copy()
            for i in range(len(self.ring_matrix[isForward])):
                index_to_put = self.get_index(i+1)
                self.ring_matrix[isForward][i] = matrix_copy[isForward][index_to_put]
            self.ring_matrix = matrix_copy
        self.top_value = self.ring_matrix[isForward][2]
'''

class game:
    def __init__(self):
        game_matrix = np.zeros((9, 8))

best_cube = better_cube([6, 4, 1, 3], [6, 2, 1, 5])
best_cube.roll(1, 1)
best_cube.roll(1, 1)
best_cube.roll(1, 1)
best_cube.roll(1, 1)
best_cube.roll(1, 1)
best_cube.roll(1, 1)
print(best_cube.top_value)
