import numpy as np

from typing import List, NamedTuple, Callable, Optional, Union, Tuple

class Link(NamedTuple):
    tensors: List['Tensor']
    grad_fn: Callable[['Tensor', List['Tensor']], Tuple['Tensor']]

Arrayable = Union[float, list, np.ndarray]

def ensure_array(arrayable: Arrayable) -> np.ndarray:
    if isinstance(arrayable, np.ndarray):
        return arrayable
    else:
        return np.array(arrayable)

########
# Grad Functions
########
def sum_backward(grad: 'Tensor', parents: List['Tensor']):
  assert len(parents) == 1, "Sum should only have one parent"
  parent = parents[0]
  return (Tensor(grad.data * np.ones_like(parent.data)),)

def add_backward(grad: 'Tensor', parents: List['Tensor']):
  assert len(parents) == 2, "Add should only have two parents"
  return (Tensor(grad.data * np.ones_like(parents[0].data)), Tensor(grad.data * np.ones_like(parents[1].data)))

def sub_backward(grad: 'Tensor', parents: List['Tensor']):
  assert len(parents) == 2, "Sub should only have two parents"
  return (Tensor(grad.data * np.ones_like(parents[0].data)), Tensor(grad.data * np.ones_like(parents[1].data) * -1))

def mul_backward(grad: 'Tensor', parents: List['Tensor']):
  assert len(parents) == 2, "Mul should only have two parents"
  x = parents[0]
  w = parents[1]
  return (Tensor(grad.data * w.data.transpose()), Tensor(x.data.transpose() * grad.data))

def relu_backward(grad: 'Tensor', parents: List['Tensor']):
  assert len(parents) == 1, "Relu should only have one parent"
  a = parents[0]
  return (Tensor(np.where(a.data > 0, grad.data, 0)),)

def pow_backward(grad: 'Tensor', parents: List['Tensor']):
  assert len(parents) == 2, "Pow should only have two parents"
  base = parents[0]
  exponent = parents[1]
  assert exponent.shape == (), "Exponent must be a scalar"
  return (Tensor(grad.data * exponent.data * base.data ** (exponent.data - 1)),)

# def relu_backward(grad: PrimativeMatrix, parents: list):
#   a = parents[0]
#   return (PrimativeMatrix([[grad[i][j] if a.matrix[i][j] > 0 else 0 for j in range(len(a.matrix[0]))] for i in range(len(a.matrix))]))

########
# Tensor
########

"""
The Tensor class which will track the gradients
"""
class Tensor():
  def __init__(self, data: Arrayable, requires_grad=False, link: Link = None, name=""):
    self.data = ensure_array(data)

    self._requires_grad = requires_grad
    self.grad: Optional['Tensor'] = None
    if self._requires_grad:
        self.zero_grad()
    
    self.link = link
    self.shape = self.data.shape
    self.name = name
  
  def __repr__(self) -> str:
    return f"Tensor({self.data.__repr__()}, requires_grad={self.requires_grad}, name={self.name})"

  def zero_grad(self):
    self.grad = Tensor(np.zeros_like(self.data))

  @property
  def requires_grad(self):
    return self._requires_grad
  
  @requires_grad.setter
  def requires_grad(self, value):
    self._requires_grad = value
    if(self._requires_grad and self.grad == None):
      self.zero_grad()

  def backward(self, grad:'Tensor' = None):
    assert self.requires_grad, "Cannot call backward on a tensor that does not require grad"
    if grad is None:
      if self.shape == ():
        grad = Tensor(1)
      else:
        raise RuntimeError("grad must be specified for non 0-tensors")

    self.grad.data += grad.data
    if (self.link is not None):
      grads = self.link.grad_fn(grad, self.link.tensors)
      t_g = list(zip(self.link.tensors, grads))
      for t, g in t_g:
        if(t.requires_grad):
          t.backward(g)      

  def apply_gradients(self, lr=0.01):
    assert self.requires_grad, "Cannot call apply_gradients on a tensor that does not require grad"
    if(self.link is None):
      self.data -= lr * self.grad.data
    else:
      for t in self.link.tensors:
        if(t.requires_grad):
          t.apply_gradients(lr)
  
  def zero_all_grads(self):
    assert self.requires_grad, "Cannot call zero_all_grads on a tensor that does not require grad"
    self.zero_grad()
    if(self.link is not None):
      for t in self.link.tensors:
        if(t.requires_grad):
          t.zero_all_grads()


  def transpose(self):
    name = "transpose(" + self.name + ")"
    return Tensor(self.data.transpose(), requires_grad=self.requires_grad, link=Link([self], None), name=name)

  def sum(self):
     name = "sum(" + self.name + ")"
     return Tensor(np.sum(self.data), requires_grad=self.requires_grad, link=Link([self], sum_backward), name = name)

  def __add__(self, other):
    assert isinstance(other, Tensor), "Cannot add a tensor to a non-tensor"
    assert self.shape == other.shape, "Cannot add tensors of different shapes"
    name = "(" + self.name + "+" + other.name + ")"
    return Tensor(self.data + other.data, requires_grad=self.requires_grad, link=Link([self, other], add_backward), name=name)
  
  def __sub__(self, other):
    assert isinstance(other, Tensor), "Cannot subtract a tensor to a non-tensor"
    assert self.shape == other.shape, "Cannot subtract tensors of different shapes"
    name = "(" + self.name + "-" + other.name + ")"
    return Tensor(self.data - other.data, requires_grad=self.requires_grad, link=Link([self, other], sub_backward), name=name)
  
  def __mul__(self, other):
    assert isinstance(other, Tensor), "Cannot multiply a tensor to a non-tensor"
    requires_grad = self.requires_grad or other.requires_grad
    name = "(" + self.name + "*" + other.name + ")"
    if(self.shape == () or other.shape == ()):
      return Tensor(self.data * other.data, requires_grad=requires_grad, link=Link([self, other], mul_backward), name = name)
    
    assert self.shape[1] == other.shape[0], "Cannot multiply tensors of incompatible shapes"
    data = self.data @ other.data
    return Tensor(data, requires_grad=requires_grad, link=Link([self, other], mul_backward), name = name)
  
  def __pow__(self, other):
    power = other
    if isinstance(other, (int, float)):
      power = Tensor(other, name=str(power))

    assert isinstance(power, Tensor)
    name = "(" + self.name + "^" + power.name + ")"
    return Tensor(self.data ** power.data, requires_grad=self.requires_grad, link=Link([self, power], pow_backward), name=name)

  def relu(self):
    name = "relu(" + self.name + ")"
    return Tensor(np.maximum(self.data, 0), requires_grad=self.requires_grad, link=Link([self], relu_backward), name=name)
  
  def print_backward(self):
    parents = ""
    if(self.link is not None):
      parents = ", ".join(list(map(lambda t: t.name, self.link.tensors))) + " -> "
      for t in self.link.tensors:
          t.print_backward()
    print(parents + self.name)