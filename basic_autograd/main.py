import random
from Network import FF, Module
from Tensor import Tensor
import numpy as np

class Network(Module):
  def __init__(self) -> None:
      super().__init__()
      self.layers = [FF(2, 2, name="1"), FF(2, 1, name="2")]
      pass

  def forward(self, X):
    for layer in self.layers:
      X = layer.forward(X)
    return X

def loss_function(y_hat, y) -> Tensor:
  m = ((y_hat - y)  ** 2).sum()
  return m


def random_binary_pair():
  input = Tensor([[random.randint(0, 1)], [random.randint(0, 1)]], name="X")
  value = input.data[0][0] ^ input.data[1][0]
  output = Tensor([[1 if value == 1 else 0]], name="Y_HAT")
  return (input, output)

def train(net, epochs=10, batch_size=1000, lr=0.01):
  loss = 0
  running_loss = 0
  lossTrace = []
  frames = []
  for epoch in range(epochs):
    running_loss = 0
    for _ in range(batch_size):
      (x, y_hat) = random_binary_pair()
      y = net.forward(x)
      loss = loss_function(y, y_hat)
      running_loss += loss.data
      loss.backward()
    lossTrace.append(running_loss / batch_size)
    loss.apply_gradients(lr / batch_size)
    loss.zero_all_grads()
    if(epoch % 10 == 0):
      next_frame = (create_frame(net), running_loss / batch_size, epoch) 
      frames.append(next_frame)
  return lossTrace, net, frames

def test(net):
  n = 0
  total = 1000
  for _ in range(1000):
    (x, y_hat) = random_binary_pair()
    y = net.forward(x)
    if((y.data[0][0] > 0.5) == (y_hat.data[0][0] == 1)): 
      n+=1
    pass
  print(f"{n*100/total:.2f}")

def simple_test():
  x = Tensor([10, -10, 10, -5, 6, 3, 1.0], requires_grad=True)
  y_axis = []
  for i in range(100):
      sum_of_squares = (x * x).sum()  # is a 0-tensor
      sum_of_squares.backward()
      x.apply_gradients(0.1)
      x.zero_all_grads()
      y_axis.append(sum_of_squares.data)
      print(i, sum_of_squares.data)
  return y_axis


def plot_surface(net: Network):
  import matplotlib.pyplot as plt
  import numpy as np

  # Generate data
  X = np.arange(0, 1, 0.05)
  Y = np.arange(0, 1, 0.05)
  X, Y = np.meshgrid(X, Y)
  Z = np.zeros_like(X) 
  Z_HAT = np.zeros_like(X)
  for i in range(len(X)):
    for j in range(len(X[i])):
      Z[i][j] = net.forward(Tensor([[X[i][j]], [Y[i][j]]])).data[0][0]
      Z_HAT[i][j] = 1 if (X[i][j] > 0.5 and  Y[i][j] < 0.5) or (X[i][j] < 0.5 and Y[i][j]) > 0.5 else 0

  # Create a 3D plot
  fig = plt.figure()
  ax = fig.add_subplot(121, projection='3d')
  ax2 = fig.add_subplot(122, projection='3d')

  # Plot the surface
  surf = ax.plot_surface(X, Y, Z, cmap=plt.cm.coolwarm)
  ax2.plot_surface(X, Y, Z_HAT, cmap=plt.cm.coolwarm)

  # Show the plot
  plt.show()


def plot_line(arr):
  import matplotlib.pyplot as plt
  plt.plot(arr)
  plt.show()

def plot_many_lines(arr):
  #arr list of tuples (name, data)
  # plot the lines on the same axis with a legend
  import matplotlib.pyplot as pl
  for name, data in arr:
    pl.plot(data, label=name)
    pl.legend()
  pl.show()

def plot_figure_grid(arr):
  #arr list of tuples (name, data)
  import matplotlib.pyplot as plt
  fig, axs = plt.subplots(len(arr), sharex=True, sharey=True)
  for i in range(len(arr)):
    axs[i].plot(arr[i][1])
    axs[i].set_title(arr[i][0])
  plt.show()  



def create_frame(net: Network):
  # Generate data
  X = np.arange(0, 1, 0.05)
  Y = np.arange(0, 1, 0.05)
  X, Y = np.meshgrid(X, Y)
  Z = np.zeros_like(X) 
  for i in range(len(X)):
    for j in range(len(X[i])):
      Z[i][j] = net.forward(Tensor([[X[i][j]], [Y[i][j]]])).data
  return (X, Y, Z)

def animation(frames):
  # Frames is a list of tuples ((X, Y, Z), loss, epoch)
  import matplotlib.pyplot as plt
  from matplotlib.animation import FuncAnimation

  # create a figure and axis object for two plots (Z and loss) in a 2x1 grid
  fig, (ax1, ax2) = plt.subplots(1, 2)

  loss_y = [sublist[1] for sublist in frames]

  # define the function to update the plot
  def update(frame_number):
    X, Y, Z = frames[frame_number][0]
    epoch = frames[frame_number][2]
    ax1.clear()
    ax1.imshow(Z, cmap=plt.cm.coolwarm)
    ax1.set_title(f"Epoch: {epoch}")
    #Relable the X and Y axis
    ax1.set_xticks(np.arange(0, 20, 5))
    ax1.set_xticklabels(np.arange(0, 1, 0.25))
    ax1.set_yticks(np.arange(0, 20, 5))
    ax1.set_yticklabels(np.arange(0, 1, 0.25))

    ax2.clear()
    # loss = [sublist[1] for sublist in frames[:frame_number]]
    ax2.plot(loss_y)
    ax2.scatter(frame_number, frames[frame_number][1])
    ax2.set_title("Loss")
    ax2.set_xlabel("Epoch")


  # create the animation
  ani = FuncAnimation(fig, update, frames=len(frames), interval=50) 

  # display the animation
  plt.show()


if __name__ == '__main__':
  # Create a array of Z values
  net = Network()
  loss_trace, net, frames = train(net, 2000, 50, 0.5)
  animation(frames)
  # traces = []
  # for i in range(1, 5):
  #   net = Network()
  #   loss_trace, net = train(net, 1000, 1, 0.1 ** float(i))
  #   traces.append((f"Lr={0.1 ** i:.2e}", loss_trace))
  # plot_figure_grid(traces)