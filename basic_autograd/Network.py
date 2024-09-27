import abc
import random
from typing import List

from Tensor import Tensor

class Module():
  @abc.abstractmethod
  def forward(self, X):
    pass

class Relu(Module):
  def forward(self, X: Tensor):
    return X.relu()

class FF(Module):
  def __init__(self, input_size, output_size, name=""):
    self.input_size = input_size
    self.output_size = output_size
    self.weights = Tensor([[2*(random.random() - 0.5) for i in range(input_size)] for j in range(output_size)], requires_grad=True, name = "w_" + name)
    self.bias = Tensor([[random.random()] for i in range(output_size)], requires_grad=True, name = "b_" + name)
    self.relu = Relu()
    self.name = name
    pass

  def forward(self, X: Tensor):
    res = self.weights * X + self.bias
    res = self.relu.forward(res)
    res.name = "a_" + self.name
    return res


class Sequential(Module):
  def __init__(self, layers: List[Module]):
    self.layers = layers
    pass

  def forward(self, X: Tensor):
    for layer in self.layers:
      X = layer.forward(X)
    return X